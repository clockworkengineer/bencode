use crate::io::traits::IDestination;
use crate::nodes::node::*;
 use crate::stringify::common::escape_string;

use std::collections::BTreeMap;

pub fn stringify(node: &Node, destination: &mut dyn IDestination) -> Result<(), String> {
    match node {
        Node::Dictionary(dict) => stringify_dictionary(dict, "", destination),
        _ => Err("TOML format requires a dictionary at the root level".to_string()),
    }
}

fn stringify_value(value: &Node, add_cr: bool, destination: &mut dyn IDestination) -> Result<(), String> {
    match value {
        Node::Str(s) => stringify_str(s, destination),
        Node::Integer(value) => stringify_number(value, destination),
        Node::List(items) => stringify_list(items, destination)?,
        Node::None => destination.add_bytes("null"),
        Node::Dictionary(_) => return Ok(()), // Handled separately for table syntax
    }
    if add_cr {
        destination.add_bytes("\n");
    }
    Ok(())
}
fn stringify_str(s: &str, destination: &mut dyn IDestination) {
    destination.add_bytes("\"");
    escape_string(s, destination);
    destination.add_bytes("\"");
}

fn stringify_number(value: &i64, destination: &mut dyn IDestination) {
    destination.add_bytes(&value.to_string());
}

fn stringify_list(items: &Vec<Node>, destination: &mut dyn IDestination) -> Result<(), String> {
    if items.is_empty() {
        destination.add_bytes("[]");
        return Ok(());
    }

    // Check first item's type
    let first_type = match &items[0] {
        Node::Str(_) => "string",
        Node::Integer(_) => "number",
        Node::List(_) => "List",
        Node::Dictionary(_) => "Dictionary",
        Node::None => "null"
    };

    // Validate all items are same type
    for item in items {
        let item_type = match item {
            Node::Str(_) => "string",
            Node::Integer(_) => "number",
            Node::List(_) => "List",
            Node::Dictionary(_) => "Dictionary",
            Node::None => "null"
        };
        if item_type != first_type {
            return Err("TOML Lists must contain elements of the same type".to_string());
        }
    }

    destination.add_bytes("[");
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            destination.add_bytes(", ");
        }
        stringify_value(item, false, destination)?;
    }
    destination.add_bytes("]");
    Ok(())
}
fn stringify_key_value_pair(prefix: &str, destination: &mut dyn IDestination, is_first: &mut bool, key: &String, value: &Node) -> Result<(), String> {
    if !prefix.is_empty() && *is_first {
        destination.add_bytes("[");
        destination.add_bytes(prefix);
        destination.add_bytes("]\n");
        *is_first = false;
    }
    destination.add_bytes(key);
    destination.add_bytes(" = ");
    stringify_value(value, true, destination)?;
    Ok(())
}

fn stringify_dictionary(dict: &std::collections::HashMap<String, Node>, prefix: &str, destination: &mut dyn IDestination) -> Result<(), String> {
    if dict.is_empty() {
        return Ok(());
    }

    let dict_sorted: BTreeMap<_, _> = dict.iter().collect();
    let mut tables = BTreeMap::new();
    let mut list_tables = BTreeMap::new();
    let mut is_first = true;

    // First pass - handle simple key-value pairs and collect tables/Lists of tables
    for (key, value) in dict_sorted {
        match value {
            Node::Dictionary(nested) => {
                tables.insert(key, nested);
            }
            Node::List(items) => {
                if items.iter().all(|item| matches!(item, Node::Dictionary(_))) {
                    // Collect Lists of tables
                    list_tables.insert(key, items);
                } else {
                    stringify_key_value_pair(prefix, destination, &mut is_first, key, value)?;
                }
            }
            _ => {
                stringify_key_value_pair(prefix, destination, &mut is_first, key, value)?;
            }
        }
    }

    // Second pass - handle nested tables
    for (key, nested) in tables {
        let new_prefix = calculate_prefix(prefix, key);
        stringify_dictionary(nested, &new_prefix, destination)?;
    }

    // Third pass - handle Lists of tables
    let list_tables_sorted: BTreeMap<_, _> = list_tables.into_iter().collect();
    for (key, items) in list_tables_sorted {
        for item in items {
            if let Node::Dictionary(nested) = item {
                let new_prefix = calculate_prefix(prefix, key);
                destination.add_bytes("[[");
                destination.add_bytes(&new_prefix);
                destination.add_bytes("]]\n");
                let nested_sorted: BTreeMap<_, _> = nested.iter().collect();
                for (inner_key, inner_value) in &nested_sorted {
                    match inner_value {
                        Node::Dictionary(_) => {
                        }
                        Node::List(_) => {
                        }
                        _ => {
                            destination.add_bytes(inner_key);
                            destination.add_bytes(" = ");
                            stringify_value(inner_value, true, destination)?;
                        }
                    }
                }

                for (inner_key, inner_value) in &nested_sorted {
                    match inner_value {
                        Node::Dictionary(inner_nested) => {
                            let inner_prefix = format!("{}.{}", new_prefix, inner_key);
                            stringify_dictionary(inner_nested, &inner_prefix, destination)?;
                        }
                        Node::List(inner_items) if inner_items.iter().all(|item| matches!(item, Node::Dictionary(_))) => {
                            for inner_item in inner_items {
                                if let Node::Dictionary(deepest) = inner_item {
                                    let inner_prefix = format!("{}.{}", new_prefix, inner_key);
                                    stringify_dictionary(deepest, &inner_prefix, destination)?;
                                }
                            }
                        }
                        _ => {
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn calculate_prefix(prefix: &str, key: &String) -> String {
    let new_prefix = if prefix.is_empty() {
        key.to_string()
    } else {
        format!("{}.{}", prefix, key)
    };
    new_prefix
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
        assert_eq!(destination.to_string(), "key = \"test\"\n");
    }

    #[test]
    fn test_stringify_integer() {
        let mut destination = BufferDestination::new();
        let mut dict = std::collections::HashMap::new();
        dict.insert("key".to_string(), make_node(42));
        let node = make_node(dict);
        stringify(&node, &mut destination).unwrap();
        assert_eq!(destination.to_string(), "key = 42\n");
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
        assert_eq!(destination.to_string(), "key = [1, 2, 3]\n");
    }

    #[test]
    fn test_stringify_dictionary() {
        let mut destination = BufferDestination::new();
        let mut dict = std::collections::HashMap::new();
        dict.insert("key".to_string(), make_node("value"));
        let node = make_node(dict);
        stringify(&node, &mut destination).unwrap();
        assert_eq!(destination.to_string(), "key = \"value\"\n");
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
            "[outer_key]\ninner_key = \"inner_value\"\n"
        );
    }

    #[test]
    fn test_stringify_none() {
        let mut destination = BufferDestination::new();
        let mut dict = std::collections::HashMap::new();
        dict.insert("key".to_string(), Node::None);
        let node = make_node(dict);
        stringify(&node, &mut destination).unwrap();
        assert_eq!(destination.to_string(), "key = null\n");
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
            "[level1.level2.level3]\ndeep_key = 123\n"
        );
    }
    #[test]
    fn test_stringify_empty_dictionary() {
        let mut dest = BufferDestination::new();
        stringify(&Node::Dictionary(HashMap::new()), &mut dest).unwrap();
        assert_eq!(dest.to_string(), "");
    }

    #[test]
    fn test_heterogeneous_list() {
        let mut dest = BufferDestination::new();
        let mut dict = HashMap::new();
        dict.insert("key".to_string(),
                    make_node(vec![make_node(1), make_node("test")]));
        let result = stringify(&Node::Dictionary(dict), &mut dest);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "TOML Lists must contain elements of the same type");
    }

    #[test]
    fn test_array_table() {
        let mut dest = BufferDestination::new();
        let mut inner1 = HashMap::new();
        inner1.insert("name".to_string(), make_node("first"));
        let mut inner2 = HashMap::new();
        inner2.insert("name".to_string(), make_node("second"));

        let mut dict = HashMap::new();
        dict.insert("items".to_string(),
                    make_node(vec![make_node(inner1), make_node(inner2)]));

        stringify(&Node::Dictionary(dict), &mut dest).unwrap();
        assert_eq!(dest.to_string(),
                   "[[items]]\nname = \"first\"\n[[items]]\nname = \"second\"\n");
    }

    #[test]
    fn test_mixed_array_table() {
        let mut dest = BufferDestination::new();
        let mut inner = HashMap::new();
        inner.insert("simple".to_string(), make_node(42));
        let mut nested = HashMap::new();
        nested.insert("value".to_string(), make_node("test"));
        inner.insert("complex".to_string(), make_node(nested));

        let mut dict = HashMap::new();
        dict.insert("items".to_string(), make_node(vec![make_node(inner)]));

        stringify(&Node::Dictionary(dict), &mut dest).unwrap();
        assert_eq!(dest.to_string(),
                   "[[items]]\nsimple = 42\n[items.complex]\nvalue = \"test\"\n");
    }

    #[test]
    fn test_nested_array_tables() {
        let mut dest = BufferDestination::new();
        let mut deepest = HashMap::new();
        deepest.insert("value".to_string(), make_node(42));

        let mut inner = HashMap::new();
        inner.insert("nested".to_string(),
                     make_node(vec![make_node(deepest.clone())]));

        let mut dict = HashMap::new();
        dict.insert("items".to_string(), make_node(vec![make_node(inner)]));

        stringify(&Node::Dictionary(dict), &mut dest).unwrap();
        assert_eq!(dest.to_string(),
                   "[[items]]\n[items.nested]\nvalue = 42\n");
    }
}

