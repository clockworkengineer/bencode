//! Default parser implementation for bencode format.
//! Provides functionality to parse bencode-encoded data into Node structures.

use crate::bencode_lib::nodes::node::Node;
use std::collections::HashMap;
use crate::bencode_lib::io::traits::ISource;

/// Parses bencode data from the given source into a Node structure.
/// Handles integers, strings, lists, and dictionaries based on their prefix character.
///
/// # Arguments
/// * `source` - The source containing bencode-encoded data
///
/// # Returns
/// * `Result<Node, String>` - Parsed Node or error message
pub fn parse(source: &mut dyn ISource) -> Result<Node, String> {
    match source.current() {
        Some('i') => parse_integer(source),
        Some('l') => parse_list(source),
        Some('d') => parse_dictionary(source),
        Some('0'..='9') => parse_string(source),
        Some(c) => Err(format!("Unexpected character: {}", c)),
        None => Err("Empty input".to_string())
    }
}

/// Parses an integer value from the source, expecting format 'i<number>e'.
/// Handles both positive and negative integers, rejecting invalid formats like '-0'.
///
/// # Arguments
/// * `source` - The source containing the integer to parse
///
/// # Returns
/// * `Result<Node, String>` - Integer Node or error message
fn parse_integer(source: &mut dyn ISource) -> Result<Node, String> {
    source.next(); // skip 'i'
    let mut number = String::new();
    while let Some(c) = source.current() {
        if c == 'e' {
            source.next();
            if number == "-0" {
                return Err("Invalid integer".to_string());
            }
            return number.parse::<i64>()
                .map(Node::Integer)
                .map_err(|_| "Invalid integer".to_string());
        }
        number.push(c);
        source.next();
    }
    Err("Unterminated integer".to_string())
}

/// Parses a string value from the source, expecting format '<length>:<string>'.
/// Validates the string length and ensures the full string content is available.
///
/// # Arguments
/// * `source` - The source containing the string to parse
///
/// # Returns
/// * `Result<Node, String>` - String Node or error message
fn parse_string(source: &mut dyn ISource) -> Result<Node, String> {
    let mut length = String::new();
    while let Some(c) = source.current() {
        if c == ':' {
            source.next();
            break;
        }
        length.push(c);
        source.next();
    }

    let len = length.parse::<usize>()
        .map_err(|_| "Invalid string length".to_string())?;
    let mut string = String::new();
    for _ in 0..len {
        if let Some(c) = source.current() {
            string.push(c);
            source.next();
        } else {
            return Err("String too short".to_string());
        }
    }
    Ok(Node::Str(string))
}

/// Parses a list from the source, expecting format 'l<elements>e'.
/// Recursively parses all elements until the end marker is found.
///
/// # Arguments
/// * `source` - The source containing the list to parse
///
/// # Returns
/// * `Result<Node, String>` - List Node or error message
fn parse_list(source: &mut dyn ISource) -> Result<Node, String> {
    source.next(); // skip 'l'
    let mut list = Vec::new();
    while let Some(c) = source.current() {
        if c == 'e' {
            source.next();
            return Ok(Node::List(list));
        }
        list.push(parse(source)?);
    }
    Err("Unterminated list".to_string())
}

/// Parses a dictionary from the source, expecting format 'd<key><value>...e'.
/// Ensures keys are strings and are in sorted order.
///
/// # Arguments
/// * `source` - The source containing the dictionary to parse
///
/// # Returns
/// * `Result<Node, String>` - Dictionary Node or error message
fn parse_dictionary(source: &mut dyn ISource) -> Result<Node, String> {
    source.next(); // skip 'd'
    let mut dict = HashMap::new();
    let mut last_key = String::new();
    while let Some(c) = source.current() {
        if c == 'e' {
            source.next();
            return Ok(Node::Dictionary(dict));
        }
        match parse_string(source)? {
            Node::Str(key) => {
                if key <= last_key {
                    return Err("Dictionary keys must be in order".to_string());
                }
                last_key = key.clone();
                let value = parse(source)?;
                dict.insert(key, value);
            }
            _ => return Err("Dictionary key must be string".to_string())
        }
    }
    Err("Unterminated dictionary".to_string())
}

#[cfg(test)]
mod tests {
    use crate::BufferSource;
    use super::*;

    #[test]
    fn parse_integer_works() {
        let mut source = BufferSource::new(b"i32e");
        assert!(matches!(parse(&mut source), Ok(Node::Integer(32))));
    }

    #[test]
    fn parse_string_works() {
        let mut source = BufferSource::new(b"4:test");
        assert!(matches!(parse(&mut source), Ok(Node::Str(s)) if s == "test"));
    }

    #[test]
    fn parse_list_works() {
        let mut source = BufferSource::new(b"li32ei33ee");
        match parse(&mut source) {
            Ok(Node::List(list)) => {
                assert_eq!(list.len(), 2);
                assert!(matches!(&list[0], Node::Integer(32)));
                assert!(matches!(&list[1], Node::Integer(33)));
            }
            _ => { assert_eq!(false, true); }
        }
    }

    #[test]
    fn parse_dictionary_works() {
        let mut source = BufferSource::new(b"d4:testi32ee");
        match parse(&mut source) {
            Ok(Node::Dictionary(dict)) => {
                assert_eq!(dict.len(), 1);
                assert!(matches!(dict.get("test"), Some(Node::Integer(32))));
            }
            _ => { assert_eq!(false, true); }
        }
    }
    #[test]
    fn parse_integer_with_error() {
        let mut source = BufferSource::new(b"i32");
        assert!(matches!(parse(&mut source), Err(s) if s == "Unterminated integer"));
    }

    #[test]
    fn parse_string_with_error() {
        let mut source = BufferSource::new(b"4:tes");
        assert!(matches!(parse(&mut source), Err(s) if s == "String too short"));
    }

    #[test]
    fn parse_negative_integer_works() {
        let mut source = BufferSource::new(b"i-32e");
        assert!(matches!(parse(&mut source), Ok(Node::Integer(-32))));
    }

    #[test]
    fn parse_negative_zero_fails() {
        let mut source = BufferSource::new(b"i-0e");
        assert!(matches!(parse(&mut source), Err(s) if s == "Invalid integer"));
    }

    #[test]
    fn parse_list_with_error() {
        let mut source = BufferSource::new(b"li32ei33e");
        assert!(matches!(parse(&mut source), Err(s) if s == "Unterminated list"));
    }

    #[test]
    fn parse_dictionary_with_error() {
        let mut source = BufferSource::new(b"d4:testi32e");
        assert!(matches!(parse(&mut source), Err(s) if s == "Unterminated dictionary"));
    }

    #[test]
    fn parse_dictionary_with_unordered_keys_fails() {
        let mut source = BufferSource::new(b"d3:bbci32e3:abci42ee");
        assert!(matches!(parse(&mut source), Err(s) if s == "Dictionary keys must be in order"));
    }
}