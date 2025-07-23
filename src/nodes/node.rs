
use std::collections::HashMap;

pub enum Node {
    Integer(u32),
    Str(String),
    List(Vec<Node>),
    Dictionary(HashMap<String, Node>),
}

#[cfg(test)]
mod tests {
    use super::Node;
    use std::collections::HashMap;
    #[test]
    fn create_integer_works() {
        let variant = Node::Integer(32);
        match variant {
            Node::Integer(integer) => {
                assert_eq!(integer, 32);
            }
            _ => {}
        }
    }
    #[test]
    fn create_string_works() {
        let variant = Node::Str(String::from("test"));
        match variant {
            Node::Str(string) => {
                assert_eq!(string.as_str(), "test");
            }
            _ => {}
        }
    }
    #[test]
    fn create_list_works() {
        let variant = Node::List(Vec::<Node>::new());
        match variant {
            Node::List(list) => {
                assert_eq!(list.is_empty(), true);
            }
            _ => {}
        }
    }
    #[test]
    fn create_dictionary_works() {
        let variant = Node::Dictionary(HashMap::new());
        match variant {
            Node::Dictionary(list) => {
                assert_eq!(list.is_empty(), true);
            }
            _ => {}
        }
    }
}
