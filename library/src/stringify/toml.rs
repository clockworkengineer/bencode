//! TOML stringification module provides functionality for converting Node structures into TOML format.
//!
//! This module implements conversion of various Node types into their TOML string representations:
//! - Objects are converted to TOML tables
//! - Arrays are converted to TOML arrays (must contain elements of the same type)
//! - Primitive values (strings, numbers, booleans) are converted to their TOML equivalents
//! - Nested structures are handled with proper table syntax
//! - Array tables are supported for collections of objects
//!
//! The module ensures compliance with TOML specification including
//! - Proper quoting of strings
//! - Correct table and array table syntax
//! - Type consistency in arrays
//! - Proper nesting of tables and sub-tables
//!

use crate::Node;
use crate::io::traits::IDestination;
use crate::stringify::common::escape_string;
use std::collections::{BTreeMap, HashMap};

/// Converts a Node structure to a TOML formatted string
///
/// # Anode` - The root Node to convert
/// * `destination` - The destination to write the TOML string to
///
/// # Returns
/// * `Ok(())` if successful
/// * `Err(String)` if the root node is not an Object
pub fn stringify(node: &Node, destination: &mut dyn IDestination) -> Result<(), String> {
    match node {
        Node::Dictionary(dict) => stringify_object(dict, "", destination),
        _ => Err("TOML format requires a dictionary at the root level".to_string()),
    }
}

/// Converts a Node value to its TOML string representation
///
/// # Arguments
/// * `value` - The Node to convert
/// * `add_cr` - Whether to add a carriage return after the value
/// * `destination` - The destination to write to
///
/// # Returns
/// * `Ok(())` if successful
/// * `Err(String)` if the array contains mixed types
fn stringify_value(
    value: &Node,
    add_cr: bool,
    destination: &mut dyn IDestination,
) -> Result<(), String> {
    match value {
        Node::Str(s) => stringify_str(s, destination),
        Node::Integer(value) => stringify_number(value, destination),
        Node::List(items) => stringify_array(items, destination)?,
        Node::None => destination.add_bytes("null"),
        Node::Dictionary(_) => return Ok(()), // Handled separately for table syntax
    }
    if add_cr {
        destination.add_bytes("\n");
    }
    Ok(())
}
/// Converts a string value to its TOML string representation with quotes
///
/// # Arguments
/// * `s` - The string to convert
/// * `destination` - The destination to write to
fn stringify_str(s: &str, destination: &mut dyn IDestination) {
    destination.add_bytes("\"");
    escape_string(s, destination);
    destination.add_bytes("\"");
}

/// Converts a numeric value to its TOML string representation
/// Handles different numeric types including integers, floats, and bytes
///
/// # Arguments
/// * `value` - The numeric value to convert
/// * `destination` - The destination to write to
fn stringify_number(value: &i64, destination: &mut dyn IDestination) {
    destination.add_bytes(&value.to_string())
}

/// Converts an array of Nodes to its TOML string representation
/// Ensures all array elements are of the same type as required by TOML spec
///
/// # Arguments
/// * `items` - The vector of Nodes to convert
/// * `destination` - The destination to write to
///
/// # Returns
/// * `Ok(())` if successful
/// * `Err(String)` if the array contains mixed types
fn stringify_array(items: &Vec<Node>, destination: &mut dyn IDestination) -> Result<(), String> {
    let first_type = get_node_type(&items[0]);

    for item in items {
        if get_node_type(item) != first_type {
            return Err("TOML lists must contain elements of the same type".to_string());
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

/// Returns the type of Node as a static string
/// Used for type checking in arrays
///
/// # Arguments
/// * `node` - The Node to get the type of
///
/// # Returns
/// A string representing the Node type
fn get_node_type(node: &Node) -> &'static str {
    match node {
        Node::Str(_) => "string",
        Node::Integer(_) => "integer",
        Node::List(_) => "list",
        Node::Dictionary(_) => "object",
        Node::None => "null",
    }
}
/// Converts a key-value pair to its TOML string representation
/// Handles table headers and nested structures
///
/// # Arguments
/// * `prefix` - The current table path prefix
/// * `destination` - The destination to write to
/// * `is_first` - Whether this is the first entry in a table
/// * `key` - The key of the pair
/// * `value` - The value Node
///
/// # Returns
/// * `Ok(())` if successful
fn stringify_key_value_pair(
    prefix: &str,
    destination: &mut dyn IDestination,
    is_first: &mut bool,
    key: &String,
    value: &Node,
) -> Result<(), String> {
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

/// Converts a HashMap representing a TOML table to its string representation
/// Handles nested tables, array tables, and maintains proper TOML formatting.
/// This function processes the input dictionary in multiple steps:
/// 1. Sorts key-value pairs for consistent output
/// 2. Processes simple key-value pairs first
/// 3. Handles nested tables
/// 4. Handles array tables
///
/// # Arguments
/// * `dict` - The HashMap to convert containing key-value pairs
/// * `prefix` - The current table path prefix for nested structures
/// * `destination` - The destination to write the formatted TOML output
///
/// # Returns
/// * `Ok(())` if conversion was successful
/// * `Err(String)` if an error occurred during conversion
fn stringify_object(
    dict: &std::collections::HashMap<String, Node>,
    prefix: &str,
    destination: &mut dyn IDestination,
) -> Result<(), String> {
    if dict.is_empty() {
        return Ok(());
    }

    let dict_sorted: BTreeMap<_, _> = dict.iter().collect();
    let (tables_dict, array_tables_dict) = get_tables_in_dict(dict);
    let tables: BTreeMap<_, _> = tables_dict.iter().map(|(k, v)| (k, *v)).collect();
    let array_tables: BTreeMap<_, _> = array_tables_dict.iter().map(|(k, v)| (k, *v)).collect();

    process_key_value_pairs(&dict_sorted, prefix, destination)?;
    process_nested_tables(&tables, prefix, destination)?;
    process_array_tables(&array_tables, prefix, destination)?;

    Ok(())
}
/// Gets tables and array tables from a dictionary by categorizing its entries
/// This helper function processes a dictionary and identifies table entries,
/// separating them into regular tables and array tables.
///
/// # Arguments
/// * `dict` - The dictionary to process
///
/// # Returns
/// A tuple containing:
/// * HashMap of regular tables
/// * HashMap of array tables
fn get_tables_in_dict(
    dict: &HashMap<String, Node>,
) -> (
    HashMap<String, &HashMap<String, Node>>,
    HashMap<String, &Vec<Node>>,
) {
    let mut tables = HashMap::new();
    let mut array_tables = HashMap::new();

    for (key, value) in dict {
        match value {
            Node::Dictionary(nested) => {
                tables.insert(key.clone(), nested);
            }
            Node::List(items) if items.iter().all(|item| matches!(item, Node::Dictionary(_))) => {
                array_tables.insert(key.clone(), items);
            }
            _ => {}
        }
    }

    (tables, array_tables)
}

/// Processes key-value pairs in a TOML structure by iterating through sorted dictionary entries
/// This function handles simple key-value pairs while skipping tables and array tables
/// that need special processing
///
/// # Arguments
/// * `dict_sorted` - BTreeMap containing sorted key-value pairs to process
/// * `prefix` - Current table path prefix for nested structures
/// * `destination` - Destination to write the formatted TOML output
/// * `is_first` - Mutable flag indicating if this is the first entry in current table
///
/// # Returns
/// * `Ok(())` if successful
/// * `Err(String)` if an error occurred during processing
fn process_key_value_pairs<'a>(
    dict_sorted: &BTreeMap<&'a String, &'a Node>,
    prefix: &str,
    destination: &mut dyn IDestination,
) -> Result<(), String> {
    for (key, value) in dict_sorted {
        match value {
            Node::Dictionary(_) => {
                continue;
            }
            Node::List(items) => {
                if items.iter().all(|item| matches!(item, Node::Dictionary(_))) {
                    continue;
                }
            }
            _ => {}
        }
        let mut is_first = true;
        stringify_key_value_pair(prefix, destination, &mut is_first, key, value)?;
    }
    Ok(())
}

/// Processes nested tables in a TOML structure, handling proper formatting and recursion
/// This function iterates through the sorted tables and processes each nested table
/// while maintaining proper TOML table hierarchy and formatting
///
/// # Arguments
/// * `tables` - BTreeMap containing the nested table structures to process
/// * `prefix` - Current table path prefix for nested structures
/// * `destination` - Destination to write the formatted TOML output
///
/// # Returns
/// * `Ok(())` if successful
/// * `Err(String)` if an error occurred during processing
fn process_nested_tables(
    tables: &BTreeMap<&String, &std::collections::HashMap<String, Node>>,
    prefix: &str,
    destination: &mut dyn IDestination,
) -> Result<(), String> {
    for (key, nested) in tables {
        let new_prefix = calculate_prefix(prefix, key);
        stringify_object(nested, &new_prefix, destination)?;
    }
    Ok(())
}

/// Processes array tables in a TOML structure, handling proper formatting and recursion
/// This function iterates through sorted array tables and processes each table entry
/// while maintaining proper TOML array table syntax and hierarchy
///
/// # Arguments
/// * `array_tables` - BTreeMap containing the array table structures to process
/// * `prefix` - Current table path prefix for nested structures
/// * `destination` - Destination to write the formatted TOML output
///
/// # Returns
/// * `Ok(())` if successful
/// * `Err(String)` if an error occurred during processing
fn process_array_tables(
    array_tables: &BTreeMap<&String, &Vec<Node>>,
    prefix: &str,
    destination: &mut dyn IDestination,
) -> Result<(), String> {
    for (key, items) in array_tables {
        for item in &**items {
            if let Node::Dictionary(nested) = item {
                let new_prefix = calculate_prefix(prefix, key);
                destination.add_bytes("[[");
                destination.add_bytes(&new_prefix);
                destination.add_bytes("]]\n");
                process_nested_array_table(nested, &new_prefix, destination)?;
            }
        }
    }
    Ok(())
}

/// Processes a nested array table by handling both simple values and nested objects
/// This function sorts the input HashMap and processes its contents in two phases:
/// 1. Processes simple key-value pairs (non-object types)
/// 2. Processes nested objects and arrays of objects
///
/// # Arguments
/// * `nested` - HashMap containing the nested array table key-value pairs
/// * `new_prefix` - Current table path prefix for the nested structure
/// * `destination` - Destination to write the formatted TOML output
///
/// # Returns
/// * `Ok(())` if successful
/// * `Err(String)` if an error occurred during processing
fn process_nested_array_table(
    nested: &std::collections::HashMap<String, Node>,
    new_prefix: &str,
    destination: &mut dyn IDestination,
) -> Result<(), String> {
    let nested_sorted: BTreeMap<_, _> = nested.iter().collect();
    process_simple_values(&nested_sorted, destination)?;
    process_nested_objects(&nested_sorted, new_prefix, destination)?;
    Ok(())
}

/// Processes simple (non-object, non-array) key-value pairs in a TOML structure
/// This function handles basic value types like strings, numbers, and booleans
///
/// # Arguments
/// * `nested_sorted` - BTreeMap containing sorted key-value pairs to process
/// * `destination` - Destination to write the formatted TOML output
///
/// # Returns
/// * `Ok(())` if successful
/// * `Err(String)` if an error occurred during processing
fn process_simple_values(
    nested_sorted: &BTreeMap<&String, &Node>,
    destination: &mut dyn IDestination,
) -> Result<(), String> {
    for (inner_key, inner_value) in nested_sorted {
        match inner_value {
            Node::Dictionary(_) => {}

            Node::Integer(_) | Node::Str(_) => {
                let mut is_first = true;
                stringify_key_value_pair("", destination, &mut is_first, inner_key, inner_value)?;
            }
            Node::List(items) => {
                if items
                    .iter()
                    .all(|item| matches!(item, Node::Integer(_) | Node::Str(_)))
                {
                    let mut is_first = true;
                    stringify_key_value_pair(
                        "",
                        destination,
                        &mut is_first,
                        inner_key,
                        inner_value,
                    )?;
                }
            }
            _ => {}
        }
    }
    Ok(())
}

/// Processes nested objects and array tables within a TOML structure
/// This function handles complex nested structures by recursively processing them
///
/// # Arguments
/// * `nested_sorted` - BTreeMap containing sorted key-value pairs with nested structures
/// * `new_prefix` - Current table path prefix for the nested structure
/// * `destination` - Destination to write the formatted TOML output
///
/// # Returns
/// * `Ok(())` if successful
/// * `Err(String)` if an error occurred during processing
fn process_nested_objects(
    nested_sorted: &BTreeMap<&String, &Node>,
    new_prefix: &str,
    destination: &mut dyn IDestination,
) -> Result<(), String> {
    for (inner_key, inner_value) in nested_sorted {
        match inner_value {
            Node::Dictionary(inner_nested) => {
                let inner_prefix = format!("{}.{}", new_prefix, inner_key);
                stringify_object(inner_nested, &inner_prefix, destination)?;
            }
            Node::List(inner_items)
                if inner_items
                    .iter()
                    .all(|item| matches!(item, Node::Dictionary(_))) =>
            {
                for inner_item in inner_items {
                    if let Node::Dictionary(deepest) = inner_item {
                        let inner_prefix = format!("{}.{}", new_prefix, inner_key);
                        stringify_object(&deepest, &inner_prefix, destination)?;
                    }
                }
            }
            _ => {}
        }
    }
    Ok(())
}

/// Calculates a new prefix for nested TOML tables by combining the current prefix with a key
///
/// # Arguments
/// * `prefix` - The current table path prefix. Empty string for root level
/// * `key` - The key to append to the prefix
///
/// # Returns
/// A new string containing the combined prefix path:
/// - If the prefix is empty, returns the key as-is
/// - If the prefix exists, returns "prefix.key"
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
    use crate::BufferDestination;
    use crate::nodes::node::make_node;
    use std::collections::HashMap;

    #[test]
    fn test_stringify_string() {
        let mut destination = BufferDestination::new();
        let mut dict = HashMap::new();
        dict.insert("key".to_string(), make_node("test"));
        let node = make_node(dict);
        stringify(&node, &mut destination).unwrap();
        assert_eq!(destination.to_string(), "key = \"test\"\n");
    }

    #[test]
    fn test_stringify_integer() {
        let mut destination = BufferDestination::new();
        let mut dict = HashMap::new();
        dict.insert("key".to_string(), make_node(42));
        let node = make_node(dict);
        stringify(&node, &mut destination).unwrap();
        assert_eq!(destination.to_string(), "key = 42\n");
    }

    #[test]
    fn test_stringify_list() {
        let mut destination = BufferDestination::new();
        let mut dict = HashMap::new();
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
        let mut dict = HashMap::new();
        dict.insert("key".to_string(), make_node("value"));
        let node = make_node(dict);
        stringify(&node, &mut destination).unwrap();
        assert_eq!(destination.to_string(), "key = \"value\"\n");
    }

    #[test]
    fn test_stringify_none() {
        let mut destination = BufferDestination::new();
        let mut dict = HashMap::new();
        dict.insert("key".to_string(), Node::None);
        let node = make_node(dict);
        stringify(&node, &mut destination).unwrap();
        assert_eq!(destination.to_string(), "key = null\n");
    }
}
