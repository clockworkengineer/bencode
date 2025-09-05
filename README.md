# bencode_lib

A Rust library for parsing, constructing, and converting Bencode data. In addition to round‑tripping Bencode, it can render parsed data to JSON, YAML, and XML.

Bencode is a compact serialization format commonly used by BitTorrent. It supports four types: integers, byte strings, lists, and dictionaries.

## Features

- Parse Bencode into a typed tree (`Node`)
- Serialize `Node` back to canonical Bencode
- Convert `Node` to JSON, YAML, or XML
- Read/write from files or in‑memory buffers
- Small, focused API

## Installation

Add to your Cargo.toml:

- If published on crates.io:
```
toml
[dependencies]
bencode_lib = "0.1.0"
```
- Or as a workspace/path dependency:
```
toml
[dependencies]
bencode_lib = { path = "library" }
```
## Quick examples

Parse a `.torrent` (or any bencode) file and write it as YAML:
```
rust
use bencode_lib::{FileSource, FileDestination, parse, to_yaml};
use std::path::Path;

fn main() -> Result<(), String> {
    let input_path = "example.torrent";
    let output_path = Path::new(input_path).with_extension("yaml");

    // Read and parse bencode
    let mut src = FileSource::new(input_path).map_err(|e| e.to_string())?;
    let node = parse(&mut src)?;

    // Convert to YAML and write out
    let mut dst = FileDestination::new(output_path.to_string_lossy().as_ref()).map_err(|e| e.to_string())?;
    to_yaml(&node, &mut dst);
    Ok(())
}
```
Round‑trip a Bencode buffer:
```
rust
use bencode_lib::{BufferSource, BufferDestination, parse, stringify};

fn main() -> Result<(), String> {
let raw = b"d3:foo3:bar4:spamli1ei2ei3eee".to_vec();
let mut src = BufferSource::new(&raw);

    // Parse from memory
    let node = parse(&mut src)?;

    // Serialize back to bencode into memory
    let mut dst = BufferDestination::new();
    stringify(&node, &mut dst);

    // If your BufferDestination exposes the bytes, you can inspect them:
    // let bytes = dst.into_bytes();
    Ok(())
}
```
Construct a `Node` and render as JSON:
```
rust
use bencode_lib::{Node, to_json, BufferDestination};

fn main() {
let node = Node::Dict(vec![
(b"info".to_vec(), Node::List(vec![
Node::Integer(42),
Node::String(b"hello".to_vec()),
])),
]);

    let mut dst = BufferDestination::new();
    to_json(&node, &mut dst);
    let json = String::from_utf8(dst.into_bytes()).unwrap();
    println!("{}", json);
}
```
Read and write whole files with helpers:
```
rust
use bencode_lib::{read_file, write_file, Node};

fn main() -> Result<(), String> {
let node = read_file("input.bencode")?;
// ... mutate or inspect `node` ...
write_file("output.bencode", &node)?;
Ok(())
}
```
## Data model

- Integer: `Node::Integer(i64)`
- String (byte string): `Node::String(Vec<u8>)`
- List: `Node::List(Vec<Node>)`
- Dictionary: `Node::Dict(Vec<(Vec<u8>, Node)>)`
  - Keys are byte strings (raw bytes) to preserve exact data.

## API overview

- Sources/Destinations
  - `FileSource`, `FileDestination`
  - `BufferSource`, `BufferDestination`
- Core
  - `Node` — in‑memory representation of Bencode
  - `parse(&mut Source) -> Result<Node, String>`
  - `stringify(&Node, &mut Destination)`
- Converters
  - `to_json(&Node, &mut Destination)`
  - `to_yaml(&Node, &mut Destination)`
  - `to_xml(&Node, &mut Destination)`
- Utilities
  - `version() -> String` — library version
  - `read_file(path) -> Result<Node, String>`
  - `write_file(path, &Node) -> Result<(), String>`

## Error handling

- Parsing typically returns `Result<Node, String>`.
- File source/destination constructors return `Result<…, String>`.
- Serialization/conversion functions write into provided destinations.

## Minimum Supported Rust Version

- Rust 1.88.0

## License

This project is licensed under the MIT License. See the LICENSE file for details.
