//! Module providing functionality to convert bencode nodes into their string representation.
//! Implements the bencode encoding rules for different node types.

#[cfg(not(feature = "std"))]
use alloc::{format, string::String, vec::Vec};

use crate::constants::{BYTE_DICT_START, BYTE_END, BYTE_LIST_START};
use crate::io::traits::IDestination;
use crate::nodes::node::*;

/// Converts a bencode Node into its string representation and writes it to the destination.
///
/// # Arguments
/// * `node` - The bencode node to stringify
/// * `destination` - The destination to write the string representation to
pub fn stringify(node: &Node, destination: &mut dyn IDestination) -> Result<(), String> {
    match node {
        // Handle integer nodes by formatting as "i<value>e"
        Node::Integer(value) => {
            let s = format!("i{}e", value);
            destination.add_bytes(s.as_str());
        }
        // Handle string nodes by formatting as "<length>:<value>"
        Node::Str(value) => {
            let s = format!("{}:{}", value.len(), value);
            destination.add_bytes(s.as_str());
        }
        // Handle list nodes by wrapping items with 'l' and 'e' markers
        Node::List(items) => {
            destination.add_byte(BYTE_LIST_START);
            for item in items {
                stringify(item, destination)?;
            }
            destination.add_byte(BYTE_END);
        }
        // Handle dictionary nodes by wrapping sorted key-value pairs with 'd' and 'e' markers
        Node::Dictionary(items) => {
            destination.add_byte(BYTE_DICT_START);
            let mut sorted: Vec<_> = items.iter().collect();
            sorted.sort_by(|a, b| a.0.cmp(b.0));
            for (key, value) in sorted {
                stringify(&Node::Str(key.clone()), destination)?;
                stringify(value, destination)?;
            }
            destination.add_byte(BYTE_END);
        }
        // Skip None nodes as they don't have a string representation
        Node::None => {
            // Do nothing for None nodes or handle as appropriate
        }
    }
    Ok(())
}

/// Converts a bencode Node into its string representation and returns it as a String.
/// This is a convenience function that creates a BufferDestination internally.
///
/// # Arguments
/// * `node` - The bencode node to stringify
///
/// # Returns
/// * `Result<String, String>` - The bencode string representation or error message
pub fn stringify_to_string(node: &Node) -> Result<String, String> {
    use crate::io::destinations::buffer::Buffer;
    let mut destination = Buffer::new();
    stringify(node, &mut destination)?;
    Ok(destination.to_string())
}

/// Converts a bencode Node into its byte representation and returns it as a Vec<u8>.
/// This is a convenience function that creates a BufferDestination internally.
///
/// # Arguments
/// * `node` - The bencode node to stringify
///
/// # Returns
/// * `Result<Vec<u8>, String>` - The bencode byte representation or error message
pub fn stringify_to_bytes(node: &Node) -> Result<Vec<u8>, String> {
    let s = stringify_to_string(node)?;
    Ok(s.into_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BufferDestination;
    use std::collections::HashMap;

    #[test]
    fn stringify_integer_works() {
        let mut destination = BufferDestination::new();
        stringify(&make_node(32), &mut destination).unwrap();
        assert_eq!(destination.to_string(), "i32e");
    }

    #[test]
    fn stringify_string_works() {
        let mut destination = BufferDestination::new();
        stringify(&make_node("test"), &mut destination).unwrap();
        assert_eq!(destination.to_string(), "4:test");
    }

    #[test]
    fn stringify_empty_list_works() {
        let mut destination = BufferDestination::new();
        stringify(&make_node(vec![] as Vec<Node>), &mut destination).unwrap();
        assert_eq!(destination.to_string(), "le");
    }

    #[test]
    fn stringify_list_works() {
        let mut destination = BufferDestination::new();
        let list = vec![make_node(32), make_node("test")];
        stringify(&make_node(list), &mut destination).unwrap();
        assert_eq!(destination.to_string(), "li32e4:teste");
    }

    #[test]
    fn stringify_empty_dictionary_works() {
        let mut destination = BufferDestination::new();
        stringify(&make_node(HashMap::new()), &mut destination).unwrap();
        assert_eq!(destination.to_string(), "de");
    }

    #[test]
    fn stringify_dictionary_works() {
        let mut destination = BufferDestination::new();
        let mut dict = HashMap::new();
        dict.insert(String::from("key"), make_node(32));
        stringify(&make_node(dict), &mut destination).unwrap();
        assert_eq!(destination.to_string(), "d3:keyi32ee");
    }

    #[test]
    fn stringify_none_works() {
        let mut destination = BufferDestination::new();
        stringify(&Node::None, &mut destination).unwrap();
        assert_eq!(destination.to_string(), "");
    }

    // --- Integer edge cases ---

    #[test]
    fn stringify_integer_zero() {
        assert_eq!(stringify_to_string(&make_node(0i64)).unwrap(), "i0e");
    }

    #[test]
    fn stringify_integer_negative() {
        assert_eq!(stringify_to_string(&make_node(-42i64)).unwrap(), "i-42e");
    }

    #[test]
    fn stringify_integer_max_i64() {
        let expected = format!("i{}e", i64::MAX);
        assert_eq!(
            stringify_to_string(&Node::Integer(i64::MAX)).unwrap(),
            expected
        );
    }

    #[test]
    fn stringify_integer_min_i64() {
        let expected = format!("i{}e", i64::MIN);
        assert_eq!(
            stringify_to_string(&Node::Integer(i64::MIN)).unwrap(),
            expected
        );
    }

    // --- String edge cases ---

    #[test]
    fn stringify_empty_string() {
        assert_eq!(stringify_to_string(&make_node("")).unwrap(), "0:");
    }

    #[test]
    fn stringify_single_char_string() {
        assert_eq!(stringify_to_string(&make_node("x")).unwrap(), "1:x");
    }

    #[test]
    fn stringify_string_with_spaces() {
        assert_eq!(
            stringify_to_string(&make_node("hello world")).unwrap(),
            "11:hello world"
        );
    }

    #[test]
    fn stringify_string_length_matches_byte_len() {
        // "café" is 4 Unicode chars but 5 UTF-8 bytes
        let s = "café";
        let node = Node::Str(s.to_string());
        let encoded = stringify_to_string(&node).unwrap();
        let expected_len = s.len(); // Rust String::len() is byte length
        assert_eq!(encoded, format!("{}:{}", expected_len, s));
    }

    // --- List edge cases ---

    #[test]
    fn stringify_list_of_integers() {
        let list = vec![make_node(1i64), make_node(2i64), make_node(3i64)];
        assert_eq!(
            stringify_to_string(&make_node(list)).unwrap(),
            "li1ei2ei3ee"
        );
    }

    #[test]
    fn stringify_list_of_strings() {
        let list = vec![make_node("foo"), make_node("bar")];
        assert_eq!(
            stringify_to_string(&make_node(list)).unwrap(),
            "l3:foo3:bare"
        );
    }

    #[test]
    fn stringify_nested_list() {
        let inner = vec![make_node(1i64), make_node(2i64)];
        let outer = vec![make_node(inner), make_node(3i64)];
        assert_eq!(
            stringify_to_string(&make_node(outer)).unwrap(),
            "lli1ei2eei3ee"
        );
    }

    #[test]
    fn stringify_list_with_none_skipped() {
        let list = vec![make_node(1i64), Node::None, make_node(2i64)];
        // None produces no output, so it's skipped between the two integers
        assert_eq!(stringify_to_string(&make_node(list)).unwrap(), "li1ei2ee");
    }

    // --- Dictionary edge cases ---

    #[test]
    fn stringify_dictionary_sorts_keys() {
        let mut dict = HashMap::new();
        dict.insert(String::from("z"), make_node(1i64));
        dict.insert(String::from("a"), make_node(2i64));
        dict.insert(String::from("m"), make_node(3i64));
        let result = stringify_to_string(&make_node(dict)).unwrap();
        assert_eq!(result, "d1:ai2e1:mi3e1:zi1ee");
    }

    #[test]
    fn stringify_dictionary_string_value() {
        let mut dict = HashMap::new();
        dict.insert(String::from("key"), make_node("value"));
        assert_eq!(
            stringify_to_string(&make_node(dict)).unwrap(),
            "d3:key5:valuee"
        );
    }

    #[test]
    fn stringify_dictionary_list_value() {
        let mut dict = HashMap::new();
        dict.insert(
            String::from("nums"),
            make_node(vec![make_node(1i64), make_node(2i64)]),
        );
        assert_eq!(
            stringify_to_string(&make_node(dict)).unwrap(),
            "d4:numsli1ei2eee"
        );
    }

    #[test]
    fn stringify_nested_dictionary() {
        let mut inner = HashMap::new();
        inner.insert(String::from("x"), make_node(9i64));
        let mut outer = HashMap::new();
        outer.insert(String::from("inner"), make_node(inner));
        assert_eq!(
            stringify_to_string(&make_node(outer)).unwrap(),
            "d5:innerd1:xi9eee"
        );
    }

    #[test]
    fn stringify_dictionary_multiple_sorted_keys() {
        let mut dict = HashMap::new();
        dict.insert(String::from("age"), make_node(25i64));
        dict.insert(String::from("name"), make_node("John"));
        let result = stringify_to_string(&make_node(dict)).unwrap();
        assert_eq!(result, "d3:agei25e4:name4:Johne");
    }

    // --- stringify_to_bytes ---

    #[test]
    fn stringify_to_bytes_integer() {
        let bytes = stringify_to_bytes(&make_node(42i64)).unwrap();
        assert_eq!(bytes, b"i42e");
    }

    #[test]
    fn stringify_to_bytes_string() {
        let bytes = stringify_to_bytes(&make_node("hi")).unwrap();
        assert_eq!(bytes, b"2:hi");
    }

    #[test]
    fn stringify_to_bytes_list() {
        let bytes = stringify_to_bytes(&make_node(vec![make_node(1i64)])).unwrap();
        assert_eq!(bytes, b"li1ee");
    }

    // --- Round-trip consistency ---

    #[test]
    fn stringify_to_string_and_bytes_consistent() {
        let node = make_node(vec![make_node("hello"), make_node(7i64)]);
        let s = stringify_to_string(&node).unwrap();
        let b = stringify_to_bytes(&node).unwrap();
        assert_eq!(s.as_bytes(), b.as_slice());
    }

    #[test]
    fn stringify_roundtrip_with_default_parser() {
        use crate::parse_bytes;
        let original = {
            let mut dict = HashMap::new();
            dict.insert(String::from("age"), make_node(30i64));
            dict.insert(String::from("name"), make_node("Alice"));
            make_node(dict)
        };
        let encoded = stringify_to_string(&original).unwrap();
        let parsed = parse_bytes(encoded.as_bytes()).unwrap();
        let re_encoded = stringify_to_string(&parsed).unwrap();
        assert_eq!(encoded, re_encoded);
    }
}
