//! Integration tests for parsing bencode dictionaries.

use bencode_lib::BufferSource;
use bencode_lib::Node::Dictionary;
use bencode_lib::error::messages::*;
use bencode_lib::nodes::node::Node;
use bencode_lib::parser::default::parse;

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
fn test_dictionary_with_non_string_key_fails() {
    let mut source = BufferSource::new(b"di32ei42ee");
    assert!(matches!(parse(&mut source), Err(s) if s == ERR_DICT_KEY_MUST_BE_STRING));
}

#[test]
fn test_dictionary_with_unordered_keys_fails() {
    let mut source = BufferSource::new(b"d3:bbci32e3:abci42ee");
    assert!(matches!(parse(&mut source), Err(s) if s == ERR_DICT_KEYS_ORDER));
}
