//! Integration tests for parsing bencode strings.

use bencode_lib::BufferSource;
use bencode_lib::error::messages::*;
use bencode_lib::nodes::node::Node;
use bencode_lib::parser::default::parse;

#[test]
fn test_zero_length_string() {
    let mut source = BufferSource::new(b"0:");
    assert!(matches!(parse(&mut source), Ok(Node::Str(s)) if s.is_empty()));
}

#[test]
fn test_invalid_string_colon_only() {
    let mut source = BufferSource::new(b":");
    assert!(matches!(parse(&mut source), Err(s) if s == ERR_INVALID_STRING_LENGTH));
}

#[test]
fn test_invalid_string_length() {
    let mut source = BufferSource::new(b"a:test");
    assert!(matches!(parse(&mut source), Err(s) if s.contains("Unexpected character")));
}
