use crate::bencode_lib::nodes::node::*;
use crate::bencode_lib::io::destinations::buffer::{Buffer, IDestination};

pub fn stringify(node: &Node, destination: &mut dyn IDestination) {
    match node {
        Node::Integer(value) => {
            let s = format!("i{}e", value);
            destination.add_bytes(s.as_str());
        }
        Node::Str(value) => {
            let s = format!("{}:{}", value.len(), value);
            destination.add_bytes(s.as_str());
        }
        Node::List(items) => {
            destination.add_byte(b'l');
            for item in items {
                stringify(item, destination);
            }
            destination.add_byte(b'e');
        }
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn stringify_integer_works() {
        let mut destination = Buffer::new();
        stringify(&make_node(32), &mut destination);
        assert_eq!(destination.to_string(), "i32e");
    }

    #[test]
    fn stringify_string_works() {
        let mut destination = Buffer::new();
        stringify(&make_node("test"), &mut destination);
        assert_eq!(destination.to_string(), "4:test");
    }

    #[test]
    fn stringify_empty_list_works() {
        let mut destination = Buffer::new();
        stringify(&make_node(vec![] as Vec<Node>), &mut destination);
        assert_eq!(destination.to_string(), "le");
    }

    #[test]
    fn stringify_list_works() {
        let mut destination = Buffer::new();
        let list = vec![make_node(32), make_node("test")];
        stringify(&make_node(list), &mut destination);
        assert_eq!(destination.to_string(), "li32e4:teste");
    }

    #[test]
    fn stringify_empty_dictionary_works() {
        let mut destination = Buffer::new();
        stringify(&make_node(HashMap::new()), &mut destination);
        assert_eq!(destination.to_string(), "de");
    }

    #[test]
    fn stringify_dictionary_works() {
        let mut destination = Buffer::new();
        let mut dict = HashMap::new();
        dict.insert(String::from("key"), make_node(32));
        stringify(&make_node(dict), &mut destination);
        assert_eq!(destination.to_string(), "d3:keyi32ee");
    }
}