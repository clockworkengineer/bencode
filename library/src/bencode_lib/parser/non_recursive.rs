use crate::bencode_lib::nodes::node::Node;
use std::collections::HashMap;
use crate::bencode_lib::io::traits::ISource;

pub fn parse(source: &mut dyn ISource) -> Result<Node, String> {
    let mut stack: Vec<(Node, usize)> = Vec::new();
    let mut current_string = String::new();
    let mut current_number: String = String::new();

    while source.more() {
        match source.current() {
            Some('i') => {
                source.next();
                while let Some(c) = source.current() {
                    if c == 'e' {
                        break;
                    }
                    current_number.push(c);
                    source.next();
                }
                if source.current() != Some('e') {
                    return Err("Invalid integer format".to_string());
                }
                source.next();
                let value = current_number.parse::<i64>()
                    .map_err(|_| "Invalid integer".to_string())?;
                current_number.clear();

                if stack.is_empty() {
                    return Ok(Node::Integer(value));
                }
                handle_value(&mut stack, Node::Integer(value))?;
            }
            Some('l') => {
                source.next();
                stack.push((Node::List(Vec::new()), 0));
            }
            Some('d') => {
                source.next();
                stack.push((Node::Dictionary(HashMap::new()), 0));
            }
            Some('e') => {
                source.next();
                if stack.is_empty() {
                    return Err("Unexpected end marker".to_string());
                }
                let (node, _) = stack.pop().unwrap();
                if stack.is_empty() {
                    return Ok(node);
                }
                handle_value(&mut stack, node)?;
            }
            Some(c) if c.is_ascii_digit() => {
                while let Some(c) = source.current() {
                    if c == ':' {
                        break;
                    }
                    current_string.push(c);
                    source.next();
                }
                if source.current() != Some(':') {
                    return Err("Invalid string length format".to_string());
                }
                source.next();

                let length = current_string.parse::<usize>()
                    .map_err(|_| "Invalid string length".to_string())?;
                current_string.clear();

                for _ in 0..length {
                    if let Some(c) = source.current() {
                        current_string.push(c);
                        source.next();
                    } else {
                        return Err("Unexpected end of input".to_string());
                    }
                }

                let string_value = current_string.clone();
                current_string.clear();

                if stack.is_empty() {
                    return Ok(Node::Str(string_value));
                }
                handle_value(&mut stack, Node::Str(string_value))?;
            }
            Some(_) => {
                source.next();
            }
            None => break,
        }
    }

    if stack.is_empty() {
        Err("Empty input".to_string())
    } else {
        Err("Incomplete input".to_string())
    }
}

fn handle_value(stack: &mut Vec<(Node, usize)>, value: Node) -> Result<(), String> {
    let last = stack.last_mut().unwrap();
    match last.0 {
        Node::List(ref mut v) => {
            v.push(value);
        }
        Node::Dictionary(ref mut m) => {
            if last.1 % 2 == 0 {
                if let Node::Str(key) = value {
                    last.1 += 1;
                    // Using a default null-like value until the real value is inserted
                    m.insert(key, Node::List(Vec::new()));
                } else {
                    return Err("Dictionary key must be a string".to_string());
                }
            } else {
                if let Some(key) = m.keys().last() {
                    m.insert(key.clone(), value);
                }
                last.1 += 1;
            }
        }
        _ => return Err("Invalid stack state".to_string()),
    }
    Ok(())
}
