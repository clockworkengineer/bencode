# API Overview

This document provides a high-level overview of the bencode_lib API, including its main modules, types, and usage patterns.

## Main Modules
- `nodes`: Core bencode node types and utilities.
- `parser`: Bencode parsing logic.
- `stringify`: Bencode encoding and format conversions.
- `error`: Error types and handling strategies.
- `config`: Parser and encoder configuration structs.
- `memory`: Memory pool and arena allocation utilities.
- `io`: I/O helpers for reading/writing bencode data.

## Key Types
- `Node`: Represents a bencode value (int, string, list, dict).
- `ParserConfig`, `EncoderConfig`: Configuration for parsing/encoding.
- `BencodeError`: Lightweight error enum for embedded use.

## Usage Patterns
- Parse bencode data: `Node::parse(&[u8], &ParserConfig)`
- Encode bencode data: `Node::encode(&EncoderConfig)`
- Validate fields: `Node::get_required("key")`
- Convert formats: `Node::to_json()`, `Node::to_toml()`, etc.

## Example
```rust
let config = ParserConfig::default();
let node = Node::parse(&data, &config)?;
let value = node.get_required("key")?;
```

See individual module docs and example READMEs for more details.
