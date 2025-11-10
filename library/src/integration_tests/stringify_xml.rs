//! Integration tests for the bencode XML stringify functionality.
//! These tests validate the XML stringify behavior from an external perspective,
//! testing the public API against various node structures.

use crate::BufferDestination;
use crate::nodes::node::Node;
use crate::stringify::xml::stringify;
use std::collections::HashMap;

#[test]
fn test_nested_dictionary() {
    let mut destination = BufferDestination::new();
    let mut inner_dict = HashMap::new();
    inner_dict.insert("inner_key".into(), Node::Integer(42));
    let mut outer_dict = HashMap::new();
    outer_dict.insert("outer_key".into(), Node::Dictionary(inner_dict));
    stringify(&Node::Dictionary(outer_dict), &mut destination).unwrap();
    assert_eq!(
        destination.to_string(),
        "<dictionary><item><key>outer_key</key><value><dictionary><item><key>inner_key</key><value><integer>42</integer></value></item></dictionary></value></item></dictionary>"
    );
}

#[test]
fn test_nested_list_mixed() {
    let mut destination = BufferDestination::new();
    let nested = Node::List(vec![
        Node::Integer(1),
        Node::List(vec![Node::Str("nested".into())]),
        Node::None,
    ]);
    stringify(&nested, &mut destination).unwrap();
    assert_eq!(
        destination.to_string(),
        "<list><integer>1</integer><list><string>nested</string></list></list>"
    );
}

#[test]
fn test_empty_structures() {
    let mut destination = BufferDestination::new();
    stringify(&Node::List(vec![]), &mut destination).unwrap();
    assert_eq!(destination.to_string(), "<list></list>");

    let mut destination = BufferDestination::new();
    stringify(&Node::Dictionary(HashMap::new()), &mut destination).unwrap();
    assert_eq!(destination.to_string(), "<dictionary></dictionary>");
}

#[test]
fn test_complex_nested_structure() {
    let mut destination = BufferDestination::new();

    let mut inner_dict = HashMap::new();
    inner_dict.insert("id".into(), Node::Integer(1));
    inner_dict.insert("name".into(), Node::Str("Item".into()));

    let list = Node::List(vec![
        Node::Dictionary(inner_dict),
        Node::Integer(42),
        Node::Str("text".into()),
    ]);

    let mut outer_dict = HashMap::new();
    outer_dict.insert("data".into(), list);

    stringify(&Node::Dictionary(outer_dict), &mut destination).unwrap();
    let output = destination.to_string();

    assert!(output.starts_with("<dictionary>"));
    assert!(output.ends_with("</dictionary>"));
    assert!(output.contains("<key>data</key>"));
    assert!(output.contains("<list>"));
    assert!(output.contains("<integer>1</integer>"));
    assert!(output.contains("<integer>42</integer>"));
    assert!(output.contains("<string>Item</string>"));
    assert!(output.contains("<string>text</string>"));
}

#[test]
fn test_dictionary_with_multiple_keys() {
    let mut destination = BufferDestination::new();
    let mut dict = HashMap::new();
    dict.insert("key1".into(), Node::Integer(1));
    dict.insert("key2".into(), Node::Str("value2".into()));
    dict.insert("key3".into(), Node::Integer(3));

    stringify(&Node::Dictionary(dict), &mut destination).unwrap();
    let output = destination.to_string();

    assert!(output.starts_with("<dictionary>"));
    assert!(output.ends_with("</dictionary>"));
    assert!(output.contains("<key>key1</key>"));
    assert!(output.contains("<key>key2</key>"));
    assert!(output.contains("<key>key3</key>"));
    assert!(output.contains("<integer>1</integer>"));
    assert!(output.contains("<string>value2</string>"));
    assert!(output.contains("<integer>3</integer>"));
}

#[test]
fn test_deeply_nested_lists() {
    let mut destination = BufferDestination::new();
    let inner_list = Node::List(vec![Node::Integer(1), Node::Integer(2)]);
    let middle_list = Node::List(vec![inner_list, Node::Str("middle".into())]);
    let outer_list = Node::List(vec![middle_list, Node::Integer(3)]);

    stringify(&outer_list, &mut destination).unwrap();
    assert_eq!(
        destination.to_string(),
        "<list><list><list><integer>1</integer><integer>2</integer></list><string>middle</string></list><integer>3</integer></list>"
    );
}

#[test]
fn test_list_with_all_types() {
    let mut destination = BufferDestination::new();
    let mut dict = HashMap::new();
    dict.insert("key".into(), Node::Integer(100));

    let list = Node::List(vec![
        Node::Integer(42),
        Node::Str("text".into()),
        Node::Dictionary(dict),
        Node::List(vec![Node::Integer(1)]),
        Node::None,
    ]);

    stringify(&list, &mut destination).unwrap();
    let output = destination.to_string();

    assert!(output.starts_with("<list>"));
    assert!(output.ends_with("</list>"));
    assert!(output.contains("<integer>42</integer>"));
    assert!(output.contains("<string>text</string>"));
    assert!(output.contains("<dictionary>"));
    assert!(output.contains("<list><integer>1</integer></list>"));
}

#[test]
fn test_empty_string() {
    let mut destination = BufferDestination::new();
    stringify(&Node::Str("".into()), &mut destination).unwrap();
    assert_eq!(destination.to_string(), "<string></string>");
}

#[test]
fn test_negative_integer() {
    let mut destination = BufferDestination::new();
    stringify(&Node::Integer(-42), &mut destination).unwrap();
    assert_eq!(destination.to_string(), "<integer>-42</integer>");
}

#[test]
fn test_dictionary_with_none_value() {
    let mut destination = BufferDestination::new();
    let mut dict = HashMap::new();
    dict.insert("key1".into(), Node::Str("value".into()));
    dict.insert("key2".into(), Node::None);

    stringify(&Node::Dictionary(dict), &mut destination).unwrap();
    let output = destination.to_string();

    assert!(output.contains("<key>key1</key>"));
    assert!(output.contains("<string>value</string>"));
    // None values produce no output in the XML
}

#[test]
fn test_string_with_special_characters() {
    let mut destination = BufferDestination::new();
    stringify(
        &Node::Str("test\"quote\\backslash".into()),
        &mut destination,
    )
    .unwrap();
    assert_eq!(
        destination.to_string(),
        "<string>test\\\"quote\\\\backslash</string>"
    );
}

#[test]
fn test_very_deeply_nested_structure() {
    let mut destination = BufferDestination::new();
    let level5 = Node::Integer(5);
    let level4 = Node::List(vec![level5]);
    let level3 = Node::List(vec![level4]);
    let level2 = Node::List(vec![level3]);
    let level1 = Node::List(vec![level2]);

    stringify(&level1, &mut destination).unwrap();
    assert!(
        destination
            .to_string()
            .contains("<list><list><list><list><integer>5</integer></list></list></list></list>")
    );
}
