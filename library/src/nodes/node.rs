#[cfg(not(feature = "std"))]
use alloc::collections::BTreeMap as HashMap;
#[cfg(feature = "std")]
use std::collections::HashMap;

#[cfg(not(feature = "std"))]
use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use core::fmt;

/// A node in the bencode data structure that can represent different types of values.
#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    /// Represents a 64-bit signed integer value
    Integer(i64),
    /// Represents a string value
    Str(String),
    /// Represents a list of other nodes
    List(Vec<Node>),
    /// Represents a dictionary/map of string keys to node values
    Dictionary(HashMap<String, Node>),
    /// Represents an empty or uninitialized node
    None,
}

impl Node {
    pub(crate) fn add_to_list(&mut self, p0: Node) -> Result<(), &'static str> {
        match self {
            Node::List(list) => {
                list.push(p0);
                Ok(())
            }
            _ => Err("Cannot add to non-list node"),
        }
    }

    pub(crate) fn add_to_dictionary(&mut self, key: &str, p0: Node) -> Result<(), &'static str> {
        match self {
            Node::Dictionary(dict) => {
                let _ = dict.insert(key.to_string(), p0);
                Ok(())
            }
            _ => Err("Cannot add to non-dictionary node"),
        }
    }

    /// Returns true if the node is an Integer variant
    pub fn is_integer(&self) -> bool {
        matches!(self, Node::Integer(_))
    }

    /// Returns true if the node is a String variant
    pub fn is_string(&self) -> bool {
        matches!(self, Node::Str(_))
    }

    /// Returns true if the node is a List variant
    pub fn is_list(&self) -> bool {
        matches!(self, Node::List(_))
    }

    /// Returns true if the node is a Dictionary variant
    pub fn is_dictionary(&self) -> bool {
        matches!(self, Node::Dictionary(_))
    }

    /// Returns true if the node is a None variant
    pub fn is_none(&self) -> bool {
        matches!(self, Node::None)
    }

    /// Returns a reference to the inner integer value if this is an Integer node
    pub fn as_integer(&self) -> Option<&i64> {
        match self {
            Node::Integer(i) => Some(i),
            _ => None,
        }
    }

    /// Returns a reference to the inner string value if this is a Str node
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Node::Str(s) => Some(s),
            _ => None,
        }
    }

    /// Returns a reference to the inner list if this is a List node
    pub fn as_list(&self) -> Option<&Vec<Node>> {
        match self {
            Node::List(list) => Some(list),
            _ => None,
        }
    }

    /// Returns a mutable reference to the inner list if this is a List node
    pub fn as_list_mut(&mut self) -> Option<&mut Vec<Node>> {
        match self {
            Node::List(list) => Some(list),
            _ => None,
        }
    }

    /// Returns a reference to the inner dictionary if this is a Dictionary node
    pub fn as_dictionary(&self) -> Option<&HashMap<String, Node>> {
        match self {
            Node::Dictionary(dict) => Some(dict),
            _ => None,
        }
    }

    /// Returns a mutable reference to the inner dictionary if this is a Dictionary node
    pub fn as_dictionary_mut(&mut self) -> Option<&mut HashMap<String, Node>> {
        match self {
            Node::Dictionary(dict) => Some(dict),
            _ => None,
        }
    }

    /// Gets a value from a Dictionary node by key
    pub fn get(&self, key: &str) -> Option<&Node> {
        match self {
            Node::Dictionary(dict) => dict.get(key),
            _ => None,
        }
    }

    /// Gets a mutable value from a Dictionary node by key
    pub fn get_mut(&mut self, key: &str) -> Option<&mut Node> {
        match self {
            Node::Dictionary(dict) => dict.get_mut(key),
            _ => None,
        }
    }

    /// Returns the number of elements in a List or Dictionary, or 0 for other types
    pub fn len(&self) -> usize {
        match self {
            Node::List(list) => list.len(),
            Node::Dictionary(dict) => dict.len(),
            Node::Str(s) => s.len(),
            _ => 0,
        }
    }

    /// Returns true if a List or Dictionary is empty, or for other types
    pub fn is_empty(&self) -> bool {
        match self {
            Node::List(list) => list.is_empty(),
            Node::Dictionary(dict) => dict.is_empty(),
            Node::Str(s) => s.is_empty(),
            Node::None => true,
            _ => false,
        }
    }

    /// Returns the type name as a string
    pub fn type_name(&self) -> &'static str {
        match self {
            Node::Integer(_) => "integer",
            Node::Str(_) => "string",
            Node::List(_) => "list",
            Node::Dictionary(_) => "dictionary",
            Node::None => "none",
        }
    }
    
    // Validation helpers
    
    /// Get a required field from a dictionary, returning an error if not found
    pub fn get_required(&self, key: &str) -> Result<&Node, String> {
        self.get(key)
            .ok_or_else(|| format!("Missing required field: '{}'", key))
    }
    
    /// Get a required integer field from a dictionary
    pub fn get_int_required(&self, key: &str) -> Result<i64, String> {
        self.get_required(key)?
            .as_integer()
            .copied()
            .ok_or_else(|| format!("Field '{}' must be an integer", key))
    }
    
    /// Get a required string field from a dictionary
    pub fn get_string_required(&self, key: &str) -> Result<&str, String> {
        self.get_required(key)?
            .as_string()
            .ok_or_else(|| format!("Field '{}' must be a string", key))
    }
    
    /// Get a required list field from a dictionary
    pub fn get_list_required(&self, key: &str) -> Result<&Vec<Node>, String> {
        self.get_required(key)?
            .as_list()
            .ok_or_else(|| format!("Field '{}' must be a list", key))
    }
    
    /// Get a required dictionary field from a dictionary
    pub fn get_dict_required(&self, key: &str) -> Result<&HashMap<String, Node>, String> {
        self.get_required(key)?
            .as_dictionary()
            .ok_or_else(|| format!("Field '{}' must be a dictionary", key))
    }
    
    /// Get an optional integer field, returning None if not found or not an integer
    pub fn get_int_optional(&self, key: &str) -> Option<i64> {
        self.get(key).and_then(|n| n.as_integer()).copied()
    }
    
    /// Get an optional string field, returning None if not found or not a string
    pub fn get_string_optional(&self, key: &str) -> Option<&str> {
        self.get(key).and_then(|n| n.as_string())
    }
    
    /// Get an optional list field, returning None if not found or not a list
    pub fn get_list_optional(&self, key: &str) -> Option<&Vec<Node>> {
        self.get(key).and_then(|n| n.as_list())
    }
    
    /// Get an optional dictionary field, returning None if not found or not a dictionary
    pub fn get_dict_optional(&self, key: &str) -> Option<&HashMap<String, Node>> {
        self.get(key).and_then(|n| n.as_dictionary())
    }
}

/// Converts a vector of values into a List node
impl<T: Into<Node>> From<Vec<T>> for Node {
    fn from(value: Vec<T>) -> Self {
        Node::List(value.into_iter().map(|x| x.into()).collect())
    }
}

/// Converts an integer into an Integer node
impl From<i64> for Node {
    fn from(value: i64) -> Self {
        Node::Integer(value)
    }
}

/// Converts a string slice into a Str node
impl From<&str> for Node {
    fn from(value: &str) -> Self {
        Node::Str(String::from(value))
    }
}

/// Converts a String into a Str node
impl From<String> for Node {
    fn from(value: String) -> Self {
        Node::Str(value)
    }
}

/// Converts a HashMap into a Dictionary node
impl From<HashMap<String, Node>> for Node {
    fn from(value: HashMap<String, Node>) -> Self {
        Node::Dictionary(value)
    }
}

// Allow creating a List node from a static array literal, e.g., Node::from([1, 2, 3])
impl<T, const N: usize> From<[T; N]> for Node
where
    T: Into<Node>,
{
    fn from(value: [T; N]) -> Self {
        Node::List(value.into_iter().map(|x| x.into()).collect())
    }
}

// Allow creating a Dictionary node from a static array of key-value pairs.
// e.g., Node::from([("a", 1), ("b", 2)])
impl<K, V, const N: usize> From<[(K, V); N]> for Node
where
    K: Into<String>,
    V: Into<Node>,
{
    fn from(value: [(K, V); N]) -> Self {
        let mut map: HashMap<String, Node> = HashMap::new();
        for (k, v) in value.into_iter() {
            map.insert(k.into(), v.into());
        }
        Node::Dictionary(map)
    }
}

/// Helper functions to create a Node from any value that can be converted into a Node
pub fn make_node<T>(value: T) -> Node
where
    T: Into<Node>,
{
    value.into()
}

/// Implements Display trait for Node to provide human-readable string representation
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Integer(i) => write!(f, "{}", i),
            Node::Str(s) => write!(f, "\"{}\"", s),
            Node::List(list) => {
                write!(f, "[")?;
                for (i, item) in list.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            Node::Dictionary(dict) => {
                write!(f, "{{")?;
                let mut items: Vec<_> = dict.iter().collect();
                items.sort_by_key(|(k, _)| *k);
                for (i, (key, value)) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "\"{}\": {}", key, value)?;
                }
                write!(f, "}}")
            }
            Node::None => write!(f, "null"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Node, make_node};
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
        let node = make_node("test");
        match node {
            Node::Str(string) => {
                assert_eq!(string.as_str(), "test");
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
                for item in list {
                    match item {
                        Node::Integer(_) => (),
                        _ => assert_eq!(false, true),
                    }
                }
            }
            _ => assert_eq!(false, true),
        }
    }

    #[test]
    fn mixed_array_literal_to_list_node_works() {
        let node = Node::from([
            Node::Integer(1),
            Node::Str("x".to_string()),
            Node::Integer(3),
        ]);
        match node {
            Node::List(list) => {
                assert_eq!(list.len(), 3);
            }
            _ => assert_eq!(false, true),
        }
    }

    #[test]
    fn array_literal_to_dictionary_node_works() {
        let node = make_node([("a", 1), ("b", 2)]);
        match node {
            Node::Dictionary(map) => {
                assert_eq!(map.len(), 2);
                match map.get("b").unwrap() {
                    Node::Integer(i) => assert_eq!(*i, 2),
                    _ => assert_eq!(false, true),
                }
            }
            _ => assert_eq!(false, true),
        }
    }

    #[test]
    fn none_node_works() {
        let node = Node::None;
        match node {
            Node::None => (),
            _ => assert_eq!(false, true),
        }
    }

    #[test]
    fn mixed_dictionary_from_array_works() {
        let node = Node::from([
            ("int", Node::Integer(1)),
            ("str", Node::Str("test".to_string())),
            ("list", Node::List(Vec::<Node>::new())),
        ]);
        match node {
            Node::Dictionary(map) => {
                assert_eq!(map.len(), 3);
                assert!(matches!(map.get("int"), Some(Node::Integer(1))));
                assert!(matches!(map.get("str"), Some(Node::Str(_))));
                assert!(matches!(map.get("list"), Some(Node::List(_))));
            }
            _ => assert_eq!(false, true),
        }
    }

    #[test]
    fn empty_array_to_list_works() {
        let node = Node::from([] as [i64; 0]);
        match node {
            Node::List(list) => assert_eq!(list.len(), 0),
            _ => assert_eq!(false, true),
        }
    }

    #[test]
    fn empty_array_to_dictionary_works() {
        let node = Node::from([] as [(String, Node); 0]);
        match node {
            Node::Dictionary(map) => assert_eq!(map.len(), 0),
            _ => assert_eq!(false, true),
        }
    }

    #[test]
    fn test_add_to_list() {
        let mut node = Node::List(Vec::new());
        let _ = node.add_to_list(Node::Integer(42));
        match node {
            Node::List(list) => assert_eq!(list[0], Node::Integer(42)),
            _ => assert_eq!(false, true),
        }
    }

    #[test]
    fn test_add_to_list_error() {
        let mut node = Node::Integer(0);
        let result = node.add_to_list(Node::Integer(42));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Cannot add to non-list node");
    }

    #[test]
    fn test_add_to_dictionary() {
        let mut node = Node::Dictionary(HashMap::new());
        assert!(node.add_to_dictionary("test", Node::Integer(42)).is_ok());
        match node {
            Node::Dictionary(dict) => assert_eq!(dict["test"], Node::Integer(42)),
            _ => panic!("Expected dictionary"),
        }
    }

    #[test]
    fn test_add_to_dictionary_error() {
        let mut node = Node::Integer(0);
        let result = node.add_to_dictionary("test", Node::Integer(42));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Cannot add to non-dictionary node");
    }

    #[test]
    fn test_from_i64() {
        let value: i64 = 42;
        let node = Node::from(value);
        assert_eq!(node, Node::Integer(42));
    }

    #[test]
    fn test_clone_node() {
        let original = Node::Integer(42);
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    // New feature tests
    #[test]
    fn test_is_integer() {
        assert!(Node::Integer(42).is_integer());
        assert!(!Node::Str("test".to_string()).is_integer());
    }

    #[test]
    fn test_is_string() {
        assert!(Node::Str("test".to_string()).is_string());
        assert!(!Node::Integer(42).is_string());
    }

    #[test]
    fn test_is_list() {
        assert!(Node::List(vec![]).is_list());
        assert!(!Node::Integer(42).is_list());
    }

    #[test]
    fn test_is_dictionary() {
        assert!(Node::Dictionary(HashMap::new()).is_dictionary());
        assert!(!Node::Integer(42).is_dictionary());
    }

    #[test]
    fn test_is_none() {
        assert!(Node::None.is_none());
        assert!(!Node::Integer(42).is_none());
    }

    #[test]
    fn test_as_integer() {
        let node = Node::Integer(42);
        assert_eq!(node.as_integer(), Some(&42));
        assert_eq!(Node::Str("test".to_string()).as_integer(), None);
    }

    #[test]
    fn test_as_string() {
        let node = Node::Str("test".to_string());
        assert_eq!(node.as_string(), Some("test"));
        assert_eq!(Node::Integer(42).as_string(), None);
    }

    #[test]
    fn test_as_list() {
        let list = vec![Node::Integer(1), Node::Integer(2)];
        let node = Node::List(list.clone());
        assert_eq!(node.as_list(), Some(&list));
        assert_eq!(Node::Integer(42).as_list(), None);
    }

    #[test]
    fn test_as_list_mut() {
        let mut node = Node::List(vec![Node::Integer(1)]);
        if let Some(list) = node.as_list_mut() {
            list.push(Node::Integer(2));
        }
        assert_eq!(node.as_list().unwrap().len(), 2);
    }

    #[test]
    fn test_as_dictionary() {
        let mut dict = HashMap::new();
        dict.insert("key".to_string(), Node::Integer(42));
        let node = Node::Dictionary(dict.clone());
        assert_eq!(node.as_dictionary(), Some(&dict));
        assert_eq!(Node::Integer(42).as_dictionary(), None);
    }

    #[test]
    fn test_as_dictionary_mut() {
        let mut node = Node::Dictionary(HashMap::new());
        if let Some(dict) = node.as_dictionary_mut() {
            dict.insert("key".to_string(), Node::Integer(42));
        }
        assert_eq!(node.as_dictionary().unwrap().len(), 1);
    }

    #[test]
    fn test_get() {
        let mut dict = HashMap::new();
        dict.insert("key".to_string(), Node::Integer(42));
        let node = Node::Dictionary(dict);
        assert_eq!(node.get("key"), Some(&Node::Integer(42)));
        assert_eq!(node.get("missing"), None);
        assert_eq!(Node::Integer(42).get("key"), None);
    }

    #[test]
    fn test_get_mut() {
        let mut dict = HashMap::new();
        dict.insert("key".to_string(), Node::Integer(42));
        let mut node = Node::Dictionary(dict);

        if let Some(value) = node.get_mut("key") {
            *value = Node::Integer(100);
        }
        assert_eq!(node.get("key"), Some(&Node::Integer(100)));
    }

    #[test]
    fn test_len() {
        assert_eq!(
            Node::List(vec![Node::Integer(1), Node::Integer(2)]).len(),
            2
        );

        let mut dict = HashMap::new();
        dict.insert("a".to_string(), Node::Integer(1));
        assert_eq!(Node::Dictionary(dict).len(), 1);

        assert_eq!(Node::Str("hello".to_string()).len(), 5);
        assert_eq!(Node::Integer(42).len(), 0);
        assert_eq!(Node::None.len(), 0);
    }

    #[test]
    fn test_is_empty() {
        assert!(Node::List(vec![]).is_empty());
        assert!(!Node::List(vec![Node::Integer(1)]).is_empty());
        assert!(Node::Dictionary(HashMap::new()).is_empty());
        assert!(Node::Str("".to_string()).is_empty());
        assert!(!Node::Str("test".to_string()).is_empty());
        assert!(Node::None.is_empty());
        assert!(!Node::Integer(42).is_empty());
    }

    #[test]
    fn test_type_name() {
        assert_eq!(Node::Integer(42).type_name(), "integer");
        assert_eq!(Node::Str("test".to_string()).type_name(), "string");
        assert_eq!(Node::List(vec![]).type_name(), "list");
        assert_eq!(Node::Dictionary(HashMap::new()).type_name(), "dictionary");
        assert_eq!(Node::None.type_name(), "none");
    }

    #[test]
    fn test_display_integer() {
        let node = Node::Integer(42);
        assert_eq!(format!("{}", node), "42");
    }

    #[test]
    fn test_display_string() {
        let node = Node::Str("hello".to_string());
        assert_eq!(format!("{}", node), "\"hello\"");
    }

    #[test]
    fn test_display_list() {
        let node = Node::List(vec![Node::Integer(1), Node::Integer(2), Node::Integer(3)]);
        assert_eq!(format!("{}", node), "[1, 2, 3]");
    }

    #[test]
    fn test_display_dictionary() {
        let mut dict = HashMap::new();
        dict.insert("a".to_string(), Node::Integer(1));
        dict.insert("b".to_string(), Node::Integer(2));
        let node = Node::Dictionary(dict);
        assert_eq!(format!("{}", node), "{\"a\": 1, \"b\": 2}");
    }

    #[test]
    fn test_display_none() {
        let node = Node::None;
        assert_eq!(format!("{}", node), "null");
    }

    #[test]
    fn test_get_required() {
        let mut dict = HashMap::new();
        dict.insert("key".to_string(), Node::Integer(42));
        let node = Node::Dictionary(dict);
        
        assert!(node.get_required("key").is_ok());
        assert!(node.get_required("missing").is_err());
    }

    #[test]
    fn test_get_int_required() {
        let mut dict = HashMap::new();
        dict.insert("age".to_string(), Node::Integer(25));
        dict.insert("name".to_string(), Node::Str("John".to_string()));
        let node = Node::Dictionary(dict);
        
        assert_eq!(node.get_int_required("age").unwrap(), 25);
        assert!(node.get_int_required("name").is_err());
        assert!(node.get_int_required("missing").is_err());
    }

    #[test]
    fn test_get_string_required() {
        let mut dict = HashMap::new();
        dict.insert("name".to_string(), Node::Str("John".to_string()));
        dict.insert("age".to_string(), Node::Integer(25));
        let node = Node::Dictionary(dict);
        
        assert_eq!(node.get_string_required("name").unwrap(), "John");
        assert!(node.get_string_required("age").is_err());
        assert!(node.get_string_required("missing").is_err());
    }

    #[test]
    fn test_get_list_required() {
        let mut dict = HashMap::new();
        dict.insert("items".to_string(), Node::List(vec![Node::Integer(1), Node::Integer(2)]));
        dict.insert("name".to_string(), Node::Str("test".to_string()));
        let node = Node::Dictionary(dict);
        
        assert_eq!(node.get_list_required("items").unwrap().len(), 2);
        assert!(node.get_list_required("name").is_err());
        assert!(node.get_list_required("missing").is_err());
    }

    #[test]
    fn test_get_dict_required() {
        let mut inner = HashMap::new();
        inner.insert("x".to_string(), Node::Integer(1));
        
        let mut dict = HashMap::new();
        dict.insert("nested".to_string(), Node::Dictionary(inner));
        dict.insert("value".to_string(), Node::Integer(42));
        let node = Node::Dictionary(dict);
        
        assert_eq!(node.get_dict_required("nested").unwrap().len(), 1);
        assert!(node.get_dict_required("value").is_err());
        assert!(node.get_dict_required("missing").is_err());
    }

    #[test]
    fn test_get_optional_methods() {
        let mut dict = HashMap::new();
        dict.insert("age".to_string(), Node::Integer(25));
        dict.insert("name".to_string(), Node::Str("John".to_string()));
        dict.insert("items".to_string(), Node::List(vec![Node::Integer(1)]));
        let node = Node::Dictionary(dict);
        
        assert_eq!(node.get_int_optional("age"), Some(25));
        assert_eq!(node.get_int_optional("name"), None);
        assert_eq!(node.get_int_optional("missing"), None);
        
        assert_eq!(node.get_string_optional("name"), Some("John"));
        assert_eq!(node.get_string_optional("age"), None);
        assert_eq!(node.get_string_optional("missing"), None);
        
        assert_eq!(node.get_list_optional("items").map(|l| l.len()), Some(1));
        assert_eq!(node.get_list_optional("age"), None);
        assert_eq!(node.get_list_optional("missing"), None);
    }

    #[test]
    fn test_display_nested() {
        let mut inner_dict = HashMap::new();
        inner_dict.insert("x".to_string(), Node::Integer(10));

        let list = vec![Node::Integer(1), Node::Dictionary(inner_dict)];
        let node = Node::List(list);

        assert_eq!(format!("{}", node), "[1, {\"x\": 10}]");
    }
}
