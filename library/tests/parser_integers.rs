//! Integration tests for parsing bencode integers.

use bencode_lib::BufferSource;
use bencode_lib::error::messages::*;
use bencode_lib::parser::default::parse;

#[test]
fn test_invalid_integer_format_fails() {
    let mut source = BufferSource::new(b"i++32e");
    assert!(matches!(parse(&mut source), Err(s) if s == ERR_INVALID_INTEGER));
}
