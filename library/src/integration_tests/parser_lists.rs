//! Integration tests for parsing bencode lists.


#[cfg(test)]
mod tests {
    use crate::BufferSource;
    use crate::error::messages::*;
    use crate::nodes::node::Node;
    use crate::parser::default::parse;
    
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
    
    #[test]
    fn test_list_single_element() {
        let mut source = BufferSource::new(b"li42ee");
        match parse(&mut source) {
            Ok(Node::List(list)) => {
                assert_eq!(list.len(), 1);
                assert!(matches!(&list[0], Node::Integer(42)));
            }
            _ => panic!("Expected single element list"),
        }
    }
    
    #[test]
    fn test_unterminated_list() {
        let mut source = BufferSource::new(b"li1ei2e");
        assert!(matches!(parse(&mut source), Err(s) if s == ERR_UNTERMINATED_LIST));
    }
    
    #[test]
    fn test_list_of_strings() {
        let mut source = BufferSource::new(b"l3:foo3:bar3:baze");
        match parse(&mut source) {
            Ok(Node::List(list)) => {
                assert_eq!(list.len(), 3);
                assert!(matches!(&list[0], Node::Str(s) if s == "foo"));
                assert!(matches!(&list[1], Node::Str(s) if s == "bar"));
                assert!(matches!(&list[2], Node::Str(s) if s == "baz"));
            }
            _ => panic!("Expected list of strings"),
        }
    }
    
    #[test]
    fn test_deeply_nested_lists() {
        let mut source = BufferSource::new(b"llli1eeee");
        match parse(&mut source) {
            Ok(Node::List(list)) => {
                assert_eq!(list.len(), 1);
                if let Node::List(inner1) = &list[0] {
                    assert_eq!(inner1.len(), 1);
                    if let Node::List(inner2) = &inner1[0] {
                        assert_eq!(inner2.len(), 1);
                        assert!(matches!(&inner2[0], Node::Integer(1)));
                    } else {
                        panic!("Expected nested list");
                    }
                } else {
                    panic!("Expected nested list");
                }
            }
            _ => panic!("Expected deeply nested list"),
        }
    }
    
    #[test]
    fn test_list_with_empty_string() {
        let mut source = BufferSource::new(b"li1e0:i2ee");
        match parse(&mut source) {
            Ok(Node::List(list)) => {
                assert_eq!(list.len(), 3);
                assert!(matches!(&list[0], Node::Integer(1)));
                assert!(matches!(&list[1], Node::Str(s) if s.is_empty()));
                assert!(matches!(&list[2], Node::Integer(2)));
            }
            _ => panic!("Expected list with empty string"),
        }
    }
    
    #[test]
    fn test_list_with_dictionary() {
        let mut source = BufferSource::new(b"ld3:foo3:baree");
        match parse(&mut source) {
            Ok(Node::List(list)) => {
                assert_eq!(list.len(), 1);
                assert!(matches!(&list[0], Node::Dictionary(_)));
            }
            _ => panic!("Expected list with dictionary"),
        }
    }
    
    #[test]
    fn test_very_long_list() {
        // Create a list with many elements
        let data = b"li1ei2ei3ei4ei5ei6ei7ei8ei9ei10ee";
        let mut source = BufferSource::new(data);
        match parse(&mut source) {
            Ok(Node::List(list)) => {
                assert_eq!(list.len(), 10);
            }
            _ => panic!("Expected long list"),
        }
    }
    
}
