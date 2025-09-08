/// Module implementing bencode data structure types and operations.
///
/// This module provides the core functionality for working with bencode format:
/// * Parsing raw bencode data into structured representations
/// * Manipulating bencode data structures in memory
/// * Serializing bencode structures back to their encoded form
///
/// Supports all bencode data types:
/// * Byte strings (length-prefixed)
/// * Integers
/// * Lists (ordered sequences)
/// * Dictionaries (key-value pairs)
pub mod node;