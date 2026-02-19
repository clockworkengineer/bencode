# Bencode Library Examples

This directory contains comprehensive examples demonstrating the functionality of the bencode library. Each example focuses on specific features and use cases.

## Overview of Examples

### 1. **bencode_node_api** - Node API Usage
Demonstrates the Node API's type checking, accessor methods, and utility functions.

**Key Features:**
- Type checking methods (`is_integer()`, `is_string()`, `is_list()`, `is_dictionary()`, `is_none()`)
- Value accessors (`as_integer()`, `as_string()`, `as_list()`, `as_dictionary()`)
- Dictionary access methods (`get()`, `get_mut()`)
- Utility methods (`len()`, `is_empty()`, `type_name()`)
- Display trait formatting

**Run:**
```bash
cargo run --package bencode_node_api
```

## Library Installation
Add to your `Cargo.toml`:
```toml
[dependencies]
bencode_lib = { path = "library" }
```
Or use the published version:
```toml
[dependencies]
bencode_lib = "0.1.7"
```

## Quick examples

### 2. **bencode_in_memory** - In-Memory Operations
Shows how to work with bencode data in memory without file I/O.

**Key Features:**
- Convenience functions (`parse_bytes()`, `parse_str()`, `stringify_to_string()`, `stringify_to_bytes()`)
- BufferSource and BufferDestination usage
- Round-trip conversions
- Binary data handling

**Run:**
```bash
cargo run --package bencode_in_memory
```

### 3. **bencode_dictionary_ops** - Dictionary Manipulation
Comprehensive guide to creating, querying, and modifying dictionary nodes.

**Key Features:**
- Multiple dictionary creation methods
- Safe value querying
- Modifying dictionary contents
- Iterating through dictionaries
- Nested dictionary operations
- Building torrent metadata structures

**Run:**
```bash
cargo run --package bencode_dictionary_ops
```

### 4. **bencode_list_ops** - List Operations
Demonstrates list creation, modification, iteration, and transformations.

**Key Features:**
- Various list creation methods
- Element access patterns
- List modifications (push, insert, remove, clear)
- Filtering and iteration
- Transformations (map, filter, collect)
- Nested list operations
- Practical use cases (tracker lists, file paths, tags)

**Run:**
```bash
cargo run --package bencode_list_ops
```

### 5. **bencode_error_handling** - Error Handling
Shows proper error handling patterns when working with bencode data.

**Key Features:**
- Parsing error scenarios
- File I/O error handling
- Type mismatch handling
- Data validation strategies
- Error recovery patterns

**Run:**
```bash
cargo run --package bencode_error_handling
```

### 6. **bencode_format_conversions** - Format Conversions
Demonstrates converting bencode to various output formats.

**Key Features:**
- Conversion to JSON, TOML, XML, and YAML
- Simple and complex structure conversions
- Format size comparison
- Torrent metadata conversion
- Round-trip bencode conversions

**Run:**
```bash
cargo run --package bencode_format_conversions
```

### 7. **bencode_create_at_runtime** - Dynamic Structure Creation
Shows how to build complex bencode structures programmatically at runtime.

**Key Features:**
- Building nested dictionaries
- Creating complex torrent-like structures
- Multi-level nesting patterns

**Run:**
```bash
cargo run --package bencode_create_at_runtime
```

### 8. **bencode_fibonacci** - Stateful File Operations
A practical example maintaining a Fibonacci sequence in a bencode file.

**Key Features:**
- Reading bencode files
- Modifying data structures
- Writing back to files
- Stateful application pattern

**Run:**
```bash
cargo run --package bencode_fibonacci
```

### 9. **bencode_read_torrent_files** - Reading Torrent Files
Demonstrates parsing and displaying torrent file metadata.

**Key Features:**
- Opening .torrent files
- Parsing torrent metadata
- Extracting specific fields
- Error handling for malformed files

**Run:**
```bash
cargo run --package bencode_read_torrent_files
```

### 10. **bencode_torrent_to_json** - Torrent to JSON Conversion
Converts torrent files to JSON format.

**Run:**
```bash
cargo run --package bencode_torrent_to_json
```

### 11. **bencode_torrent_to_toml** - Torrent to TOML Conversion
Converts torrent files to TOML format.

**Run:**
```bash
cargo run --package bencode_torrent_to_toml
```

### 12. **bencode_torrent_to_xml** - Torrent to XML Conversion
Converts torrent files to XML format.

**Run:**
```bash
cargo run --package bencode_torrent_to_xml
```

### 13. **bencode_torrent_to_yaml** - Torrent to YAML Conversion
Converts torrent files to YAML format.

**Run:**
```bash
cargo run --package bencode_torrent_to_yaml
```

## Quick Start

To run all examples:

```bash
# From the root directory
cargo build --workspace

# Run a specific example
cargo run --package <example-name>
```

## Learning Path

If you're new to the library, we recommend going through the examples in this order:

1. **bencode_node_api** - Learn the fundamental Node API
2. **bencode_in_memory** - Understand parsing and stringifying
3. **bencode_dictionary_ops** - Master dictionary operations
4. **bencode_list_ops** - Master list operations
5. **bencode_error_handling** - Learn proper error handling
6. **bencode_format_conversions** - Explore output formats
7. **bencode_create_at_runtime** - Build complex structures
8. **bencode_fibonacci** - See a stateful application
9. **bencode_read_torrent_files** - Work with real torrent files

## Common Patterns

### Parsing Bencode Data

```rust
use bencode_lib::{parse_bytes, parse_str};

// From byte slice
let node = parse_bytes(b"i42e")?;

// From string slice
let node = parse_str("4:test")?;
```

### Creating Nodes

```rust
use bencode_lib::{Node, make_node};
use std::collections::HashMap;

// Integer
let int_node = Node::Integer(42);

// String
let str_node = Node::Str("hello".to_string());

// List using make_node
let list = make_node(vec![make_node(1), make_node(2)]);

// Dictionary
let mut dict = HashMap::new();
dict.insert("key".to_string(), make_node("value"));
let dict_node = Node::Dictionary(dict);
```

### Type Checking and Access

```rust
if node.is_integer() {
    if let Some(value) = node.as_integer() {
        println!("Integer: {}", value);
    }
}

if let Some(dict) = node.as_dictionary() {
    for (key, value) in dict {
        println!("{}: {}", key, value);
    }
}
```

### Converting to Different Formats

```rust
use bencode_lib::{to_json, to_yaml, to_xml, to_toml, BufferDestination};

let mut dest = BufferDestination::new();
to_json(&node, &mut dest);
println!("{}", String::from_utf8_lossy(&dest.buffer));
```


## Size & Performance Best Practices

- **Disable unused features**: In your Cargo.toml, use `default-features = false` and only enable what you need for minimal binary size.
- **Enable LTO and release builds**: Add `[profile.release] lto = true` and always build with `--release`.
- **Use zero-copy parsing**: Prefer `parse_borrowed()` for memory efficiency and speed.
- **Stack-allocated buffers**: Use `FixedSizeBuffer<N>` for predictable, compile-time checked stack allocation.
- **Iterative parsing**: Use `parse_iterative` for deeply nested data to avoid stack overflows.
- **Lightweight error handling**: Use `BencodeError` for no-heap, deterministic error handling in embedded/size-sensitive builds.
- **Memory pools/arenas**: Use `Arena` and `MemoryTracker` for predictable, batch allocation and memory tracking.

See the `REFRACTOR_PLAN.md` and the library README for more details and rationale.

- Use `make_node()` for convenient node creation
- Always check types before accessing values
- Use pattern matching for safe value extraction
- Remember that dictionaries are unordered in bencode
- BufferDestination is great for in-memory operations
- FileSource/FileDestination for file I/O

## Contributing

Feel free to add more examples! When creating new examples:

1. Create a new directory under `examples/`
2. Add a `Cargo.toml` with dependency on the bencode library
3. Document the example's purpose and key features
4. Add it to the workspace members in the root `Cargo.toml`
5. Update this README with a description

## Resources

- [Library Documentation](../library/README.md)
- [API Reference](../library/src/lib.rs)
- [Bencode Specification](https://en.wikipedia.org/wiki/Bencode)
