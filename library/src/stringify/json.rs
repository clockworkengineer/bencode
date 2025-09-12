use crate::nodes::node::*;
use crate::io::traits::IDestination;
use crate::stringify::common::escape_string;

/// Converts a Node structure into a JSON string representation and writes it to the given destination.
/// Handles different node types (Integer, String, List, Dictionary) according to JSON format rules.
///
/// # Arguments
/// * `node` - The Node structure to convert
/// * `destination` - The destination to write the JSON output to
pub fn stringify(node: &Node, destination: &mut dyn IDestination) -> Result<(), String> {
    
    match node {
        Node::Integer(value) => {
            destination.add_bytes(&value.to_string());
        }
        // Format a string value as JSON by wrapping it in double quotes
        Node::Str(value) => {
            destination.add_byte(b'"');
            escape_string(&value, destination);
            destination.add_byte(b'"');
        }
        Node::List(items) => {
            destination.add_byte(b'[');
            for (index, item) in items.iter().enumerate() {
                if index > 0 {
                    destination.add_byte(b',');
                }
                stringify(item, destination)?;
            }
            destination.add_byte(b']');
        }
        Node::Dictionary(items) => {
            destination.add_byte(b'{');
            let mut sorted: Vec<_> = items.iter().collect();
            sorted.sort_by(|a, b| a.0.cmp(b.0));
            for (index, (key, value)) in sorted.iter().enumerate() {
                if index > 0 {
                    destination.add_byte(b',');
                }
                destination.add_byte(b'"');
                destination.add_bytes(key);
                destination.add_bytes("\":");
                stringify(value, destination)?;
            }
            destination.add_byte(b'}');
        }
        _ => {}
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::destinations::buffer::Buffer;

    #[test]
    fn stringify_integer_works() {
        let mut destination = Buffer::new();
        stringify(&Node::Integer(42), &mut destination).unwrap();
        assert_eq!(destination.to_string(), "42");
    }

    #[test]
    fn stringify_string_works() {
        let mut destination = Buffer::new();
        stringify(&Node::Str("hello".to_string()), &mut destination).unwrap();
        assert_eq!(destination.to_string(), "\"hello\"");
    }

    #[test]
    fn stringify_list_works() {
        let mut destination = Buffer::new();
        stringify(&Node::List(vec![
            Node::Integer(1),
            Node::Integer(2),
            Node::Str("three".to_string()),
        ]), &mut destination).unwrap();
        assert_eq!(destination.to_string(), "[1,2,\"three\"]");
    }

    #[test]
    fn stringify_dictionary_works() {
        let mut destination = Buffer::new();
        let mut dict = std::collections::HashMap::new();
        dict.insert("key1".to_string(), Node::Integer(1));
        dict.insert("key2".to_string(), Node::Str("value".to_string()));
        stringify(&Node::Dictionary(dict), &mut destination).unwrap();
        assert_eq!(destination.to_string(), "{\"key1\":1,\"key2\":\"value\"}");
    }

    #[test]
    fn stringify_empty_structures_works() {
        let mut destination = Buffer::new();
        stringify(&Node::List(vec![]), &mut destination).unwrap();
        assert_eq!(destination.to_string(), "[]");
        destination.clear();
        stringify(&Node::Dictionary(std::collections::HashMap::new()), &mut destination).unwrap();
        assert_eq!(destination.to_string(), "{}");
    }

    #[test]
    fn stringify_unknown_node_works() {
        let mut destination = Buffer::new();
        stringify(&Node::None, &mut destination).unwrap();
        assert_eq!(destination.to_string(), "");
    }

    #[test]
    fn stringify_dictionary_sorting_works() {
        let mut destination = Buffer::new();
        let mut dict = std::collections::HashMap::new();
        dict.insert("z".to_string(), Node::Integer(1));
        dict.insert("a".to_string(), Node::Integer(2));
        dict.insert("m".to_string(), Node::Integer(3));
        stringify(&Node::Dictionary(dict), &mut destination).unwrap();
        assert_eq!(destination.to_string(), "{\"a\":2,\"m\":3,\"z\":1}");
    }

    // #[test]
    // fn stringify_dictionary_key_escaping_works() {
    //     let mut destination = Buffer::new();
    //     let mut dict = std::collections::HashMap::new();
    //     dict.insert("key:with\"special\\chars".to_string(), Node::Integer(1));
    //     stringify(&Node::Dictionary(dict), &mut destination).unwrap();
    //     assert_eq!(destination.to_string(), "{\"key:with\\\"special\\\\chars\":1}");
    // }

    #[test]
    fn stringify_complex_nested_structure_works() {
        let mut destination = Buffer::new();
        let mut inner_dict1 = std::collections::HashMap::new();
        inner_dict1.insert("x".to_string(), Node::Integer(1));
        let mut inner_dict2 = std::collections::HashMap::new();
        inner_dict2.insert("y".to_string(), Node::List(vec![
            Node::Str("a".to_string()),
            Node::Dictionary(inner_dict1),
            Node::Integer(42)
        ]));
        stringify(&Node::Dictionary(inner_dict2), &mut destination).unwrap();
        assert_eq!(destination.to_string(), "{\"y\":[\"a\",{\"x\":1},42]}");
    }

    #[test]
    fn stringify_nested_structures_works() {
        let mut destination = Buffer::new();
        let inner_list = Node::List(vec![Node::Integer(1), Node::Integer(2)]);
        let mut inner_dict = std::collections::HashMap::new();
        inner_dict.insert("key".to_string(), Node::Str("value".to_string()));
        let dict = Node::Dictionary(inner_dict);

        stringify(&Node::List(vec![inner_list, dict]), &mut destination).unwrap();
        assert_eq!(destination.to_string(), "[[1,2],{\"key\":\"value\"}]");
    }
    
}

