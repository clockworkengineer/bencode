use crate::bencode_lib::io::traits::IDestination;
use crate::bencode_lib::nodes::node::*;

/// Converts a bencode Node into XML format and writes it to the given destination.
/// Each node type is wrapped in appropriate XML tags based on its type.
///
/// # Arguments
/// * `node` - The bencode Node to convert
/// * `destination` - The destination to write the XML output to
pub fn stringify(node: &Node, destination: &mut dyn IDestination) {
    match node {
        Node::Str(value) => {
            // Wrap string value in <string> tags
            destination.add_bytes("<string>");
            for &byte in value.as_bytes() {
                if byte == b'"' || byte == b'\\' {
                    destination.add_byte(b'\\');
                    destination.add_byte(byte);
                } else if byte.is_ascii_graphic() || byte == b' ' {
                    destination.add_byte(byte);
                } else {
                    // Convert unprintable characters to \u escape sequence
                    let escaped = format!("\\u{:04x}", byte);
                    destination.add_bytes(&escaped);
                }
            }
            destination.add_bytes("</string>");
        }
        Node::Integer(value) => {
            // Wrap integer value in <integer> tags
            destination.add_bytes("<integer>");
            destination.add_bytes(&value.to_string());
            destination.add_bytes("</integer>");
        }
        Node::List(items) => {
            // Create list container and recursively stringify each item
            destination.add_bytes("<list>");
            for item in items {
                stringify(item, destination);
            }
            destination.add_bytes("</list>");
        }
        Node::Dictionary(items) => {
            // Create dictionary container with key-value pair items
            destination.add_bytes("<dictionary>");
            for (key, value) in items {
                destination.add_bytes("<item><key>");
                destination.add_bytes(key);
                destination.add_bytes("</key><value>");
                stringify(value, destination);
                destination.add_bytes("</value></item>");
            }
            destination.add_bytes("</dictionary>");
        }
        Node::None => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bencode_lib::io::destinations::buffer::Buffer;

    #[test]
    fn test_string_node() {
        let mut destination = Buffer::new();
        stringify(&Node::Str("test".into()), &mut destination);
        assert_eq!(destination.to_string(), "<string>test</string>");
    }

    #[test]
    fn test_integer_node() {
        let mut destination = Buffer::new();
        stringify(&Node::Integer(42), &mut destination);
        assert_eq!(destination.to_string(), "<integer>42</integer>");
    }

    #[test]
    fn test_list_node() {
        let mut destination = Buffer::new();
        stringify(
            &Node::List(vec![Node::Integer(1), Node::Str("test".into())]),
            &mut destination,
        );
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
        stringify(&Node::Dictionary(dict), &mut destination);
        assert_eq!(
            destination.to_string(),
            "<dictionary><item><key>key</key><value><string>value</string></value></item></dictionary>"
        );
    }

    #[test]
    fn test_empty_list() {
        let mut destination = Buffer::new();
        stringify(&Node::List(vec![]), &mut destination);
        assert_eq!(destination.to_string(), "<list></list>");
    }

    #[test]
    fn test_empty_dictionary() {
        let mut destination = Buffer::new();
        stringify(
            &Node::Dictionary(std::collections::HashMap::new()),
            &mut destination,
        );
        assert_eq!(destination.to_string(), "<dictionary></dictionary>");
    }

    #[test]
    fn test_none_node() {
        let mut destination = Buffer::new();
        stringify(&Node::None, &mut destination);
        assert_eq!(destination.to_string(), "");
    }

    #[test]
    fn test_nested_dictionary() {
        let mut destination = Buffer::new();
        let mut inner_dict = std::collections::HashMap::new();
        inner_dict.insert("inner_key".into(), Node::Integer(42));
        let mut outer_dict = std::collections::HashMap::new();
        outer_dict.insert("outer_key".into(), Node::Dictionary(inner_dict));
        stringify(&Node::Dictionary(outer_dict), &mut destination);
        assert_eq!(
            destination.to_string(),
            "<dictionary><item><key>outer_key</key><value><dictionary><item><key>inner_key</key><value><integer>42</integer></value></item></dictionary></value></item></dictionary>"
        );
    }

    #[test]
    fn test_nested_list_mixed() {
        let mut destination = Buffer::new();
        let nested = Node::List(vec![
            Node::Integer(1),
            Node::List(vec![Node::Str("nested".into())]),
            Node::None,
        ]);
        stringify(&nested, &mut destination);
        assert_eq!(
            destination.to_string(),
            "<list><integer>1</integer><list><string>nested</string></list></list>"
        );
    }
}
