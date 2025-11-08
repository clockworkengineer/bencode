//! Integration tests for the bencode stringify functionality.
//! These tests validate the stringify behavior from an external perspective,
//! testing the public API against various node structures.

use bencode_lib::BufferDestination;
use bencode_lib::nodes::node::{Node, make_node};
use bencode_lib::stringify::default::stringify;
use std::collections::HashMap;

#[test]
fn test_stringify_complex_dictionary() {
    let mut destination = BufferDestination::new();
    let mut dict = HashMap::new();
    dict.insert(String::from("b"), make_node(1));
    dict.insert(String::from("a"), make_node(2));
    dict.insert(String::from("c"), make_node("test"));
    stringify(&make_node(dict), &mut destination).unwrap();
    assert_eq!(destination.to_string(), "d1:ai2e1:bi1e1:c4:teste");
}

#[test]
fn test_stringify_nested_dictionary() {
    let mut destination = BufferDestination::new();
    let mut inner_dict = HashMap::new();
    inner_dict.insert(String::from("key2"), make_node("value"));
    let mut outer_dict = HashMap::new();
    outer_dict.insert(String::from("key1"), make_node(inner_dict));
    stringify(&make_node(outer_dict), &mut destination).unwrap();
    assert_eq!(destination.to_string(), "d4:key1d4:key25:valueee");
}

#[test]
fn test_stringify_list_with_none() {
    let mut destination = BufferDestination::new();
    let list = vec![make_node(32), Node::None, make_node("test")];
    stringify(&make_node(list), &mut destination).unwrap();
    assert_eq!(destination.to_string(), "li32e4:teste");
}

#[test]
fn test_stringify_dictionary_with_list() {
    let mut destination = BufferDestination::new();
    let mut dict = HashMap::new();
    dict.insert(
        String::from("list"),
        make_node(vec![make_node(1), make_node(2)]),
    );
    stringify(&make_node(dict), &mut destination).unwrap();
    assert_eq!(destination.to_string(), "d4:listli1ei2eee");
}

#[test]
fn test_stringify_deeply_nested_structure() {
    let mut destination = BufferDestination::new();

    // Create a complex nested structure
    let mut inner_dict = HashMap::new();
    inner_dict.insert(String::from("nested"), make_node("value"));

    let list = vec![make_node(1), make_node("text"), make_node(inner_dict)];

    let mut outer_dict = HashMap::new();
    outer_dict.insert(String::from("data"), make_node(list));
    outer_dict.insert(String::from("count"), make_node(42));

    stringify(&make_node(outer_dict), &mut destination).unwrap();
    assert_eq!(
        destination.to_string(),
        "d5:counti42e4:datali1e4:textd6:nested5:valueeee"
    );
}

#[test]
fn test_stringify_multiple_nested_lists() {
    let mut destination = BufferDestination::new();
    let inner_list1 = vec![make_node(1), make_node(2)];
    let inner_list2 = vec![make_node(3), make_node(4)];
    let outer_list = vec![make_node(inner_list1), make_node(inner_list2)];
    stringify(&make_node(outer_list), &mut destination).unwrap();
    assert_eq!(destination.to_string(), "lli1ei2eeli3ei4eee");
}

#[test]
fn test_stringify_dictionary_key_ordering() {
    let mut destination = BufferDestination::new();
    let mut dict = HashMap::new();
    // Insert in non-alphabetical order
    dict.insert(String::from("zebra"), make_node(1));
    dict.insert(String::from("apple"), make_node(2));
    dict.insert(String::from("middle"), make_node(3));
    stringify(&make_node(dict), &mut destination).unwrap();
    // Should be ordered alphabetically
    assert_eq!(destination.to_string(), "d5:applei2e6:middlei3e5:zebrai1ee");
}

#[test]
fn test_stringify_empty_string() {
    let mut destination = BufferDestination::new();
    stringify(&make_node(""), &mut destination).unwrap();
    assert_eq!(destination.to_string(), "0:");
}

#[test]
fn test_stringify_negative_integer() {
    let mut destination = BufferDestination::new();
    stringify(&make_node(-42), &mut destination).unwrap();
    assert_eq!(destination.to_string(), "i-42e");
}

#[test]
fn test_stringify_mixed_dictionary() {
    let mut destination = BufferDestination::new();
    let mut dict = HashMap::new();
    dict.insert(String::from("integer"), make_node(100));
    dict.insert(String::from("string"), make_node("hello"));
    dict.insert(
        String::from("list"),
        make_node(vec![make_node(1), make_node(2), make_node(3)]),
    );

    let mut nested = HashMap::new();
    nested.insert(String::from("x"), make_node(10));
    dict.insert(String::from("dict"), make_node(nested));

    stringify(&make_node(dict), &mut destination).unwrap();
    assert_eq!(
        destination.to_string(),
        "d4:dictd1:xi10ee7:integeri100e4:listli1ei2ei3ee6:string5:helloe"
    );
}
