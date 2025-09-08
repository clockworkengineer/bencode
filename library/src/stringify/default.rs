//! Module providing functionality to convert bencode nodes into their string representation.
//! Implements the bencode encoding rules for different node types.

use crate::nodes::node::*;
use crate::io::traits::IDestination;

/// Converts a bencode Node into its string representation and writes it to the destination.
///
/// # Arguments
/// * `node` - The bencode node to stringify
/// * `destination` - The destination to write the string representation to
pub fn stringify(node: &Node, destination: &mut dyn IDestination) {
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
            destination.add_byte(b'l');
            for item in items {
                stringify(item, destination);
            }
            destination.add_byte(b'e');
        }
        // Handle dictionary nodes by wrapping sorted key-value pairs with 'd' and 'e' markers
        Node::Dictionary(items) => {
            destination.add_byte(b'd');
            let mut sorted: Vec<_> = items.iter().collect();
            sorted.sort_by(|a, b| a.0.cmp(b.0));
            for (key, value) in sorted {
                stringify(&Node::Str(key.clone()), destination);
                stringify(value, destination);
            }
            destination.add_byte(b'e');
        }
        // Skip None nodes as they don't have a string representation
        Node::None => {
            // Do nothing for None nodes or handle as appropriate
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use crate::BufferDestination;

    #[test]
    fn stringify_integer_works() {
        let mut destination = BufferDestination::new();
        stringify(&make_node(32), &mut destination);
        assert_eq!(destination.to_string(), "i32e");
    }

    #[test]
    fn stringify_string_works() {
        let mut destination = BufferDestination::new();
        stringify(&make_node("test"), &mut destination);
        assert_eq!(destination.to_string(), "4:test");
    }

    #[test]
    fn stringify_empty_list_works() {
        let mut destination = BufferDestination::new();
        stringify(&make_node(vec![] as Vec<Node>), &mut destination);
        assert_eq!(destination.to_string(), "le");
    }

    #[test]
    fn stringify_list_works() {
        let mut destination = BufferDestination::new();
        let list = vec![make_node(32), make_node("test")];
        stringify(&make_node(list), &mut destination);
        assert_eq!(destination.to_string(), "li32e4:teste");
    }

    #[test]
    fn stringify_empty_dictionary_works() {
        let mut destination = BufferDestination::new();
        stringify(&make_node(HashMap::new()), &mut destination);
        assert_eq!(destination.to_string(), "de");
    }

    #[test]
    fn stringify_dictionary_works() {
        let mut destination = BufferDestination::new();
        let mut dict = HashMap::new();
        dict.insert(String::from("key"), make_node(32));
        stringify(&make_node(dict), &mut destination);
        assert_eq!(destination.to_string(), "d3:keyi32ee");
    }

    #[test]
    fn stringify_none_works() {
        let mut destination = BufferDestination::new();
        stringify(&Node::None, &mut destination);
        assert_eq!(destination.to_string(), "");
    }

    #[test]
    fn stringify_complex_dictionary_works() {
        let mut destination = BufferDestination::new();
        let mut dict = HashMap::new();
        dict.insert(String::from("b"), make_node(1));
        dict.insert(String::from("a"), make_node(2));
        dict.insert(String::from("c"), make_node("test"));
        stringify(&make_node(dict), &mut destination);
        assert_eq!(destination.to_string(), "d1:ai2e1:bi1e1:c4:teste");
    }

    #[test]
    fn stringify_nested_dictionary_works() {
        let mut destination = BufferDestination::new();
        let mut inner_dict = HashMap::new();
        inner_dict.insert(String::from("key2"), make_node("value"));
        let mut outer_dict = HashMap::new();
        outer_dict.insert(String::from("key1"), make_node(inner_dict));
        stringify(&make_node(outer_dict), &mut destination);
        assert_eq!(destination.to_string(), "d4:key1d4:key25:valueee");
    }

    #[test]
    fn stringify_list_with_none_works() {
        let mut destination = BufferDestination::new();
        let list = vec![make_node(32), Node::None, make_node("test")];
        stringify(&make_node(list), &mut destination);
        assert_eq!(destination.to_string(), "li32e4:teste");
    }

    #[test]
    fn stringify_dictionary_with_list_works() {
        let mut destination = BufferDestination::new();
        let mut dict = HashMap::new();
        dict.insert(String::from("list"), make_node(vec![make_node(1), make_node(2)]));
        stringify(&make_node(dict), &mut destination);
        assert_eq!(destination.to_string(), "d4:listli1ei2eee");
    }
}