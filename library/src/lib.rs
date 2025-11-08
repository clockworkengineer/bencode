/// Module defining custom error types and error handling functionality
pub mod error;
/// Module providing input/output operations for reading and writing bencode data
pub mod io;
/// Module containing utility functions and helper methods
pub mod misc;
/// A Rust library for encoding and decoding data in the Bencode format, commonly used in BitTorrent files.
/// This library provides functionality to parse, create, modify and serialize bencode data structures
/// with support for various formats including JSON, YAML and XML conversion.

/// Module defining the core data structures for representing bencode nodes
pub mod nodes;
/// Module containing the parsing logic to decode bencode format into data structures
pub mod parser;
/// Module implementing serialization of data structures back to bencode format
pub mod stringify;

///
/// Bencode_lib API
///

/// Returns the current version of the bencode library
pub use misc::get_version as version;
/// Reads and parses a bencode-encoded file from disc
pub use misc::read_bencode_file as read_file;
/// Writes bencode-encoded data to a file on disk
pub use misc::write_bencode_file as write_file;

/// Destination implementation for writing bencode data to a memory buffer
pub use io::destinations::buffer::Buffer as BufferDestination;
/// Destination implementation for writing bencode data to a file
pub use io::destinations::file::File as FileDestination;
/// Source implementation for reading bencode data from a memory buffer
pub use io::sources::buffer::Buffer as BufferSource;
/// Source implementation for reading bencode data from a file
pub use io::sources::file::File as FileSource;

/// Core data structure representing a bencode node in the parsed tree
pub use nodes::node::Node;
pub use nodes::node::make_node;

/// Parses bencode data into a Node tree structure
pub use parser::default::parse;
/// Parses bencode data from a byte slice into a Node tree structure
pub use parser::default::parse_bytes;
/// Parses bencode data from a string into a Node tree structure
pub use parser::default::parse_str;
/// Converts a Node tree back to bencode format
pub use stringify::default::stringify;
/// Converts a Node tree to bencode format as bytes
pub use stringify::default::stringify_to_bytes;
/// Converts a Node tree to bencode format as a String
pub use stringify::default::stringify_to_string;
/// Converts a Node tree to JSON format
pub use stringify::json::stringify as to_json;
/// Converts a Node tree to TOML format
pub use stringify::toml::stringify as to_toml;
/// Converts a Node tree to XML format
pub use stringify::xml::stringify as to_xml;
/// Converts a Node tree to YAML format
pub use stringify::yaml::stringify as to_yaml;
