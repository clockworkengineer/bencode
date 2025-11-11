#[cfg(not(feature = "std"))]
use alloc::string::{String, ToString};

use crate::io::traits::IDestination;
use crate::nodes::node::*;
use crate::stringify::common::escape_string;

/// Converts a bencode Node into XML format and writes it to the given destination.
/// Each node type is wrapped in appropriate XML tags based on its type.
///
/// # Arguments
/// * `node` - The bencode Node to convert
/// * `destination` - The destination to write the XML output to
pub fn stringify(node: &Node, destination: &mut dyn IDestination) -> Result<(), String> {
    match node {
        Node::Str(value) => {
            // Wrap string value in <string> tags
            destination.add_bytes("<string>");
            escape_string(&value, destination);
            destination.add_bytes("</string>");
        }
        Node::Integer(value) => {
            // Wrap integer value in <integer> tags
            destination.add_bytes("<integer>");
            destination.add_bytes(&value.to_string());
            destination.add_bytes("</integer>");
        }
        Node::List(items) => {
            // Create a list container and recursively stringify each item
            destination.add_bytes("<list>");
            for item in items {
                stringify(item, destination)?;
            }
            destination.add_bytes("</list>");
        }
        Node::Dictionary(items) => {
            // Create a dictionary container with key-value pair items
            destination.add_bytes("<dictionary>");
            for (key, value) in items {
                destination.add_bytes("<item><key>");
                destination.add_bytes(key);
                destination.add_bytes("</key><value>");
                stringify(value, destination)?;
                destination.add_bytes("</value></item>");
            }
            destination.add_bytes("</dictionary>");
        }
        Node::None => {}
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::destinations::buffer::Buffer;

    #[test]
    fn test_string_node() {
        let mut destination = Buffer::new();
        stringify(&Node::Str("test".into()), &mut destination).unwrap();
        assert_eq!(destination.to_string(), "<string>test</string>");
    }

    #[test]
    fn test_integer_node() {
        let mut destination = Buffer::new();
        stringify(&Node::Integer(42), &mut destination).unwrap();
        assert_eq!(destination.to_string(), "<integer>42</integer>");
    }

    #[test]
    fn test_list_node() {
        let mut destination = Buffer::new();
        stringify(
            &Node::List(vec![Node::Integer(1), Node::Str("test".into())]),
            &mut destination,
        )
        .unwrap();
        assert_eq!(
            destination.to_string(),
            "<list><integer>1</integer><string>test</string></list>"
        );
    }

    #[test]
    fn test_dictionary_node() {
        let mut destination = Buffer::new();
        let mut dict = std::collections::HashMap::new();
        dict.insert("key".into(), Node::Str("value".into()));
        stringify(&Node::Dictionary(dict), &mut destination).unwrap();
        assert_eq!(
            destination.to_string(),
            "<dictionary><item><key>key</key><value><string>value</string></value></item></dictionary>"
        );
    }

    #[test]
    fn test_none_node() {
        let mut destination = Buffer::new();
        stringify(&Node::None, &mut destination).unwrap();
        assert_eq!(destination.to_string(), "");
    }
}
