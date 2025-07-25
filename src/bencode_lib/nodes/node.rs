
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
            _ => {assert_eq!(false, true);}
        }
    }
    #[test]
    fn create_string_works() {
        let variant = Node::Str(String::from("test"));
        match variant {
            Node::Str(string) => {
                assert_eq!(string.as_str(), "test");
            }
            _ => {assert_eq!(false, true);}
        }
    }
    #[test]
    fn create_list_works() {
        let variant = Node::List(Vec::<Node>::new());
        match variant {
            Node::List(list) => {
                assert_eq!(list.is_empty(), true);
            }
            _ => {assert_eq!(false, true);}
        }
    }
    #[test]
    fn push_to_list_works() {
        let variant = Node::List(Vec::<Node>::new());
        match variant {
            Node::List(mut list) => {
                list.push(Node::Integer(32));
                assert_eq!(list.len(), 1);
                match list[0] {
                    Node::Integer(integer) => {
                        assert_eq!(integer, 32);
                    }
                    _ => {assert_eq!(false, true);}
                }
            }
            _ => {assert_eq!(false, true);}
        }
    }
    #[test]
    fn push_multiple_to_list_works() {
        let variant = Node::List(Vec::<Node>::new());
        match variant {
            Node::List(mut list) => {
                list.push(Node::Integer(32));
                list.push(Node::Integer(33));
                list.push(Node::Integer(34));
                list.push(Node::Integer(35));
                list.push(Node::Integer(36));
                assert_eq!(list.len(), 5);
                match list[4] {
                    Node::Integer(integer) => {
                        assert_eq!(integer, 36);
                    }
                    _ => {assert_eq!(false, true);}
                }
            }
            _ => {assert_eq!(false, true);}
        }
    }
    #[test]
    fn create_dictionary_works() {
        let variant = Node::Dictionary(HashMap::new());
        match variant {
            Node::Dictionary(dictionary) => {
                assert_eq!(dictionary.is_empty(), true);
            }
            _ => {assert_eq!(false, true);}
        }
    }
    #[test]
    fn add_to_dictionary_works() {
        let variant = Node::Dictionary(HashMap::new());
        match variant {
            Node::Dictionary(mut dictionary) => {
                dictionary.insert(String::from("test"), Node::Integer(32));
                assert_eq!(dictionary.len(), 1);
                match dictionary["test"] {
                    Node::Integer(integer) => {
                        assert_eq!(integer, 32);
                    }
                    _ => {assert_eq!(false, true);}
                }
            }
            _ => {assert_eq!(false, true);}
        }
    }
    #[test]
    fn add_multiple_to_dictionary_works() {
        let variant = Node::Dictionary(HashMap::new());
        match variant {
            Node::Dictionary(mut dictionary) => {
                dictionary.insert(String::from("test1"), Node::Integer(32));
                dictionary.insert(String::from("test2"), Node::Integer(33));
                dictionary.insert(String::from("test3"), Node::Integer(34));
                dictionary.insert(String::from("test4"), Node::Integer(35));
                dictionary.insert(String::from("test5"), Node::Integer(36));
                assert_eq!(dictionary.len(), 5);
                match dictionary["test5"] {
                    Node::Integer(integer) => {
                        assert_eq!(integer, 36);
                    }
                    _ => {assert_eq!(false, true);}
                }
            }
            _ => {assert_eq!(false, true);}
        }
    }
}
