use crate::nodes::node::*;
use crate::io::traits::IDestination;
use crate::stringify::common::escape_string;

pub fn stringify(node: &Node, destination: &mut dyn IDestination) {
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
                        stringify(value, destination);
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
        }
        _ => write_value(node, destination),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::destinations::buffer::Buffer;
    use crate::nodes::node::make_node;

    #[test]
    fn test_stringify_string() {
        let mut destination = Buffer::new();
        let node = make_node("test");
        stringify(&node, &mut destination);
        assert_eq!(destination.to_string(), "\"test\"");
    }

    #[test]
    fn test_stringify_integer() {
        let mut destination = Buffer::new();
        let node = make_node(42);
        stringify(&node, &mut destination);
        assert_eq!(destination.to_string(), "42");
    }

    #[test]
    fn test_stringify_list() {
        let mut destination = Buffer::new();
        let node = make_node(vec![make_node(1), make_node(2), make_node(3)]);
        stringify(&node, &mut destination);
        assert_eq!(destination.to_string(), "[1, 2, 3]");
    }

    #[test]
    fn test_stringify_dictionary() {
        let mut destination = Buffer::new();
        let mut dict = std::collections::HashMap::new();
        dict.insert("key".to_string(), make_node("value"));
        let node = make_node(dict);
        stringify(&node, &mut destination);
        assert_eq!(destination.to_string(), "key = \"value\"");
    }

    #[test]
    fn test_stringify_nested_dictionary() {
        let mut destination = Buffer::new();
        let mut inner_dict = std::collections::HashMap::new();
        inner_dict.insert("inner_key".to_string(), make_node("inner_value"));
        let mut outer_dict = std::collections::HashMap::new();
        outer_dict.insert("outer_key".to_string(), make_node(inner_dict));
        let node = make_node(outer_dict);
        stringify(&node, &mut destination);
        assert_eq!(destination.to_string(), "\n[outer_key]\ninner_key = \"inner_value\"");
    }

    #[test]
    fn test_stringify_none() {
        let mut destination = Buffer::new();
        let node = Node::None;
        stringify(&node, &mut destination);
        assert_eq!(destination.to_string(), "null");
    }
}

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


