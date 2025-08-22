//! Default parser implementation for bencode format.
//! Provides functionality to parse bencode-encoded data into Node structures.

use crate::bencode_lib::nodes::node::Node;
use std::collections::HashMap;
use crate::bencode_lib::io::traits::ISource;
use crate::bencode_lib::error::messages::*;

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
        Some(c) => Err(unexpected_character(c)),
        None => Err(ERR_EMPTY_INPUT
            .to_string())
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
                return Err(ERR_INVALID_INTEGER.to_string());
            }
            return number.parse::<i64>()
                .map(Node::Integer)
                .map_err(|_| ERR_INVALID_INTEGER.to_string());
        }
        number.push(c);
        source.next();
    }
    Err(ERR_UNTERMINATED_INTEGER.to_string())
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
        .map_err(|_| ERR_INVALID_STRING_LENGTH.to_string())?;
    let mut string = String::new();
    for _ in 0..len {
        if let Some(c) = source.current() {
            string.push(c);
            source.next();
        } else {
            return Err(ERR_INVALID_STRING_LENGTH.to_string());
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
    Err(ERR_UNTERMINATED_LIST.to_string())
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
        match parse_string(source) {
            Ok(Node::Str(key)) => {
                if key <= last_key {
                    return Err(ERR_DICT_KEYS_ORDER.to_string());
                }
                last_key = key.clone();
                let value = parse(source)?;
                dict.insert(key, value);
            }
            _ => return Err(ERR_DICT_KEY_MUST_BE_STRING.to_string())

        }
    }
    Err(ERR_UNTERMINATED_DICTIONARY.to_string())
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
        assert!(matches!(parse(&mut source), Err(s) if s == ERR_UNTERMINATED_INTEGER));
    }

    #[test]
    fn parse_string_with_error() {
        let mut source = BufferSource::new(b"4:tes");
        assert!(matches!(parse(&mut source), Err(s) if s == ERR_INVALID_STRING_LENGTH));
    }

    #[test]
    fn parse_negative_integer_works() {
        let mut source = BufferSource::new(b"i-32e");
        assert!(matches!(parse(&mut source), Ok(Node::Integer(-32))));
    }

    #[test]
    fn parse_negative_zero_fails() {
        let mut source = BufferSource::new(b"i-0e");
        assert!(matches!(parse(&mut source), Err(s) if s == ERR_INVALID_INTEGER));
    }

    #[test]
    fn parse_list_with_error() {
        let mut source = BufferSource::new(b"li32ei33e");
        assert!(matches!(parse(&mut source), Err(s) if s == ERR_UNTERMINATED_LIST));
    }

    #[test]
    fn parse_dictionary_with_error() {
        let mut source = BufferSource::new(b"d4:testi32e");
        assert!(matches!(parse(&mut source), Err(s) if s == ERR_UNTERMINATED_DICTIONARY));
    }

    #[test]
    fn parse_dictionary_with_unordered_keys_fails() {
        let mut source = BufferSource::new(b"d3:bbci32e3:abci42ee");
        assert!(matches!(parse(&mut source), Err(s) if s == ERR_DICT_KEYS_ORDER));
    }

    #[test]
    fn parse_nested_list_works() {
        let mut source = BufferSource::new(b"lli1ei2eeli3ei4eee");
        match parse(&mut source) {
            Ok(Node::List(list)) => {
                assert_eq!(list.len(), 2);
                assert!(matches!(&list[0], Node::List(l) if l.len() == 2));
                assert!(matches!(&list[1], Node::List(l) if l.len() == 2));
            }
            _ => { assert_eq!(false, true); }
        }
    }

    #[test]
    fn parse_empty_list_works() {
        let mut source = BufferSource::new(b"le");
        match parse(&mut source) {
            Ok(Node::List(list)) => {
                assert_eq!(list.len(), 0);
            }
            _ => { assert_eq!(false, true); }
        }
    }

    #[test]
    fn parse_empty_dictionary_works() {
        let mut source = BufferSource::new(b"de");
        match parse(&mut source) {
            Ok(Node::Dictionary(dict)) => {
                assert_eq!(dict.len(), 0);
            }
            _ => { assert_eq!(false, true); }
        }
    }

    #[test]
    fn parse_dictionary_with_non_string_key_fails() {
        let mut source = BufferSource::new(b"di32ei42ee");
        assert!(matches!(parse(&mut source), Err(s) if s == ERR_DICT_KEY_MUST_BE_STRING));
    }

    #[test]
    fn parse_mixed_list_works() {
        let mut source = BufferSource::new(b"li32e4:testi-42ee");
        match parse(&mut source) {
            Ok(Node::List(list)) => {
                assert_eq!(list.len(), 3);
                assert!(matches!(&list[0], Node::Integer(32)));
                assert!(matches!(&list[1], Node::Str(s) if s == "test"));
                assert!(matches!(&list[2], Node::Integer(-42)));
            }
            _ => { assert_eq!(false, true); }
        }
    }

    #[test]
    fn parse_invalid_integer_format_fails() {
        let mut source = BufferSource::new(b"i++32e");
        assert!(matches!(parse(&mut source), Err(s) if s == ERR_INVALID_INTEGER));
    }
    #[test]
    fn test_complex_dictionary() {
        let mut source = BufferSource::new(b"d3:foo3:bar5:hello5:world4:test5:valuee");
        let result = parse(&mut source);
        assert!(result.is_ok());
        if let Ok(Node::Dictionary(map)) = result {
            assert_eq!(map.len(), 3);
            assert_eq!(map.get("foo").unwrap(), &Node::Str("bar".to_string()));
            assert_eq!(map.get("test").unwrap(), &Node::Str("value".to_string()));
            assert_eq!(map.get("hello").unwrap(), &Node::Str("world".to_string()));
        } else {
            assert_eq!(false, true);
        }
    }

}