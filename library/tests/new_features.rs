//! Integration tests for new features added to the bencode library

use bencode_lib::{Node, parse_bytes, parse_str, stringify_to_string, stringify_to_bytes, make_node};
use std::collections::HashMap;

#[test]
fn test_parse_bytes_convenience() {
    let data = b"i42e";
    let result = parse_bytes(data);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Node::Integer(42));
}

#[test]
fn test_parse_str_convenience() {
    let data = "4:test";
    let result = parse_str(data);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Node::Str("test".to_string()));
}

#[test]
fn test_stringify_to_string_convenience() {
    let node = Node::Integer(42);
    let result = stringify_to_string(&node);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "i42e");
}

#[test]
fn test_stringify_to_bytes_convenience() {
    let node = Node::Integer(42);
    let result = stringify_to_bytes(&node);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), b"i42e");
}

#[test]
fn test_node_type_checking() {
    let int_node = Node::Integer(42);
    assert!(int_node.is_integer());
    assert!(!int_node.is_string());
    assert!(!int_node.is_list());
    assert!(!int_node.is_dictionary());
    assert!(!int_node.is_none());
    
    let str_node = Node::Str("test".to_string());
    assert!(!str_node.is_integer());
    assert!(str_node.is_string());
    
    let list_node = Node::List(vec![]);
    assert!(list_node.is_list());
    
    let dict_node = Node::Dictionary(HashMap::new());
    assert!(dict_node.is_dictionary());
    
    let none_node = Node::None;
    assert!(none_node.is_none());
}

#[test]
fn test_node_as_methods() {
    let int_node = Node::Integer(42);
    assert_eq!(int_node.as_integer(), Some(&42));
    assert_eq!(int_node.as_string(), None);
    
    let str_node = Node::Str("test".to_string());
    assert_eq!(str_node.as_string(), Some("test"));
    assert_eq!(str_node.as_integer(), None);
    
    let list_node = Node::List(vec![Node::Integer(1), Node::Integer(2)]);
    assert_eq!(list_node.as_list().unwrap().len(), 2);
    
    let mut dict = HashMap::new();
    dict.insert("key".to_string(), Node::Integer(100));
    let dict_node = Node::Dictionary(dict);
    assert_eq!(dict_node.as_dictionary().unwrap().len(), 1);
}

#[test]
fn test_node_get_methods() {
    let mut dict = HashMap::new();
    dict.insert("a".to_string(), Node::Integer(1));
    dict.insert("b".to_string(), Node::Integer(2));
    let node = Node::Dictionary(dict);
    
    assert_eq!(node.get("a"), Some(&Node::Integer(1)));
    assert_eq!(node.get("b"), Some(&Node::Integer(2)));
    assert_eq!(node.get("c"), None);
}

#[test]
fn test_node_get_mut() {
    let mut dict = HashMap::new();
    dict.insert("key".to_string(), Node::Integer(42));
    let mut node = Node::Dictionary(dict);
    
    if let Some(value) = node.get_mut("key") {
        *value = Node::Integer(100);
    }
    
    assert_eq!(node.get("key"), Some(&Node::Integer(100)));
}

#[test]
fn test_node_len_and_is_empty() {
    let empty_list = Node::List(vec![]);
    assert_eq!(empty_list.len(), 0);
    assert!(empty_list.is_empty());
    
    let list = Node::List(vec![Node::Integer(1), Node::Integer(2), Node::Integer(3)]);
    assert_eq!(list.len(), 3);
    assert!(!list.is_empty());
    
    let empty_dict = Node::Dictionary(HashMap::new());
    assert_eq!(empty_dict.len(), 0);
    assert!(empty_dict.is_empty());
    
    let mut dict = HashMap::new();
    dict.insert("a".to_string(), Node::Integer(1));
    dict.insert("b".to_string(), Node::Integer(2));
    let dict_node = Node::Dictionary(dict);
    assert_eq!(dict_node.len(), 2);
    assert!(!dict_node.is_empty());
    
    let str_node = Node::Str("hello".to_string());
    assert_eq!(str_node.len(), 5);
    assert!(!str_node.is_empty());
    
    let empty_str = Node::Str("".to_string());
    assert_eq!(empty_str.len(), 0);
    assert!(empty_str.is_empty());
}

#[test]
fn test_node_type_name() {
    assert_eq!(Node::Integer(42).type_name(), "integer");
    assert_eq!(Node::Str("test".to_string()).type_name(), "string");
    assert_eq!(Node::List(vec![]).type_name(), "list");
    assert_eq!(Node::Dictionary(HashMap::new()).type_name(), "dictionary");
    assert_eq!(Node::None.type_name(), "none");
}

#[test]
fn test_node_display() {
    let int_node = Node::Integer(42);
    assert_eq!(format!("{}", int_node), "42");
    
    let str_node = Node::Str("hello".to_string());
    assert_eq!(format!("{}", str_node), "\"hello\"");
    
    let list_node = Node::List(vec![Node::Integer(1), Node::Integer(2)]);
    assert_eq!(format!("{}", list_node), "[1, 2]");
    
    let mut dict = HashMap::new();
    dict.insert("x".to_string(), Node::Integer(10));
    let dict_node = Node::Dictionary(dict);
    assert_eq!(format!("{}", dict_node), "{\"x\": 10}");
    
    assert_eq!(format!("{}", Node::None), "null");
}

#[test]
fn test_as_list_mut() {
    let mut node = Node::List(vec![Node::Integer(1)]);
    
    if let Some(list) = node.as_list_mut() {
        list.push(Node::Integer(2));
        list.push(Node::Integer(3));
    }
    
    assert_eq!(node.len(), 3);
    assert_eq!(node.as_list().unwrap()[2], Node::Integer(3));
}

#[test]
fn test_as_dictionary_mut() {
    let mut node = Node::Dictionary(HashMap::new());
    
    if let Some(dict) = node.as_dictionary_mut() {
        dict.insert("a".to_string(), Node::Integer(1));
        dict.insert("b".to_string(), Node::Integer(2));
    }
    
    assert_eq!(node.len(), 2);
    assert_eq!(node.get("a"), Some(&Node::Integer(1)));
}

#[test]
fn test_complex_nested_display() {
    let mut inner_dict = HashMap::new();
    inner_dict.insert("nested".to_string(), Node::Integer(42));
    
    let complex = Node::List(vec![
        Node::Integer(1),
        Node::Str("test".to_string()),
        Node::Dictionary(inner_dict),
    ]);
    
    let display = format!("{}", complex);
    assert!(display.contains("1"));
    assert!(display.contains("\"test\""));
    assert!(display.contains("\"nested\""));
    assert!(display.contains("42"));
}

#[test]
fn test_round_trip_with_convenience_methods() {
    // Create a complex structure
    let mut dict = HashMap::new();
    dict.insert("number".to_string(), Node::Integer(42));
    dict.insert("text".to_string(), Node::Str("hello".to_string()));
    dict.insert("list".to_string(), Node::List(vec![
        Node::Integer(1),
        Node::Integer(2),
        Node::Integer(3),
    ]));
    let original = Node::Dictionary(dict);
    
    // Stringify to string
    let bencode_str = stringify_to_string(&original).unwrap();
    
    // Parse back
    let parsed = parse_str(&bencode_str).unwrap();
    
    // Verify they match
    assert_eq!(original, parsed);
}

#[test]
fn test_make_node_with_new_methods() {
    let node = make_node(42);
    assert!(node.is_integer());
    assert_eq!(node.as_integer(), Some(&42));
    
    let node = make_node("test");
    assert!(node.is_string());
    assert_eq!(node.as_string(), Some("test"));
}
