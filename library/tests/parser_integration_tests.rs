//! Integration tests for the bencode parser.
//! These tests validate the parser's behavior from an external perspective,
//! testing the public API against various bencode inputs.

use bencode_lib::BufferSource;
use bencode_lib::Node::Dictionary;
use bencode_lib::error::messages::*;
use bencode_lib::nodes::node::Node;
use bencode_lib::parser::default::parse;

#[test]
fn test_complex_dictionary() {
    let mut source = BufferSource::new(b"d3:foo3:bar5:hello5:world4:test5:valuee");
    let result = parse(&mut source);
    assert!(result.is_ok());
    if let Ok(Dictionary(map)) = result {
        assert_eq!(map.len(), 3);
        assert_eq!(map.get("foo").unwrap(), &Node::Str("bar".to_string()));
        assert_eq!(map.get("test").unwrap(), &Node::Str("value".to_string()));
        assert_eq!(map.get("hello").unwrap(), &Node::Str("world".to_string()));
    } else {
        panic!("Expected dictionary");
    }
}

#[test]
fn test_invalid_character() {
    let mut source = BufferSource::new(b"x123");
    assert!(matches!(parse(&mut source), Err(s) if s.contains("Unexpected character")));
}

#[test]
fn test_empty_input() {
    let mut source = BufferSource::new(b"");
    assert!(matches!(parse(&mut source), Err(s) if s == ERR_EMPTY_INPUT));
}

#[test]
fn test_invalid_string_colon_only() {
    let mut source = BufferSource::new(b":");
    assert!(matches!(parse(&mut source), Err(s) if s == ERR_INVALID_STRING_LENGTH));
}

#[test]
fn test_zero_length_string() {
    let mut source = BufferSource::new(b"0:");
    assert!(matches!(parse(&mut source), Ok(Node::Str(s)) if s.is_empty()));
}

#[test]
fn test_invalid_string_length() {
    let mut source = BufferSource::new(b"a:test");
    assert!(matches!(parse(&mut source), Err(s) if s.contains("Unexpected character")));
}

#[test]
fn test_nested_list_works() {
    let mut source = BufferSource::new(b"lli1ei2eeli3ei4eee");
    match parse(&mut source) {
        Ok(Node::List(list)) => {
            assert_eq!(list.len(), 2);
            assert!(matches!(&list[0], Node::List(l) if l.len() == 2));
            assert!(matches!(&list[1], Node::List(l) if l.len() == 2));
        }
        _ => panic!("Expected nested list"),
    }
}

#[test]
fn test_mixed_list_works() {
    let mut source = BufferSource::new(b"li32e4:testi-42ee");
    match parse(&mut source) {
        Ok(Node::List(list)) => {
            assert_eq!(list.len(), 3);
            assert!(matches!(&list[0], Node::Integer(32)));
            assert!(matches!(&list[1], Node::Str(s) if s == "test"));
            assert!(matches!(&list[2], Node::Integer(-42)));
        }
        _ => panic!("Expected mixed list"),
    }
}

#[test]
fn test_empty_list_works() {
    let mut source = BufferSource::new(b"le");
    match parse(&mut source) {
        Ok(Node::List(list)) => {
            assert_eq!(list.len(), 0);
        }
        _ => panic!("Expected empty list"),
    }
}

#[test]
fn test_empty_dictionary_works() {
    let mut source = BufferSource::new(b"de");
    match parse(&mut source) {
        Ok(Dictionary(dict)) => {
            assert_eq!(dict.len(), 0);
        }
        _ => panic!("Expected empty dictionary"),
    }
}

#[test]
fn test_dictionary_with_non_string_key_fails() {
    let mut source = BufferSource::new(b"di32ei42ee");
    assert!(matches!(parse(&mut source), Err(s) if s == ERR_DICT_KEY_MUST_BE_STRING));
}

#[test]
fn test_dictionary_with_unordered_keys_fails() {
    let mut source = BufferSource::new(b"d3:bbci32e3:abci42ee");
    assert!(matches!(parse(&mut source), Err(s) if s == ERR_DICT_KEYS_ORDER));
}

#[test]
fn test_invalid_integer_format_fails() {
    let mut source = BufferSource::new(b"i++32e");
    assert!(matches!(parse(&mut source), Err(s) if s == ERR_INVALID_INTEGER));
}
