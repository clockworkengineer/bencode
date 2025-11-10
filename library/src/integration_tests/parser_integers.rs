//! Integration tests for parsing bencode integers.

use crate::BufferSource;
use crate::error::messages::*;
use crate::nodes::node::Node;
use crate::parser::default::parse;

#[test]
fn test_invalid_integer_format_fails() {
    let mut source = BufferSource::new(b"i++32e");
    assert!(matches!(parse(&mut source), Err(s) if s == ERR_INVALID_INTEGER));
}

#[test]
fn test_valid_positive_integer() {
    let mut source = BufferSource::new(b"i42e");
    assert!(matches!(parse(&mut source), Ok(Node::Integer(42))));
}

#[test]
fn test_valid_negative_integer() {
    let mut source = BufferSource::new(b"i-42e");
    assert!(matches!(parse(&mut source), Ok(Node::Integer(-42))));
}

#[test]
fn test_integer_zero() {
    let mut source = BufferSource::new(b"i0e");
    assert!(matches!(parse(&mut source), Ok(Node::Integer(0))));
}

#[test]
fn test_negative_zero_fails() {
    let mut source = BufferSource::new(b"i-0e");
    assert!(matches!(parse(&mut source), Err(s) if s == ERR_INVALID_INTEGER));
}

#[test]
fn test_unterminated_integer() {
    let mut source = BufferSource::new(b"i42");
    assert!(matches!(parse(&mut source), Err(s) if s == ERR_UNTERMINATED_INTEGER));
}

#[test]
fn test_large_positive_integer() {
    let mut source = BufferSource::new(b"i9223372036854775807e");
    assert!(matches!(
        parse(&mut source),
        Ok(Node::Integer(9223372036854775807))
    ));
}

#[test]
fn test_large_negative_integer() {
    let mut source = BufferSource::new(b"i-9223372036854775808e");
    assert!(matches!(
        parse(&mut source),
        Ok(Node::Integer(-9223372036854775808))
    ));
}

#[test]
fn test_integer_with_invalid_chars() {
    let mut source = BufferSource::new(b"i42ae");
    assert!(matches!(parse(&mut source), Err(s) if s == ERR_INVALID_INTEGER));
}

#[test]
fn test_integer_empty() {
    let mut source = BufferSource::new(b"ie");
    assert!(matches!(parse(&mut source), Err(s) if s == ERR_INVALID_INTEGER));
}

#[test]
fn test_integer_only_negative_sign() {
    let mut source = BufferSource::new(b"i-e");
    assert!(matches!(parse(&mut source), Err(s) if s == ERR_INVALID_INTEGER));
}

#[test]
fn test_integer_with_leading_zeros() {
    // Bencode should accept integers with leading zeros (parser doesn't validate this)
    let mut source = BufferSource::new(b"i00042e");
    assert!(matches!(parse(&mut source), Ok(Node::Integer(42))));
}

#[test]
fn test_integer_overflow() {
    // Test a number that's too large for i64
    let mut source = BufferSource::new(b"i99999999999999999999e");
    assert!(matches!(parse(&mut source), Err(s) if s == ERR_INVALID_INTEGER));
}
