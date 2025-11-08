//! Integration tests for parsing bencode lists.

use bencode_lib::BufferSource;
use bencode_lib::nodes::node::Node;
use bencode_lib::parser::default::parse;

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
