pub mod bencode_lib;

/// Returns the current version of the bencode library
pub use bencode_lib::misc::get_version as version;
/// Reads and parses a bencode-encoded file from disk
pub use bencode_lib::misc::read_bencode_file as read_file;
/// Writes bencode-encoded data to a file on disk
pub use bencode_lib::misc::write_bencode_file as write_file;

/// Source implementation for reading bencode data from a memory buffer
pub use bencode_lib::io::sources::buffer::Buffer as BufferSource;
/// Destination implementation for writing bencode data to a memory buffer
pub use bencode_lib::io::destinations::buffer::Buffer as BufferDestination;
/// Source implementation for reading bencode data from a file
pub use bencode_lib::io::sources::file::File as FileSource;
/// Destination implementation for writing bencode data to a file
pub use bencode_lib::io::destinations::file::File as FileDestination;

/// Core data structure representing a bencode node in the parsed tree
pub use bencode_lib::nodes::node::Node as Node;

/// Converts a Node tree back to bencode format
pub use bencode_lib::stringify::default::stringify as stringify;
/// Parses bencode data into a Node tree structure
pub use bencode_lib::parser::non_recursive::parse as parse;
/// Converts a Node tree to JSON format
pub use bencode_lib::stringify::json::stringify as to_json;
/// Converts a Node tree to YAML format
pub use bencode_lib::stringify::yaml::stringify as to_yaml;
/// Converts a Node tree to XML format
pub use bencode_lib::stringify::xml::stringify as to_xml;
