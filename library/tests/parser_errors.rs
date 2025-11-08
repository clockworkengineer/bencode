//! Integration tests for parser error handling.

use bencode_lib::BufferSource;
use bencode_lib::error::messages::*;
use bencode_lib::parser::default::parse;

#[test]
fn test_empty_input() {
    let mut source = BufferSource::new(b"");
    assert!(matches!(parse(&mut source), Err(s) if s == ERR_EMPTY_INPUT));
}

#[test]
fn test_invalid_character() {
    let mut source = BufferSource::new(b"x123");
    assert!(matches!(parse(&mut source), Err(s) if s.contains("Unexpected character")));
}
