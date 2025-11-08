//! Integration tests for the bencode TOML stringify functionality.
//! These tests validate the TOML stringify behavior from an external perspective,
//! testing the public API against various node structures.

use bencode_lib::BufferDestination;
use bencode_lib::BufferSource;
use bencode_lib::nodes::node::{Node, make_node};
use bencode_lib::stringify::toml::stringify;
use std::collections::HashMap;

#[test]
fn test_stringify_nested_dictionary() {
    let mut destination = BufferDestination::new();
    let mut inner_dict = HashMap::new();
    inner_dict.insert("inner_key".to_string(), make_node("inner_value"));
    let mut outer_dict = HashMap::new();
    outer_dict.insert("outer_key".to_string(), make_node(inner_dict));
    let node = make_node(outer_dict);
    stringify(&node, &mut destination).unwrap();
    assert_eq!(
        destination.to_string(),
        "[outer_key]\ninner_key = \"inner_value\"\n"
    );
}

#[test]
fn test_stringify_deeply_nested_dictionary() {
    let mut level3 = HashMap::new();
    level3.insert("deep_key".to_string(), Node::Integer(123));
    let level3 = Node::Dictionary(level3);

    let mut level2 = HashMap::new();
    level2.insert("level3".to_string(), level3);
    let level2 = Node::Dictionary(level2);

    let mut level1 = HashMap::new();
    level1.insert("level2".to_string(), level2);
    let level1 = Node::Dictionary(level1);

    let mut root = HashMap::new();
    root.insert("level1".to_string(), level1);

    let mut dest = BufferDestination::new();
    stringify(&Node::Dictionary(root), &mut dest).unwrap();
    assert_eq!(dest.to_string(), "[level1.level2.level3]\ndeep_key = 123\n");
}

#[test]
fn test_stringify_non_dictionary_root() {
    let mut destination = BufferDestination::new();
    let node = make_node("test");
    let result = stringify(&node, &mut destination);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "TOML format requires a dictionary at the root level"
    );
}

#[test]
fn test_heterogeneous_list() {
    let mut dest = BufferDestination::new();
    let mut dict = HashMap::new();
    dict.insert(
        "key".to_string(),
        make_node(vec![make_node(1), make_node("test")]),
    );
    let result = stringify(&Node::Dictionary(dict), &mut dest);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "TOML lists must contain elements of the same type"
    );
}

#[test]
fn test_array_table() {
    let mut dest = BufferDestination::new();
    let mut inner1 = HashMap::new();
    inner1.insert("name".to_string(), make_node("first"));
    let mut inner2 = HashMap::new();
    inner2.insert("name".to_string(), make_node("second"));

    let mut dict = HashMap::new();
    dict.insert(
        "items".to_string(),
        make_node(vec![make_node(inner1), make_node(inner2)]),
    );

    stringify(&Node::Dictionary(dict), &mut dest).unwrap();
    assert_eq!(
        dest.to_string(),
        "[[items]]\nname = \"first\"\n[[items]]\nname = \"second\"\n"
    );
}

#[test]
fn test_mixed_array_table() {
    let mut dest = BufferDestination::new();
    let mut inner = HashMap::new();
    inner.insert("simple".to_string(), make_node(42));
    let mut nested = HashMap::new();
    nested.insert("value".to_string(), make_node("test"));
    inner.insert("complex".to_string(), make_node(nested));

    let mut dict = HashMap::new();
    dict.insert("items".to_string(), make_node(vec![make_node(inner)]));

    stringify(&Node::Dictionary(dict), &mut dest).unwrap();
    assert_eq!(
        dest.to_string(),
        "[[items]]\nsimple = 42\n[items.complex]\nvalue = \"test\"\n"
    );
}

#[test]
fn test_nested_array_tables() {
    let mut dest = BufferDestination::new();
    let mut deepest = HashMap::new();
    deepest.insert("value".to_string(), make_node(42));

    let mut inner = HashMap::new();
    inner.insert(
        "nested".to_string(),
        make_node(vec![make_node(deepest.clone())]),
    );

    let mut dict = HashMap::new();
    dict.insert("items".to_string(), make_node(vec![make_node(inner)]));

    stringify(&Node::Dictionary(dict), &mut dest).unwrap();
    assert_eq!(dest.to_string(), "[[items]]\n[items.nested]\nvalue = 42\n");
}

#[test]
fn test_stringify_nested_object_with_array() {
    let mut source = BufferSource::new(
        b"d4:infod5:filesld6:lengthi351874e4:pathl10:large.jpegeed6:lengthi100e4:pathl1:2eeeee",
    );
    let node = bencode_lib::parse(&mut source).unwrap();
    let mut dest = BufferDestination::new();
    stringify(&node, &mut dest).unwrap();
    assert_eq!(
        dest.to_string(),
        "[[info.files]]\nlength = 351874\npath = [\"large.jpeg\"]\n[[info.files]]\nlength = 100\npath = [\"2\"]\n"
    );
}

#[test]
fn test_stringify_nested_object_with_array_and_object_and_array() {
    let mut source = BufferSource::new(
        b"d4:infod5:filesld6:lengthi351874e4:pathl10:large.jpegeed6:lengthi100e4:pathl1:2eeeee4:filesld6:lengthi351874e4:pathl10:large.jpege",
    );
    let node = bencode_lib::parse(&mut source).unwrap();
    let mut dest = BufferDestination::new();
    stringify(&node, &mut dest).unwrap();
    assert_eq!(
        dest.to_string(),
        "[[info.files]]\nlength = 351874\npath = [\"large.jpeg\"]\n[[info.files]]\nlength = 100\npath = [\"2\"]\n"
    );
}

#[test]
fn test_stringify_empty_dictionary() {
    let mut dest = BufferDestination::new();
    stringify(&Node::Dictionary(HashMap::new()), &mut dest).unwrap();
    assert_eq!(dest.to_string(), "");
}

#[test]
fn test_stringify_complex_mixed_structure() {
    let mut dest = BufferDestination::new();

    // Create a complex structure with multiple nesting levels
    let mut dict = HashMap::new();
    dict.insert("title".to_string(), make_node("Test Document"));
    dict.insert("version".to_string(), make_node(1));

    let mut author = HashMap::new();
    author.insert("name".to_string(), make_node("John Doe"));
    author.insert("email".to_string(), make_node("john@example.com"));
    dict.insert("author".to_string(), make_node(author));

    let mut item1 = HashMap::new();
    item1.insert("id".to_string(), make_node(1));
    item1.insert("name".to_string(), make_node("Item One"));

    let mut item2 = HashMap::new();
    item2.insert("id".to_string(), make_node(2));
    item2.insert("name".to_string(), make_node("Item Two"));

    dict.insert(
        "items".to_string(),
        make_node(vec![make_node(item1), make_node(item2)]),
    );

    stringify(&Node::Dictionary(dict), &mut dest).unwrap();
    let output = dest.to_string();

    // Verify key components are present
    assert!(output.contains("title = \"Test Document\""));
    assert!(output.contains("version = 1"));
    assert!(output.contains("[author]"));
    assert!(output.contains("name = \"John Doe\""));
    assert!(output.contains("[[items]]"));
    assert!(output.contains("id = 1"));
    assert!(output.contains("name = \"Item One\""));
}

#[test]
fn test_stringify_with_list_of_primitives() {
    let mut dest = BufferDestination::new();
    let mut dict = HashMap::new();
    dict.insert(
        "numbers".to_string(),
        make_node(vec![make_node(1), make_node(2), make_node(3)]),
    );
    dict.insert(
        "strings".to_string(),
        make_node(vec![make_node("a"), make_node("b"), make_node("c")]),
    );

    stringify(&Node::Dictionary(dict), &mut dest).unwrap();
    let output = dest.to_string();

    assert!(output.contains("numbers = [1, 2, 3]"));
    assert!(output.contains("strings = [\"a\", \"b\", \"c\"]"));
}
