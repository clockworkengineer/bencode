use crate::io::traits::IDestination;
use crate::nodes::node::*;
use crate::stringify::common::escape_string;

fn write_value(node: &Node, destination: &mut dyn IDestination) {
    match node {
        Node::Str(value) => {
            destination.add_byte(b'"');
            escape_string(value, destination);
            destination.add_byte(b'"');
        }
        Node::Integer(value) => {
            destination.add_bytes(&value.to_string());
        }
        Node::List(items) => {
            destination.add_byte(b'[');
            let mut is_first = true;
            for item in items {
                if !is_first {
                    destination.add_bytes(", ");
                }
                write_value(item, destination);
                is_first = false;
            }
            destination.add_byte(b']');
        }
        Node::Dictionary(items) => {
            let mut is_first = true;
            for (key, value) in items {
                if !is_first {
                    destination.add_bytes(", ");
                }
                destination.add_bytes(key);
                destination.add_bytes(" = ");
                write_value(value, destination);
                is_first = false;
            }
        }
        Node::None => {
            // In TOML, we'll represent None as null (though TOML doesn't officially support null)
            destination.add_bytes("null");
        }
    }
}
pub fn stringify(node: &Node, destination: &mut dyn IDestination) -> Result<(), String> {
    match node {
        Node::Dictionary(items) => {
            let mut is_first = true;
            for (key, value) in items {
                match value {
                    Node::Dictionary(_inner_dict) => {
                        if !is_first {
                            destination.add_byte(b'\n');
                        }
                        destination.add_bytes("\n[");
                        destination.add_bytes(key);
                        destination.add_bytes("]\n");
                        stringify(value, destination)?;
                    }
                    _ => {
                        if !is_first {
                            destination.add_byte(b'\n');
                        }
                        destination.add_bytes(key);
                        destination.add_bytes(" = ");
                        write_value(value, destination);
                    }
                }
                is_first = false;
            }
            Ok(())
        }
        _ => Err("TOML format requires a dictionary at the root level".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::io::destinations::BufferDestination::Buffer;
    use crate::nodes::node::make_node;
    use std::collections::HashMap;
    use crate::BufferDestination;

    #[test]
    fn test_stringify_string() {
        let mut destination = BufferDestination::new();
        let mut dict = std::collections::HashMap::new();
        dict.insert("key".to_string(), make_node("test"));
        let node = make_node(dict);
        stringify(&node, &mut destination).unwrap();
        assert_eq!(destination.to_string(), "key = \"test\"");
    }

    #[test]
    fn test_stringify_integer() {
        let mut destination = BufferDestination::new();
        let mut dict = std::collections::HashMap::new();
        dict.insert("key".to_string(), make_node(42));
        let node = make_node(dict);
        stringify(&node, &mut destination).unwrap();
        assert_eq!(destination.to_string(), "key = 42");
    }

    #[test]
    fn test_stringify_list() {
        let mut destination = BufferDestination::new();
        let mut dict = std::collections::HashMap::new();
        dict.insert(
            "key".to_string(),
            make_node(vec![make_node(1), make_node(2), make_node(3)]),
        );
        let node = make_node(dict);
        stringify(&node, &mut destination).unwrap();
        assert_eq!(destination.to_string(), "key = [1, 2, 3]");
    }

    #[test]
    fn test_stringify_dictionary() {
        let mut destination = BufferDestination::new();
        let mut dict = std::collections::HashMap::new();
        dict.insert("key".to_string(), make_node("value"));
        let node = make_node(dict);
        stringify(&node, &mut destination).unwrap();
        assert_eq!(destination.to_string(), "key = \"value\"");
    }

    #[test]
    fn test_stringify_nested_dictionary() {
        let mut destination = BufferDestination::new();
        let mut inner_dict = std::collections::HashMap::new();
        inner_dict.insert("inner_key".to_string(), make_node("inner_value"));
        let mut outer_dict = std::collections::HashMap::new();
        outer_dict.insert("outer_key".to_string(), make_node(inner_dict));
        let node = make_node(outer_dict);
        stringify(&node, &mut destination).unwrap();
        assert_eq!(
            destination.to_string(),
            "\n[outer_key]\ninner_key = \"inner_value\""
        );
    }

    #[test]
    fn test_stringify_none() {
        let mut destination = BufferDestination::new();
        let mut dict = std::collections::HashMap::new();
        dict.insert("key".to_string(), Node::None);
        let node = make_node(dict);
        stringify(&node, &mut destination).unwrap();
        assert_eq!(destination.to_string(), "key = null");
    }

    #[test]
    fn test_stringify_non_dictionary_root() {
        let mut destination = BufferDestination::new();
        let node = make_node("test");
        let result = stringify(&node, &mut destination);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "TOML format requires a dictionary at the root level"
        );
    }
    // #[test]
    // fn test_stringify_deeply_nested_dictionary() {
    //     let mut level3 = HashMap::new();
    //     level3.insert(
    //         "deep_key".to_string(),
    //         Node::Integer(123),
    //     );
    //     let level3 = Node::Dictionary(level3);
    //
    //     let mut level2 = HashMap::new();
    //     level2.insert("level3".to_string(), level3);
    //     let level2 = Node::Dictionary(level2);
    //
    //     let mut level1 = HashMap::new();
    //     level1.insert("level2".to_string(), level2);
    //     let level1 = Node::Dictionary(level1);
    //
    //     let mut root = HashMap::new();
    //     root.insert("level1".to_string(), level1);
    //
    //     let mut dest = BufferDestination::new();
    //     stringify(&Node::Dictionary(root), &mut dest).unwrap();
    //     assert_eq!(
    //         dest.to_string(),
    //         "\n[level1.level2.level3]\ndeep_key = 123\n"
    //     );
    // }
    #[test]
    fn test_stringify_empty_dictionary() {
        let mut dest = BufferDestination::new();
        stringify(&Node::Dictionary(HashMap::new()), &mut dest).unwrap();
        assert_eq!(dest.to_string(), "");
    }

}
