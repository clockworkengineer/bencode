//! Bencode library implementation providing encoding and decoding functionality
//! for the Bencode data format commonly used in BitTorrent applications.

/// Module defining the core data structures for representing bencode nodes
pub mod nodes;
/// Module providing input/output operations for reading and writing bencode data
pub mod io;
/// Module containing the parsing logic to decode bencode format into data structures
pub mod parser;
/// Module implementing serialization of data structures back to bencode format
pub mod stringify;
/// Module containing utility functions and helper methods
pub mod misc;