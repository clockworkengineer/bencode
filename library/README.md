# bencode_lib

A Rust library for parsing, constructing, and converting Bencode data. Designed for embedded systems, resource-constrained environments, and general-purpose use. Supports round-tripping Bencode and conversion to JSON, YAML, XML, and TOML.

## Features

- Parse Bencode into a typed tree (`Node`)
- Serialize `Node` back to canonical Bencode
- Convert `Node` to JSON, YAML, XML, or TOML
- Zero-copy and no_std support for embedded
- Memory pool/arena allocation
- Lightweight error handling
- Stack-based iterative parser for deep nesting
- Validation helpers for ergonomic field extraction
- Configurable parsing and encoding
- Read/write from files or in‑memory buffers

## Installation

Add to your `Cargo.toml`:

- If published on crates.io:
```
toml
[dependencies]
bencode_lib = "0.1.5"
```
- Or as a workspace/path dependency:
```
toml
[dependencies]
bencode_lib = { path = "library" }
```

## Quick Examples

Parse a `.torrent` file and write as YAML:
```rust
use bencode_lib::{FileSource, FileDestination, parse, to_yaml};
let mut src = FileSource::new("example.torrent")?;
let node = parse(&mut src)?;
let mut dst = FileDestination::new("example.yaml")?;
to_yaml(&node, &mut dst);
```
Round-trip a Bencode buffer:
```rust
use bencode_lib::{BufferSource, BufferDestination, parse, stringify};
let raw = b"d3:foo3:bar4:spamli1ei2ei3eee".to_vec();
let mut src = BufferSource::new(raw);
let node = parse(&mut src)?;
let mut dst = BufferDestination::new();
stringify(&node, &mut dst);
```
Construct a `Node` and render as JSON:
```rust
use bencode_lib::{Node, to_json, BufferDestination};
let node = Node::from_dict(vec![
    ("info", Node::from_list(vec![Node::from_int(42), Node::from_str("hello")]))
]);
let mut dst = BufferDestination::new();
to_json(&node, &mut dst);
println!("{}", dst.to_string());
```

## Data Model

- Integer: `Node::Integer(i64)`
- String: `Node::String(Vec<u8>)`
- List: `Node::List(Vec<Node>)`
- Dictionary: `Node::Dict(Vec<(Vec<u8>, Node)>)`

## API Overview

- Sources/Destinations: `FileSource`, `FileDestination`, `BufferSource`, `BufferDestination`
- Core: `Node`, `parse`, `stringify`
- Converters: `to_json`, `to_yaml`, `to_xml`, `to_toml`
- Utilities: `version`, `read_file`, `write_file`
- Embedded: `Arena`, `MemoryTracker`, `StackBuffer`, `BencodeError`, `parse_borrowed`, `validate_bencode`
- Config: `ParserConfig`, `EncoderConfig`

## Error Handling

- Parsing returns `Result<Node, BencodeError>`
- File operations return `Result<_, BencodeError>`
- Validation helpers for required/optional fields

## Minimum Supported Rust Version

- Rust 1.88.0

## Documentation

- See `docs/DEVELOPER_GUIDE.md` for contributor info
- See `docs/API_OVERVIEW.md` for API details
- See `docs/EMBEDDED_GUIDE.md` for embedded usage
- Example READMEs in `examples/`

## License

MIT License. See LICENSE file for details.
