//! Non-recursive bencode parser implementation that uses a stack-based approach
//! to handle nested data structures without recursion.

use crate::bencode_lib::nodes::node::Node;
use std::collections::HashMap;
use crate::bencode_lib::io::traits::ISource;
use crate::bencode_lib::error::messages::*;

/// Parses bencode data from the given source using a non-recursive, stack-based approach.
///
/// # Arguments
/// * `source` - The source containing bencode-encoded data to parse
///
/// # Returns
/// * `Ok(Node)` - Successfully parsed bencode data as a Node
/// * `Err(String)` - Error message if parsing fails
pub fn parse(source: &mut dyn ISource) -> Result<Node, String> {
    let mut stack: Vec<(Node, usize)> = Vec::new();
    let mut current_string = String::new();
    let mut current_number: String = String::new();

    while source.more() {
        match source.current() {
            Some('i') => {
                source.next();
                while let Some(c) = source.current() {
                    if c == 'e' {
                        break;
                    }
                    current_number.push(c);
                    source.next();
                }
                if source.current() != Some('e') {
                    return Err("Invalid integer format".to_string());
                }
                source.next();
                let value = current_number.parse::<i64>()
                    .map_err(|_| ERR_INVALID_INTEGER.to_string())?;
                current_number.clear();

                if stack.is_empty() {
                    return Ok(Node::Integer(value));
                }
                handle_value(&mut stack, Node::Integer(value))?;
            }
            Some('l') => {
                source.next();
                stack.push((Node::List(Vec::new()), 0));
            }
            Some('d') => {
                source.next();
                stack.push((Node::Dictionary(HashMap::new()), 0));
            }
            Some('e') => {
                source.next();
                if stack.is_empty() {
                    return Err("Unexpected end marker".to_string());
                }
                let (node, _) = stack.pop().unwrap();
                if stack.is_empty() {
                    return Ok(node);
                }
                handle_value(&mut stack, node)?;
            }
            Some(c) if c.is_ascii_digit() => {
                while let Some(c) = source.current() {
                    if c == ':' {
                        break;
                    }
                    current_string.push(c);
                    source.next();
                }
                if source.current() != Some(':') {
                    return Err("Invalid string length format".to_string());
                }
                source.next();

                let length = current_string.parse::<usize>()
                    .map_err(|_| ERR_INVALID_STRING_LENGTH.to_string())?;
                current_string.clear();

                for _ in 0..length {
                    if let Some(c) = source.current() {
                        current_string.push(c);
                        source.next();
                    } else {
                        return Err("Unexpected end of input".to_string());
                    }
                }

                let string_value = current_string.clone();
                current_string.clear();

                if stack.is_empty() {
                    return Ok(Node::Str(string_value));
                }
                handle_value(&mut stack, Node::Str(string_value))?;
            }
            Some(_) => {
                source.next();
            }
            None => break,
        }
    }

    if stack.is_empty() {
        Err(ERR_EMPTY_INPUT
            .to_string())
    } else {
        Err("Incomplete input".to_string())
    }
}

/// Handles adding a value to the current container (list or dictionary) on top of the stack.
///
/// # Arguments
/// * `stack` - The stack of nested containers being built
/// * `value` - The value to add to the current container
///
/// # Returns
/// * `Ok(())` - Value was successfully added
/// * `Err(String)` - Error message if the value couldn't be added
fn handle_value(stack: &mut Vec<(Node, usize)>, value: Node) -> Result<(), String> {
    let last = stack.last_mut().unwrap();
    match last.0 {
        Node::List(ref mut v) => {
            v.push(value);
        }
        Node::Dictionary(ref mut m) => {
            if last.1 % 2 == 0 {
                if let Node::Str(key) = value {
                    last.1 += 1;
                    // Using a default null-like value until the real value is inserted
                    m.insert(key, Node::List(Vec::new()));
                } else {
                    return Err("Dictionary key must be a string".to_string());
                }
            } else {
                if let Some(key) = m.keys().last() {
                    m.insert(key.clone(), value);
                }
                last.1 += 1;
            }
        }
        _ => return Err("Invalid stack state".to_string()),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bencode_lib::io::sources::buffer::Buffer;

    #[test]
    fn test_parse_integer() {
        let mut source = Buffer::new(b"i42e");
        let result = parse(&mut source);
        assert!(result.is_ok());
        if let Ok(Node::Integer(val)) = result {
            assert_eq!(val, 42);
        } else {
            panic!("Expected Node::Integer(42)");
        }
    }

    #[test]
    fn test_parse_string() {
        let mut source = Buffer::new(b"4:test");
        let result = parse(&mut source);
        assert!(result.is_ok());
        if let Ok(Node::Str(val)) = result {
            assert_eq!(val, "test".to_string());
        } else {
            panic!("Expected Node::Str(\"test\")");
        }
    }

    #[test]
    fn test_parse_list() {
        let mut source = Buffer::new(b"li42e4:teste");
        let result = parse(&mut source);
        assert!(result.is_ok());
        if let Ok(Node::List(items)) = result {
            assert_eq!(items.len(), 2);
            if let Node::Integer(val) = &items[0] {
                assert_eq!(*val, 42);
            } else {
                panic!("Expected first item to be Node::Integer(42)");
            }
            if let Node::Str(val) = &items[1] {
                assert_eq!(val, "test");
            } else {
                panic!("Expected second item to be Node::Str(\"test\")");
            }
        } else {
            panic!("Expected Node::List");
        }
    }

    #[test]
    fn test_parse_dictionary() {
        let mut source = Buffer::new(b"d3:key5:valuee");
        let result = parse(&mut source);
        assert!(result.is_ok());
        if let Ok(Node::Dictionary(map)) = result {
            assert_eq!(map.len(), 1);
            assert!(map.contains_key("key"));
            if let Some(Node::Str(val)) = map.get("key") {
                assert_eq!(val, "value");
            } else {
                panic!("Expected value for key \"key\" to be Node::Str(\"value\")");
            }
        } else {
            panic!("Expected Node::Dictionary");
        }
    }

    #[test]
    fn test_invalid_integer() {
        let mut source = Buffer::new(b"i42");
        assert!(parse(&mut source).is_err());
    }

    #[test]
    fn test_invalid_string_length() {
        let mut source = Buffer::new(b"4:te");
        assert!(parse(&mut source).is_err());
    }

    #[test]
    fn test_incomplete_list() {
        let mut source = Buffer::new(b"li42e");
        assert!(parse(&mut source).is_err());
    }

    #[test]
    fn test_invalid_dictionary_key() {
        let mut source = Buffer::new(b"di42e5:valuee");
        assert!(parse(&mut source).is_err());
    }
}
