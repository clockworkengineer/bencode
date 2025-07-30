use crate::bencode_lib::nodes::node::Node;
use crate::bencode_lib::nodes::node::make_node;

pub fn stringify(node: &Node) -> String {
    match node {
        Node::Integer(value) => format!("i{}e", value),
        Node::Str(value) => format!("{}:{}", value.len(), value),
        Node::List(items) => {
            let mut result = String::from("l");
            for item in items {
                result.push_str(&stringify(item));
            }
            result.push('e');
            result
        }
        Node::Dictionary(items) => {
            let mut result = String::from("d");
            let mut sorted: Vec<_> = items.iter().collect();
            sorted.sort_by(|a, b| a.0.cmp(b.0));
            for (key, value) in sorted {
                result.push_str(&stringify(&Node::Str(key.clone())));
                result.push_str(&stringify(value));
            }
            result.push('e');
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn stringify_integer_works() {
        assert_eq!(stringify(&make_node(32)), "i32e");
    }

    #[test]
    fn stringify_string_works() {
        assert_eq!(stringify(&make_node("test")), "4:test");
    }

    #[test]
    fn stringify_empty_list_works() {
        assert_eq!(stringify(&make_node(vec![] as Vec<Node>)), "le");
    }

    #[test]
    fn stringify_list_works() {
        let list = vec![make_node(32), make_node("test")];
        assert_eq!(stringify(&make_node(list)), "li32e4:teste");
    }

    #[test]
    fn stringify_empty_dictionary_works() {
        assert_eq!(stringify(&make_node(HashMap::new())), "de");
    }

    #[test]
    fn stringify_dictionary_works() {
        let mut dict = HashMap::new();
        dict.insert(String::from("key"), make_node(32));
        assert_eq!(stringify(&make_node(dict)), "d3:keyi32ee");
    }
}