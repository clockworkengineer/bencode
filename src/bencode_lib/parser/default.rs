use crate::bencode_lib::io::sources::buffer::Buffer;
use crate::bencode_lib::nodes::node::Node;
use std::collections::HashMap;

pub fn parse(source: &mut Buffer) -> Option<Node> {
    match source.current()? {
        'i' => parse_integer(source),
        'l' => parse_list(source),
        'd' => parse_dictionary(source),
        '0'..='9' => parse_string(source),
        _ => None,
    }
}

fn parse_integer(source: &mut Buffer) -> Option<Node> {
    source.next(); // skip 'i'
    let mut number = String::new();
    while let Some(c) = source.current() {
        if c == 'e' {
            source.next();
            return number.parse::<u32>().ok().map(Node::Integer);
        }
        number.push(c);
        source.next();
    }
    None
}

fn parse_string(source: &mut Buffer) -> Option<Node> {
    let mut length = String::new();
    while let Some(c) = source.current() {
        if c == ':' {
            source.next();
            break;
        }
        length.push(c);
        source.next();
    }

    let len = length.parse::<usize>().ok()?;
    let mut string = String::new();
    for _ in 0..len {
        if let Some(c) = source.current() {
            string.push(c);
            source.next();
        } else {
            return None;
        }
    }
    Some(Node::Str(string))
}

fn parse_list(source: &mut Buffer) -> Option<Node> {
    source.next(); // skip 'l'
    let mut list = Vec::new();
    while let Some(c) = source.current() {
        if c == 'e' {
            source.next();
            return Some(Node::List(list));
        }
        if let Some(node) = parse(source) {
            list.push(node);
        } else {
            return None;
        }
    }
    None
}

fn parse_dictionary(source: &mut Buffer) -> Option<Node> {
    source.next(); // skip 'd'
    let mut dict = HashMap::new();
    while let Some(c) = source.current() {
        if c == 'e' {
            source.next();
            return Some(Node::Dictionary(dict));
        }
        if let Some(Node::Str(key)) = parse_string(source) {
            if let Some(value) = parse(source) {
                dict.insert(key, value);
                continue;
            }
        }
        return None;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_integer_works() {
        let mut source = Buffer::new(b"i32e");
        assert!(matches!(parse(&mut source), Some(Node::Integer(32))));
    }

    #[test]
    fn parse_string_works() {
        let mut source = Buffer::new(b"4:test");
        assert!(matches!(parse(&mut source), Some(Node::Str(s)) if s == "test"));
    }

    #[test]
    fn parse_list_works() {
        let mut source = Buffer::new(b"li32ei33ee");
        match parse(&mut source) {
            Some(Node::List(list)) => {
                assert_eq!(list.len(), 2);
                assert!(matches!(&list[0], Node::Integer(32)));
                assert!(matches!(&list[1], Node::Integer(33)));
            }
            _ => panic!("Expected list"),
        }
    }

    #[test]
    fn parse_dictionary_works() {
        let mut source = Buffer::new(b"d4:testi32ee");
        match parse(&mut source) {
            Some(Node::Dictionary(dict)) => {
                assert_eq!(dict.len(), 1);
                assert!(matches!(dict.get("test"), Some(Node::Integer(32))));
            }
            _ => panic!("Expected dictionary"),
        }
    }
    #[test]
    fn parse_integer_with_error() {
        let mut source = Buffer::new(b"i32");
        assert!(matches!(parse(&mut source), None));
    }

    #[test]
    fn parse_string_with_error() {
        let mut source = Buffer::new(b"4:tes");
        assert!(matches!(parse(&mut source), None));
    }

    #[test]
    fn parse_list_with_error() {
        let mut source = Buffer::new(b"li32ei33");
        assert!(matches!(parse(&mut source), None));
    }

    #[test]
    fn parse_dictionary_with_error() {
        let mut source = Buffer::new(b"d4:testi32e");
        assert!(matches!(parse(&mut source), None));
    }
}
