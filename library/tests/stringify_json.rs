//! Integration tests for the bencode JSON stringify functionality.
//! These tests validate the JSON stringify behavior from an external perspective,
//! testing the public API against various node structures.

use bencode_lib::BufferDestination;
use bencode_lib::io::traits::IDestination;
use bencode_lib::nodes::node::Node;
use bencode_lib::stringify::json::stringify;
use std::collections::HashMap;

#[test]
fn test_stringify_dictionary_sorting() {
    let mut destination = BufferDestination::new();
    let mut dict = HashMap::new();
    dict.insert("z".to_string(), Node::Integer(1));
    dict.insert("a".to_string(), Node::Integer(2));
    dict.insert("m".to_string(), Node::Integer(3));
    stringify(&Node::Dictionary(dict), &mut destination).unwrap();
    assert_eq!(destination.to_string(), "{\"a\":2,\"m\":3,\"z\":1}");
}

#[test]
fn test_stringify_complex_nested_structure() {
    let mut destination = BufferDestination::new();
    let mut inner_dict1 = HashMap::new();
    inner_dict1.insert("x".to_string(), Node::Integer(1));
    let mut inner_dict2 = HashMap::new();
    inner_dict2.insert(
        "y".to_string(),
        Node::List(vec![
            Node::Str("a".to_string()),
            Node::Dictionary(inner_dict1),
            Node::Integer(42),
        ]),
    );
    stringify(&Node::Dictionary(inner_dict2), &mut destination).unwrap();
    assert_eq!(destination.to_string(), "{\"y\":[\"a\",{\"x\":1},42]}");
}

#[test]
fn test_stringify_nested_structures() {
    let mut destination = BufferDestination::new();
    let inner_list = Node::List(vec![Node::Integer(1), Node::Integer(2)]);
    let mut inner_dict = HashMap::new();
    inner_dict.insert("key".to_string(), Node::Str("value".to_string()));
    let dict = Node::Dictionary(inner_dict);

    stringify(&Node::List(vec![inner_list, dict]), &mut destination).unwrap();
    assert_eq!(destination.to_string(), "[[1,2],{\"key\":\"value\"}]");
}

#[test]
fn test_stringify_list_with_none() {
    let mut destination = BufferDestination::new();
    let list = vec![Node::Integer(1), Node::None, Node::Integer(2)];
    stringify(&Node::List(list), &mut destination).unwrap();
    assert_eq!(destination.to_string(), "[1,null,2]");
}

#[test]
fn test_stringify_dictionary_with_list() {
    let mut destination = BufferDestination::new();
    let mut dict = HashMap::new();
    dict.insert(
        "list".to_string(),
        Node::List(vec![Node::Integer(1), Node::Integer(2)]),
    );
    stringify(&Node::Dictionary(dict), &mut destination).unwrap();
    assert_eq!(destination.to_string(), "{\"list\":[1,2]}");
}

#[test]
fn test_stringify_empty_structures() {
    let mut destination = BufferDestination::new();
    stringify(&Node::List(vec![]), &mut destination).unwrap();
    assert_eq!(destination.to_string(), "[]");
    destination.clear();
    stringify(&Node::Dictionary(HashMap::new()), &mut destination).unwrap();
    assert_eq!(destination.to_string(), "{}");
}

#[test]
fn test_stringify_deeply_nested_dictionary() {
    let mut destination = BufferDestination::new();
    let mut level3 = HashMap::new();
    level3.insert("level3".to_string(), Node::Integer(3));

    let mut level2 = HashMap::new();
    level2.insert("level2".to_string(), Node::Dictionary(level3));

    let mut level1 = HashMap::new();
    level1.insert("level1".to_string(), Node::Dictionary(level2));

    stringify(&Node::Dictionary(level1), &mut destination).unwrap();
    assert_eq!(
        destination.to_string(),
        "{\"level1\":{\"level2\":{\"level3\":3}}}"
    );
}

#[test]
fn test_stringify_mixed_types_in_list() {
    let mut destination = BufferDestination::new();
    let mut dict = HashMap::new();
    dict.insert("key".to_string(), Node::Str("value".to_string()));

    let list = vec![
        Node::Integer(42),
        Node::Str("text".to_string()),
        Node::List(vec![Node::Integer(1), Node::Integer(2)]),
        Node::Dictionary(dict),
        Node::None,
    ];

    stringify(&Node::List(list), &mut destination).unwrap();
    assert_eq!(
        destination.to_string(),
        "[42,\"text\",[1,2],{\"key\":\"value\"},null]"
    );
}

#[test]
fn test_stringify_multiple_none_values() {
    let mut destination = BufferDestination::new();
    let list = vec![Node::None, Node::None, Node::None];
    stringify(&Node::List(list), &mut destination).unwrap();
    assert_eq!(destination.to_string(), "[null,null,null]");
}

#[test]
fn test_stringify_dictionary_with_empty_string_key() {
    let mut destination = BufferDestination::new();
    let mut dict = HashMap::new();
    dict.insert("".to_string(), Node::Integer(1));
    dict.insert("key".to_string(), Node::Integer(2));
    stringify(&Node::Dictionary(dict), &mut destination).unwrap();
    assert_eq!(destination.to_string(), "{\"\":1,\"key\":2}");
}

#[test]
fn test_stringify_negative_integers() {
    let mut destination = BufferDestination::new();
    let list = vec![
        Node::Integer(-1),
        Node::Integer(-100),
        Node::Integer(0),
        Node::Integer(100),
    ];
    stringify(&Node::List(list), &mut destination).unwrap();
    assert_eq!(destination.to_string(), "[-1,-100,0,100]");
}

#[test]
fn test_stringify_string_with_escapes() {
    let mut destination = BufferDestination::new();
    stringify(
        &Node::Str("hello\"world\\test".to_string()),
        &mut destination,
    )
    .unwrap();
    assert_eq!(destination.to_string(), "\"hello\\\"world\\\\test\"");
}

#[test]
fn test_stringify_string_with_newline_tab() {
    let mut destination = BufferDestination::new();
    stringify(
        &Node::Str("line1\nline2\ttab".to_string()),
        &mut destination,
    )
    .unwrap();
    assert_eq!(destination.to_string(), "\"line1\\u000aline2\\u0009tab\"");
}
