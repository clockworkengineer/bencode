//! Borrowed/zero-copy node implementation for embedded systems.
//! This module provides a Node variant that holds references to the input buffer
//! instead of allocating and copying data, reducing memory usage.

#[cfg(not(feature = "std"))]
use alloc::collections::BTreeMap as HashMap;
#[cfg(feature = "std")]
use std::collections::HashMap;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use core::fmt;

/// A borrowed node that references data from the input buffer without allocation.
/// This is useful for embedded systems where memory is limited.
/// 
/// The lifetime parameter 'a represents the lifetime of the borrowed input data.
#[derive(Clone, Debug, PartialEq)]
pub enum BorrowedNode<'a> {
    /// Represents a 64-bit signed integer value
    Integer(i64),
    /// Represents a string value as a borrowed byte slice
    /// Note: bencode strings are byte strings, not necessarily UTF-8
    Bytes(&'a [u8]),
    /// Represents a list of other borrowed nodes
    List(Vec<BorrowedNode<'a>>),
    /// Represents a dictionary/map of byte string keys to borrowed node values
    Dictionary(HashMap<&'a [u8], BorrowedNode<'a>>),
}

impl<'a> BorrowedNode<'a> {
    /// Returns true if the node is an Integer variant
    pub fn is_integer(&self) -> bool {
        matches!(self, BorrowedNode::Integer(_))
    }

    /// Returns true if the node is a Bytes variant
    pub fn is_bytes(&self) -> bool {
        matches!(self, BorrowedNode::Bytes(_))
    }

    /// Returns true if the node is a List variant
    pub fn is_list(&self) -> bool {
        matches!(self, BorrowedNode::List(_))
    }

    /// Returns true if the node is a Dictionary variant
    pub fn is_dictionary(&self) -> bool {
        matches!(self, BorrowedNode::Dictionary(_))
    }

    /// Returns the integer value if this is an Integer node
    pub fn as_integer(&self) -> Option<i64> {
        match self {
            BorrowedNode::Integer(i) => Some(*i),
            _ => None,
        }
    }

    /// Returns the byte slice if this is a Bytes node
    pub fn as_bytes(&self) -> Option<&'a [u8]> {
        match self {
            BorrowedNode::Bytes(b) => Some(b),
            _ => None,
        }
    }

    /// Returns the list reference if this is a List node
    pub fn as_list(&self) -> Option<&Vec<BorrowedNode<'a>>> {
        match self {
            BorrowedNode::List(l) => Some(l),
            _ => None,
        }
    }

    /// Returns the dictionary reference if this is a Dictionary node
    pub fn as_dictionary(&self) -> Option<&HashMap<&'a [u8], BorrowedNode<'a>>> {
        match self {
            BorrowedNode::Dictionary(d) => Some(d),
            _ => None,
        }
    }
}

impl<'a> fmt::Display for BorrowedNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BorrowedNode::Integer(i) => write!(f, "{}", i),
            BorrowedNode::Bytes(b) => {
                // Try to display as UTF-8, fallback to debug format
                match core::str::from_utf8(b) {
                    Ok(s) => write!(f, "\"{}\"", s),
                    Err(_) => write!(f, "{:?}", b),
                }
            }
            BorrowedNode::List(list) => {
                write!(f, "[")?;
                for (i, item) in list.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            BorrowedNode::Dictionary(dict) => {
                write!(f, "{{")?;
                for (i, (key, value)) in dict.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    match core::str::from_utf8(key) {
                        Ok(s) => write!(f, "\"{}\": {}", s, value)?,
                        Err(_) => write!(f, "{:?}: {}", key, value)?,
                    }
                }
                write!(f, "}}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn borrowed_node_type_checks() {
        let int_node = BorrowedNode::Integer(42);
        assert!(int_node.is_integer());
        assert!(!int_node.is_bytes());
        assert!(!int_node.is_list());
        assert!(!int_node.is_dictionary());

        let bytes_node = BorrowedNode::Bytes(b"hello");
        assert!(!bytes_node.is_integer());
        assert!(bytes_node.is_bytes());
        assert!(!bytes_node.is_list());
        assert!(!bytes_node.is_dictionary());
    }

    #[test]
    fn borrowed_node_as_methods() {
        let int_node = BorrowedNode::Integer(42);
        assert_eq!(int_node.as_integer(), Some(42));
        assert_eq!(int_node.as_bytes(), None);

        let bytes_node = BorrowedNode::Bytes(b"test");
        assert_eq!(bytes_node.as_bytes(), Some(&b"test"[..]));
        assert_eq!(bytes_node.as_integer(), None);
    }

    #[test]
    fn borrowed_node_display() {
        let int_node = BorrowedNode::Integer(42);
        assert_eq!(format!("{}", int_node), "42");

        let bytes_node = BorrowedNode::Bytes(b"hello");
        assert_eq!(format!("{}", bytes_node), "\"hello\"");
    }
}
