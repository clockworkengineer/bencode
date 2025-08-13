use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Node {
    Integer(i64),
    Str(String),
    List(Vec<Node>),
    Dictionary(HashMap<String, Node>),
}

impl<T: Into<Node>> From<Vec<T>> for Node {
    fn from(value: Vec<T>) -> Self {
        Node::List(value.into_iter().map(|x| x.into()).collect())
    }
}

impl From<i64> for Node {
    fn from(value: i64) -> Self {
        Node::Integer(value)
    }
}

impl From<&str> for Node {
    fn from(value: &str) -> Self {
        Node::Str(String::from(value))
    }
}

impl From<String> for Node {
    fn from(value: String) -> Self {
        Node::Str(value)
    }
}

impl From<HashMap<String, Node>> for Node {
    fn from(value: HashMap<String, Node>) -> Self {
        Node::Dictionary(value)
    }
}

// Allow creating a List node from a static array literal, e.g. Node::from([1, 2, 3])
impl<T, const N: usize> From<[T; N]> for Node
where
    T: Into<Node>,
{
    fn from(value: [T; N]) -> Self {
        Node::List(value.into_iter().map(|x| x.into()).collect())
    }
}

// Allow creating a Dictionary node from a static array of key-value pairs,
// e.g. Node::from([("a", 1), ("b", 2)])
impl<K, V, const N: usize> From<[(K, V); N]> for Node
where
    K: Into<String>,
    V: Into<Node>,
{
    fn from(value: [(K, V); N]) -> Self {
        let mut map: HashMap<String, Node> = HashMap::with_capacity(N);
        for (k, v) in value.into_iter() {
            map.insert(k.into(), v.into());
        }
        Node::Dictionary(map)
    }
}

pub fn make_node<T>(value: T) -> Node
where
    T: Into<Node>,
{
    value.into()
}

#[cfg(test)]
mod tests {
    use super::{make_node, Node};
    use std::collections::HashMap;

    #[test]
    fn create_integer_works() {
        let variant = Node::Integer(32);
        match variant {
            Node::Integer(integer) => {
                assert_eq!(integer, 32);
            }
            _ => {
                assert_eq!(false, true);
            }
        }
    }
    #[test]
    fn create_string_works() {
        let variant = Node::Str(String::from("test"));
        match variant {
            Node::Str(string) => {
                assert_eq!(string.as_str(), "test");
            }
            _ => {
                assert_eq!(false, true);
            }
        }
    }
    #[test]
    fn create_list_works() {
        let variant = Node::List(Vec::<Node>::new());
        match variant {
            Node::List(list) => {
                assert_eq!(list.is_empty(), true);
            }
            _ => {
                assert_eq!(false, true);
            }
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
                    _ => {
                        assert_eq!(false, true);
                    }
                }
            }
            _ => {
                assert_eq!(false, true);
            }
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
                    _ => {
                        assert_eq!(false, true);
                    }
                }
            }
            _ => {
                assert_eq!(false, true);
            }
        }
    }
    #[test]
    fn create_dictionary_works() {
        let variant = Node::Dictionary(HashMap::new());
        match variant {
            Node::Dictionary(dictionary) => {
                assert_eq!(dictionary.is_empty(), true);
            }
            _ => {
                assert_eq!(false, true);
            }
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
                    _ => {
                        assert_eq!(false, true);
                    }
                }
            }
            _ => {
                assert_eq!(false, true);
            }
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
                    _ => {
                        assert_eq!(false, true);
                    }
                }
            }
            _ => {
                assert_eq!(false, true);
            }
        }
    }
    #[test]
    fn make_an_integer_node_works() {
        let node = make_node(32);
        match node {
            Node::Integer(integer) => {
                assert_eq!(integer, 32);
            }
            _ => {
                assert_eq!(false, true);
            }
        }
    }
    #[test]
    fn make_a_string_node_works() {
        let node = make_node("i32e");
        match node {
            Node::Str(string) => {
                assert_eq!(string.as_str(), "i32e");
            }
            _ => {
                assert_eq!(false, true);
            }
        }
    }
    #[test]
    fn make_a_list_node_works() {
        let node = make_node(Vec::<Node>::new());
        match node {
            Node::List(list) => {
                assert_eq!(list.is_empty(), true);
            }
            _ => {
                assert_eq!(false, true);
            }
        }
    }
    #[test]
    fn make_a_dictionary_node_works() {
        let node = make_node(HashMap::<String, Node>::new());
        match node {
            Node::Dictionary(dictionary) => {
                assert_eq!(dictionary.is_empty(), true);
            }
            _ => {
                assert_eq!(false, true);
            }
        }
    }

    // New tests for static initializer lists
    #[test]
    fn array_literal_to_list_node_works() {
        let node = make_node([1, 2, 3]);
        match node {
            Node::List(list) => {
                assert_eq!(list.len(), 3);
            }
            _ => panic!("Expected List"),
        }
    }

    #[test]
    fn mixed_array_literal_to_list_node_works() {
        let node = Node::from([Node::Integer(1), Node::Str("x".to_string()), Node::Integer(3)]);
        match node {
            Node::List(list) => {
                assert_eq!(list.len(), 3);
            }
            _ => panic!("Expected List"),
        }
    }

    #[test]
    fn kv_array_literal_to_dictionary_node_works() {
        let node = make_node([("a", 1), ("b", 2)]);
        match node {
            Node::Dictionary(map) => {
                assert_eq!(map.len(), 2);
                match map.get("b").unwrap() {
                    Node::Integer(i) => assert_eq!(*i, 2),
                    _ => panic!("Expected Integer for key 'b'"),
                }
            }
            _ => panic!("Expected Dictionary"),
        }
    }
}
