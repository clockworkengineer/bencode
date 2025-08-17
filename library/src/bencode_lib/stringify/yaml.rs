//! YAML serialization functionality for Bencode nodes.
//! Provides methods to convert Bencode data structures into YAML formatted output.

use crate::bencode_lib::nodes::node::*;
use crate::bencode_lib::io::traits::IDestination;

/// Writes the specified number of indentation spaces to the destination.
///
/// # Arguments
/// * `level` - The indentation level (number of 2-space indents)
/// * `destination` - The output destination to write to
fn write_indent(level: usize, destination: &mut dyn IDestination) {
        for _ in 1..level {
            destination.add_bytes("  ");
        }
}

/// Recursively writes a Bencode node to the destination in YAML format.
///
/// # Arguments
/// * `node` - The Bencode node to serialize
/// * `level` - Current indentation level
/// * `destination` - The output destination to write to
fn write_node(node: &Node, level: usize, destination: &mut dyn IDestination) {
    match node {
        // Write integer values directly
        Node::Integer(n) => destination.add_bytes(&n.to_string()),
        // Write strings with quotes and proper UTF-8 encoding
        Node::Str(s) => destination.add_bytes(&format!("\"{}\"", String::from_utf8_lossy(s.as_ref()))),
        // Write lists with proper YAML array formatting
        Node::List(items) => {
            if items.is_empty() {
                destination.add_bytes("[]")
            } else {
                destination.add_bytes("\n");
                for item in items {
                    write_indent(level + 1, destination);
                    destination.add_bytes("- ");
                    write_node(item, level + 1, destination);
                    destination.add_bytes("\n");
                }
            }
        }
        // Write dictionaries with proper YAML mapping format
        Node::Dictionary(dict) => {
            if dict.is_empty() {
                destination.add_bytes("{}")
            } else {
                destination.add_bytes("\n");
                let mut sorted: Vec<_> = dict.iter().collect();
                sorted.sort_by(|a, b| a.0.cmp(b.0));
                for (key, value) in sorted {
                    write_indent(level + 1, destination);
                    destination.add_bytes(&format!("{}: ", String::from_utf8_lossy(key.as_ref())));
                    write_node(value, level + 1, destination);
                    destination.add_bytes("\n");
                }
            }
        }
        // Handle unknown/unsupported node types
        _ => destination.add_bytes("unknown"),
    }
}

/// Converts a Bencode node to YAML format and writes it to the destination.
///
/// # Arguments
/// * `node` - The root Bencode node to serialize
/// * `destination` - The output destination to write the YAML to
pub fn stringify(node: &Node, destination: &mut dyn IDestination) {
    write_node(node, 0, destination);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bencode_lib::io::destinations::buffer::Buffer;

    #[test]
    fn stringify_empty_list_works() {
        let mut destination = Buffer::new();
        stringify(&Node::List(vec![]), &mut destination);
        assert_eq!(destination.to_string(), "[]");
    }

    #[test]
    fn stringify_list_works() {
        let mut destination = Buffer::new();
        stringify(&Node::List(vec![Node::Integer(1), Node::Integer(2)]), &mut destination);
        assert_eq!(destination.to_string(), "\n- 1\n- 2\n");
    }

    #[test]
    fn stringify_empty_dictionary_works() {
        let mut destination = Buffer::new();
        let dict = std::collections::HashMap::new();
        stringify(&Node::Dictionary(dict), &mut destination);
        assert_eq!(destination.to_string(), "{}");
    }

    #[test]
    fn stringify_dictionary_works() {
        let mut destination = Buffer::new();
        let mut dict = std::collections::HashMap::new();
        dict.insert("key".to_string(), Node::Integer(1));
        stringify(&Node::Dictionary(dict), &mut destination);
        assert_eq!(destination.to_string(), "\nkey: 1\n");
    }

    #[test]
    fn stringify_integer_works() {
        let mut destination = Buffer::new();
        stringify(&Node::Integer(42), &mut destination);
        assert_eq!(destination.to_string(), "42");
    }

    #[test]
    fn stringify_string_works() {
        let mut destination = Buffer::new();
        stringify(&Node::Str(String::from("test")), &mut destination);
        assert_eq!(destination.to_string(), "\"test\"");
    }
}




