/// A Rust library for encoding and decoding data in the Bencode format, commonly used in BitTorrent files.
/// This library provides functionality to parse, create, modify and serialize bencode data structures
/// with support for various formats including JSON, YAML and XML conversion.

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
/// Module defining custom error types and error handling functionality
pub mod error;

///
/// Bencode_lib API
///

/// Returns the current version of the bencode library
pub use misc::get_version as version;
/// Reads and parses a bencode-encoded file from disc
pub use misc::read_bencode_file as read_file;
/// Writes bencode-encoded data to a file on disk
pub use misc::write_bencode_file as write_file;

/// Source implementation for reading bencode data from a memory buffer
pub use io::sources::buffer::Buffer as BufferSource;
/// Destination implementation for writing bencode data to a memory buffer
pub use io::destinations::buffer::Buffer as BufferDestination;
/// Source implementation for reading bencode data from a file
pub use io::sources::file::File as FileSource;
/// Destination implementation for writing bencode data to a file
pub use io::destinations::file::File as FileDestination;

/// Core data structure representing a bencode node in the parsed tree
pub use nodes::node::Node as Node;
pub use nodes::node::make_node as make_node;

/// Converts a Node tree back to bencode format
pub use stringify::default::stringify as stringify;
/// Parses bencode data into a Node tree structure
pub use parser::default::parse as parse;
/// Converts a Node tree to JSON format
pub use stringify::json::stringify as to_json;
/// Converts a Node tree to YAML format
pub use stringify::yaml::stringify as to_yaml;
/// Converts a Node tree to XML format
pub use stringify::xml::stringify as to_xml;
