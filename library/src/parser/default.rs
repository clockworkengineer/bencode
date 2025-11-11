//! Default parser implementation for bencode format.
//! Provides functionality to parse bencode-encoded data into Node structures.

#[cfg(not(feature = "std"))]
use alloc::{
    string::{String, ToString},
    vec,
};

use crate::HashMap;
use crate::Node::Dictionary;
use crate::error::messages::*;
use crate::io::traits::ISource;
use crate::nodes::node::Node;

/// Start marker for bencode integer values ('i')
/// Format: i<digits>e
/// Examples: i42e, i-42e, i0e
const INTEGER_START: char = 'i';
/// End marker for bencode integer values ('e')
/// Terminates an integer value started with INTEGER_START
/// Examples: i42e, i-42e, i0e
const INTEGER_END: char = 'e';
/// Start marker for bencode list values ('l')
/// Format: l<bencoded values>e
/// Examples: le (empty list), li1ei2ee (list of integers)
const LIST_START: char = 'l';
/// End marker for bencode list values ('e')
/// Terminates a list started with LIST_START
/// Examples: le (empty list), li1ei2ee (list of integers)
const LIST_END: char = 'e';
/// Start marker for bencode dictionary values ('d')
/// Format: d<bencoded string><bencoded value>...e
/// Examples: de (empty dict), d3:foo3:bare (single key-value)
const DICT_START: char = 'd';
/// End marker for bencode dictionary values ('e')
/// Terminates a dictionary started with DICT_START
/// Examples: de (empty dict), d3:foo3:bare (single key-value)
const DICT_END: char = 'e';
/// Separator between string length and content (':')
/// Format: <length>:<bytes>
/// Examples: 4:test, 0:, 5:hello
const STRING_SEPARATOR: char = ':';

/// Parses the length prefix of a bencode string, expecting digits followed by ':'.
/// Reads characters until ':' is found and converts them to a numeric length.
///
/// # Arguments
/// * `source` - The source containing the string length to parse
///
/// # Returns
/// * `Result<usize, String>` - Parsed length value or error message
fn parse_string_length(source: &mut dyn ISource) -> Result<usize, String> {
    let mut length = String::new();
    while let Some(c) = source.current() {
        if c == STRING_SEPARATOR {
            source.next();
            break;
        }
        length.push(c);
        source.next();
    }

    length
        .parse::<usize>()
        .map_err(|_| ERR_INVALID_STRING_LENGTH.to_string())
}

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
        Some(INTEGER_START) => parse_integer(source),
        Some(LIST_START) => parse_list(source),
        Some(DICT_START) => parse_dictionary(source),
        Some('0'..='9') => parse_string(source),
        Some(STRING_SEPARATOR) => Err(ERR_INVALID_STRING_LENGTH.to_string()),
        Some(c) => Err(unexpected_character(c)),
        None => Err(ERR_EMPTY_INPUT.to_string()),
    }
}

/// Parses bencode data from a byte slice into a Node structure.
/// This is a convenience function that creates a BufferSource internally.
///
/// # Arguments
/// * `data` - The byte slice containing bencode-encoded data
///
/// # Returns
/// * `Result<Node, String>` - Parsed Node or error message
pub fn parse_bytes(data: &[u8]) -> Result<Node, String> {
    use crate::io::sources::buffer::Buffer;
    let mut source = Buffer::new(data);
    parse(&mut source)
}

/// Parses bencode data from a string into a Node structure.
/// This is a convenience function that creates a BufferSource internally.
///
/// # Arguments
/// * `data` - The string containing bencode-encoded data
///
/// # Returns
/// * `Result<Node, String>` - Parsed Node or error message
pub fn parse_str(data: &str) -> Result<Node, String> {
    parse_bytes(data.as_bytes())
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
        if c == INTEGER_END {
            source.next();
            if number == "-0" {
                return Err(ERR_INVALID_INTEGER.to_string());
            }
            return number
                .parse::<i64>()
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
    let mut string = String::new();
    for _ in 0..parse_string_length(source)? {
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
    let mut list = Node::List(vec![]);
    while let Some(c) = source.current() {
        if c == LIST_END {
            source.next();
            return Ok(list);
        }
        list.add_to_list(parse(source)?)
            .map_err(|e| e.to_string())?;
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
    let mut dict = Dictionary(HashMap::new());
    let mut last_key = String::new();
    while let Some(c) = source.current() {
        if c == DICT_END {
            source.next();
            return Ok(dict);
        }
        match parse_string(source) {
            Ok(Node::Str(key)) => {
                if key <= last_key {
                    return Err(ERR_DICT_KEYS_ORDER.to_string());
                }
                last_key = key.clone();
                let value = parse(source)?;
                dict.add_to_dictionary(&key, value)
                    .map_err(|e| e.to_string())?;
            }
            _ => return Err(ERR_DICT_KEY_MUST_BE_STRING.to_string()),
        }
    }
    Err(ERR_UNTERMINATED_DICTIONARY.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BufferSource;

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
            _ => {
                assert_eq!(false, true);
            }
        }
    }

    #[test]
    fn parse_dictionary_works() {
        let mut source = BufferSource::new(b"d4:testi32ee");
        match parse(&mut source) {
            Ok(Dictionary(dict)) => {
                assert_eq!(dict.len(), 1);
                assert!(matches!(dict.get("test"), Some(Node::Integer(32))));
            }
            _ => {
                assert_eq!(false, true);
            }
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
}
