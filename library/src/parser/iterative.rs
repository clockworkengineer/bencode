//! Iterative (stack-based) parser implementation for bencode format.
//!
//! This parser avoids recursion by using an explicit stack, making it suitable
//! for embedded systems with limited stack space or deeply nested structures.

#[cfg(not(feature = "std"))]
use alloc::{
    string::{String, ToString},
    vec,
    vec::Vec,
};

use crate::HashMap;
use crate::Node::Dictionary;
use crate::error::messages::*;
use crate::io::traits::ISource;
use crate::nodes::node::Node;

/// Parser state machine states
enum ParseState {
    /// Parsing the initial/next value
    ParseValue,
    /// Inside a list, collecting elements
    InList { elements: Vec<Node> },
    /// Inside a dictionary, expecting a key
    InDictKey {
        entries: HashMap<String, Node>,
        last_key: String,
    },
    /// Inside a dictionary, expecting a value for the given key
    InDictValue {
        entries: HashMap<String, Node>,
        key: String,
        last_key: String,
    },
}

/// Start marker for bencode integer values ('i')
const INTEGER_START: char = 'i';
/// End marker for bencode values ('e')
const END_MARKER: char = 'e';
/// Start marker for bencode list values ('l')
const LIST_START: char = 'l';
/// Start marker for bencode dictionary values ('d')
const DICT_START: char = 'd';
/// Separator between string length and content (':')
const STRING_SEPARATOR: char = ':';

/// Parses the length prefix of a bencode string.
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

/// Parses an integer value from the source.
fn parse_integer(source: &mut dyn ISource) -> Result<Node, String> {
    source.next(); // skip 'i'
    let mut number = String::new();
    while let Some(c) = source.current() {
        if c == END_MARKER {
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

/// Parses a string value from the source.
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

/// Iterative parser that uses an explicit stack instead of recursion.
/// This is suitable for embedded systems with limited stack space.
///
/// # Arguments
/// * `source` - The source containing bencode-encoded data
///
/// # Returns
/// * `Result<Node, String>` - Parsed Node or error message
///
/// # Example
/// ```
/// use bencode_lib::{parse_iterative, BufferSource};
///
/// let data = b"d4:name4:John3:agei25ee";
/// let mut source = BufferSource::new(data);
/// let node = parse_iterative(&mut source).unwrap();
/// ```
pub fn parse_iterative(source: &mut dyn ISource) -> Result<Node, String> {
    let mut stack: Vec<ParseState> = vec![ParseState::ParseValue];
    let mut value_stack: Vec<Node> = vec![];

    while let Some(state) = stack.pop() {
        match state {
            ParseState::ParseValue => {
                match source.current() {
                    Some(INTEGER_START) => {
                        value_stack.push(parse_integer(source)?);
                    }
                    Some(LIST_START) => {
                        source.next(); // skip 'l'
                        if source.current() == Some(END_MARKER) {
                            source.next();
                            value_stack.push(Node::List(vec![]));
                        } else {
                            stack.push(ParseState::InList { elements: vec![] });
                            stack.push(ParseState::ParseValue);
                        }
                    }
                    Some(DICT_START) => {
                        source.next(); // skip 'd'
                        if source.current() == Some(END_MARKER) {
                            source.next();
                            value_stack.push(Dictionary(HashMap::new()));
                        } else {
                            stack.push(ParseState::InDictKey {
                                entries: HashMap::new(),
                                last_key: String::new(),
                            });
                            stack.push(ParseState::ParseValue);
                        }
                    }
                    Some('0'..='9') => {
                        value_stack.push(parse_string(source)?);
                    }
                    Some(STRING_SEPARATOR) => {
                        return Err(ERR_INVALID_STRING_LENGTH.to_string());
                    }
                    Some(c) => {
                        return Err(unexpected_character(c));
                    }
                    None => {
                        return Err(ERR_EMPTY_INPUT.to_string());
                    }
                }
            }

            ParseState::InList { mut elements } => {
                // Pop the parsed value from value_stack
                if let Some(value) = value_stack.pop() {
                    elements.push(value);
                }

                // Check if list is complete
                match source.current() {
                    Some(END_MARKER) => {
                        source.next();
                        value_stack.push(Node::List(elements));
                    }
                    Some(_) => {
                        // Continue parsing next element
                        stack.push(ParseState::InList { elements });
                        stack.push(ParseState::ParseValue);
                    }
                    None => {
                        return Err(ERR_UNTERMINATED_LIST.to_string());
                    }
                }
            }

            ParseState::InDictKey { entries, last_key } => {
                // Pop the parsed key from value_stack
                if let Some(Node::Str(key)) = value_stack.pop() {
                    if key <= last_key {
                        return Err(ERR_DICT_KEYS_ORDER.to_string());
                    }
                    // Now parse the value for this key
                    stack.push(ParseState::InDictValue {
                        entries,
                        key: key.clone(),
                        last_key: key,
                    });
                    stack.push(ParseState::ParseValue);
                } else {
                    return Err(ERR_DICT_KEY_MUST_BE_STRING.to_string());
                }
            }

            ParseState::InDictValue {
                mut entries,
                key,
                last_key,
            } => {
                // Pop the parsed value from value_stack
                if let Some(value) = value_stack.pop() {
                    entries.insert(key, value);
                }

                // Check if dictionary is complete
                match source.current() {
                    Some(END_MARKER) => {
                        source.next();
                        value_stack.push(Dictionary(entries));
                    }
                    Some(_) => {
                        // Continue parsing next key-value pair
                        stack.push(ParseState::InDictKey { entries, last_key });
                        stack.push(ParseState::ParseValue);
                    }
                    None => {
                        return Err(ERR_UNTERMINATED_DICTIONARY.to_string());
                    }
                }
            }
        }
    }

    // Should have exactly one value left
    if value_stack.len() == 1 {
        Ok(value_stack.pop().unwrap())
    } else {
        Err("Parser error: unexpected state".to_string())
    }
}

/// Parses bencode data from a byte slice using iterative parser.
///
/// # Arguments
/// * `data` - The byte slice containing bencode-encoded data
///
/// # Returns
/// * `Result<Node, String>` - Parsed Node or error message
pub fn parse_bytes_iterative(data: &[u8]) -> Result<Node, String> {
    use crate::io::sources::buffer::Buffer;
    let mut source = Buffer::new(data);
    parse_iterative(&mut source)
}

/// Parses bencode data from a string using iterative parser.
///
/// # Arguments
/// * `data` - The string containing bencode-encoded data
///
/// # Returns
/// * `Result<Node, String>` - Parsed Node or error message
pub fn parse_str_iterative(data: &str) -> Result<Node, String> {
    parse_bytes_iterative(data.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BufferSource;

    #[test]
    fn parse_integer_works() {
        let mut source = BufferSource::new(b"i32e");
        assert!(matches!(
            parse_iterative(&mut source),
            Ok(Node::Integer(32))
        ));
    }

    #[test]
    fn parse_string_works() {
        let mut source = BufferSource::new(b"4:test");
        assert!(matches!(parse_iterative(&mut source), Ok(Node::Str(s)) if s == "test"));
    }

    #[test]
    fn parse_list_works() {
        let mut source = BufferSource::new(b"li32ei33ee");
        match parse_iterative(&mut source) {
            Ok(Node::List(list)) => {
                assert_eq!(list.len(), 2);
                assert!(matches!(&list[0], Node::Integer(32)));
                assert!(matches!(&list[1], Node::Integer(33)));
            }
            _ => panic!("Expected list"),
        }
    }

    #[test]
    fn parse_dictionary_works() {
        let mut source = BufferSource::new(b"d4:testi32ee");
        match parse_iterative(&mut source) {
            Ok(Dictionary(dict)) => {
                assert_eq!(dict.len(), 1);
                assert!(matches!(dict.get("test"), Some(Node::Integer(32))));
            }
            _ => panic!("Expected dictionary"),
        }
    }

    #[test]
    fn parse_nested_list_works() {
        let mut source = BufferSource::new(b"lli1ei2eeli3ei4eee");
        match parse_iterative(&mut source) {
            Ok(Node::List(list)) => {
                assert_eq!(list.len(), 2);
                assert!(matches!(&list[0], Node::List(_)));
                assert!(matches!(&list[1], Node::List(_)));
            }
            _ => panic!("Expected nested list"),
        }
    }

    #[test]
    fn parse_deeply_nested_works() {
        // Create a deeply nested list: [[[[[[10]]]]]]
        let mut nested = String::from("i10e");
        for _ in 0..100 {
            nested = format!("l{}e", nested);
        }
        let result = parse_bytes_iterative(nested.as_bytes());
        assert!(result.is_ok());
    }

    #[test]
    fn parse_empty_list_works() {
        let mut source = BufferSource::new(b"le");
        match parse_iterative(&mut source) {
            Ok(Node::List(list)) => assert_eq!(list.len(), 0),
            _ => panic!("Expected empty list"),
        }
    }

    #[test]
    fn parse_empty_dict_works() {
        let mut source = BufferSource::new(b"de");
        match parse_iterative(&mut source) {
            Ok(Dictionary(dict)) => assert_eq!(dict.len(), 0),
            _ => panic!("Expected empty dictionary"),
        }
    }

    #[test]
    fn parse_integer_with_error() {
        let mut source = BufferSource::new(b"i32");
        assert!(matches!(parse_iterative(&mut source), Err(s) if s == ERR_UNTERMINATED_INTEGER));
    }

    #[test]
    fn parse_negative_zero_fails() {
        let mut source = BufferSource::new(b"i-0e");
        assert!(matches!(parse_iterative(&mut source), Err(s) if s == ERR_INVALID_INTEGER));
    }

    #[test]
    fn parse_unordered_dict_keys_fails() {
        let mut source = BufferSource::new(b"d3:bbci32e3:abci42ee");
        assert!(matches!(parse_iterative(&mut source), Err(s) if s == ERR_DICT_KEYS_ORDER));
    }
}
