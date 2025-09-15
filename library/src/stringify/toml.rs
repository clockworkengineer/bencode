use crate::io::traits::IDestination;
use crate::nodes::node::*;
use crate::stringify::common::escape_string;


pub fn stringify(node: &Node, destination: &mut dyn IDestination) -> Result<(), String> {
    match node {
        Node::Dictionary(dict) => stringify_dictionary(dict, "", destination),
        _ => Err("TOML format requires a dictionary at the root level".to_string()),
    }
}

fn stringify_value(value: &Node, destination: &mut dyn IDestination) -> Result<(), String> {
    match value {
        Node::Str(s) => {
            destination.add_bytes("\"");
            escape_string(s, destination);
            destination.add_bytes("\"");
        }
        Node::Integer(i) => destination.add_bytes(&i.to_string()),
        Node::List(items) => stringify_array(items, destination)?,
        Node::None => destination.add_bytes("null"),
        Node::Dictionary(_) => return Ok(()), // Handled separately for table syntax
    }
    Ok(())
}

fn stringify_array(items: &Vec<Node>, destination: &mut dyn IDestination) -> Result<(), String> {
    destination.add_bytes("[");
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            destination.add_bytes(", ");
        }
        stringify_value(item, destination)?;
    }
    destination.add_bytes("]");
    Ok(())
}

fn stringify_dictionary(dict: &std::collections::HashMap<String, Node>, prefix: &str, destination: &mut dyn IDestination) -> Result<(), String> {
    if dict.is_empty() {
        return Ok(());
    }

    let mut tables = std::collections::HashMap::new();
    let mut is_first = true;
    // First pass - handle simple key-value pairs
    for (key, value) in dict {
        if let Node::Dictionary(nested) = value {
            tables.insert(key, nested);
        } else {
            if !prefix.is_empty() && is_first {
                destination.add_bytes("\n[");
                destination.add_bytes(prefix);
                destination.add_bytes("]\n");
                is_first = false;
            }
            destination.add_bytes(key);
            destination.add_bytes(" = ");
            stringify_value(value, destination)?;
            if !prefix.is_empty() {
                destination.add_bytes("\n");
            }
        }
    }

    // Second pass - handle nested tables
    for (key, nested) in tables {
        let new_prefix = if prefix.is_empty() {
            key.to_string()
        } else {
            format!("{}.{}", prefix, key)
        };
        stringify_dictionary(nested, &new_prefix, destination)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::io::destinations::BufferDestination::Buffer;
    use crate::nodes::node::make_node;
    use std::collections::HashMap;
    use crate::BufferDestination;

    #[test]
    fn test_stringify_string() {
        let mut destination = BufferDestination::new();
        let mut dict = std::collections::HashMap::new();
        dict.insert("key".to_string(), make_node("test"));
        let node = make_node(dict);
        stringify(&node, &mut destination).unwrap();
        assert_eq!(destination.to_string(), "key = \"test\"");
    }

    #[test]
    fn test_stringify_integer() {
        let mut destination = BufferDestination::new();
        let mut dict = std::collections::HashMap::new();
        dict.insert("key".to_string(), make_node(42));
        let node = make_node(dict);
        stringify(&node, &mut destination).unwrap();
        assert_eq!(destination.to_string(), "key = 42");
    }

    #[test]
    fn test_stringify_list() {
        let mut destination = BufferDestination::new();
        let mut dict = std::collections::HashMap::new();
        dict.insert(
            "key".to_string(),
            make_node(vec![make_node(1), make_node(2), make_node(3)]),
        );
        let node = make_node(dict);
        stringify(&node, &mut destination).unwrap();
        assert_eq!(destination.to_string(), "key = [1, 2, 3]");
    }

    #[test]
    fn test_stringify_dictionary() {
        let mut destination = BufferDestination::new();
        let mut dict = std::collections::HashMap::new();
        dict.insert("key".to_string(), make_node("value"));
        let node = make_node(dict);
        stringify(&node, &mut destination).unwrap();
        assert_eq!(destination.to_string(), "key = \"value\"");
    }

    #[test]
    fn test_stringify_nested_dictionary() {
        let mut destination = BufferDestination::new();
        let mut inner_dict = std::collections::HashMap::new();
        inner_dict.insert("inner_key".to_string(), make_node("inner_value"));
        let mut outer_dict = std::collections::HashMap::new();
        outer_dict.insert("outer_key".to_string(), make_node(inner_dict));
        let node = make_node(outer_dict);
        stringify(&node, &mut destination).unwrap();
        assert_eq!(
            destination.to_string(),
            "\n[outer_key]\ninner_key = \"inner_value\"\n"
        );
    }

    #[test]
    fn test_stringify_none() {
        let mut destination = BufferDestination::new();
        let mut dict = std::collections::HashMap::new();
        dict.insert("key".to_string(), Node::None);
        let node = make_node(dict);
        stringify(&node, &mut destination).unwrap();
        assert_eq!(destination.to_string(), "key = null");
    }

    #[test]
    fn test_stringify_non_dictionary_root() {
        let mut destination = BufferDestination::new();
        let node = make_node("test");
        let result = stringify(&node, &mut destination);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "TOML format requires a dictionary at the root level"
        );
    }
    #[test]
    fn test_stringify_deeply_nested_dictionary() {
        let mut level3 = HashMap::new();
        level3.insert(
            "deep_key".to_string(),
            Node::Integer(123),
        );
        let level3 = Node::Dictionary(level3);

        let mut level2 = HashMap::new();
        level2.insert("level3".to_string(), level3);
        let level2 = Node::Dictionary(level2);

        let mut level1 = HashMap::new();
        level1.insert("level2".to_string(), level2);
        let level1 = Node::Dictionary(level1);

        let mut root = HashMap::new();
        root.insert("level1".to_string(), level1);

        let mut dest = BufferDestination::new();
        stringify(&Node::Dictionary(root), &mut dest).unwrap();
        assert_eq!(
            dest.to_string(),
            "\n[level1.level2.level3]\ndeep_key = 123\n"
        );
    }
    #[test]
    fn test_stringify_empty_dictionary() {
        let mut dest = BufferDestination::new();
        stringify(&Node::Dictionary(HashMap::new()), &mut dest).unwrap();
        assert_eq!(dest.to_string(), "");
    }

}
