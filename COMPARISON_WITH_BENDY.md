# Comparison: bencode_lib vs bendy

This document compares our `bencode_lib` implementation with `bendy`, the most popular bencode library on crates.io (537K+ downloads).

## Feature Comparison

| Feature | bencode_lib | bendy | Winner |
|---------|-------------|-------|--------|
| **Core Functionality** |
| Basic encoding/decoding | ✅ | ✅ | Tie |
| no_std support | ✅ | ✅ | Tie |
| Canonicalization enforcement | ✅ (NEW) | ✅ | Tie |
| **Advanced Parsing** |
| Zero-copy parsing | ✅ `BorrowedNode` | ✅ | Tie |
| Iterative parser (no recursion) | ✅ `parse_iterative()` | ❌ | **bencode_lib** |
| Depth limiting | ✅ `ParserConfig` (NEW) | ✅ | Tie |
| **Error Handling** |
| String errors | ✅ | ✅ | Tie |
| Lightweight enum errors | ✅ `BencodeError` (4 bytes) | ❌ | **bencode_lib** |
| Error context/tracing | Partial | ✅ | bendy |
| **API Design** |
| Direct Node manipulation | ✅ | ✅ | Tie |
| Validation helpers | ✅ (NEW) | Partial | **bencode_lib** |
| Trait-based API | ❌ TODO | ✅ `ToBencode/FromBencode` | bendy |
| Derive macros | ❌ TODO | ❌ | Tie |
| **Embedded Systems** |
| Memory management tools | ✅ `Arena/StackBuffer` | ❌ | **bencode_lib** |
| Const generics | ✅ `MemoryBounds` | ❌ | **bencode_lib** |
| Memory tracking | ✅ `MemoryTracker` | ❌ | **bencode_lib** |
| Stack-based parsing | ✅ `parse_iterative()` | ❌ | **bencode_lib** |
| Binary size optimization | ✅ Optional features | ❌ | **bencode_lib** |
| **Format Conversion** |
| JSON | ✅ | ❌ | **bencode_lib** |
| TOML | ✅ | ❌ | **bencode_lib** |
| XML | ✅ | ❌ | **bencode_lib** |
| YAML | ✅ | ❌ | **bencode_lib** |
| **Integration** |
| Serde support | ❌ TODO | ✅ | bendy |
| Inspection/reflection API | ❌ | ✅ | bendy |
| **Configuration** |
| Parser configuration | ✅ `ParserConfig` (NEW) | ✅ | Tie |
| Encoder configuration | ✅ `EncoderConfig` (NEW) | ✅ | Tie |

## Recent Improvements (from bendy comparison)

### ✅ Implemented
1. **Validation helpers** - `get_required()`, `get_int_required()`, `get_string_required()`, etc.
2. **Configuration API** - `ParserConfig` and `EncoderConfig` for customization
3. **Depth limiting** - Built-in protection against stack overflow

### 🔄 In Progress
4. **Trait-based API** - `ToBencode` and `FromBencode` traits (TODO)
5. **Error context** - Path tracking for better error messages (TODO)

### 📋 Planned
6. **Derive macros** - `#[derive(ToBencode, FromBencode)]` (TODO)
7. **Serde integration** - Optional serde support (TODO)
8. **Benchmarks** - Performance comparison with bendy (TODO)

## Unique Advantages of bencode_lib

### For Embedded Systems ⭐⭐⭐
- **Memory Management**: `Arena`, `StackBuffer`, `MemoryTracker`
- **Const Generics**: Compile-time memory bounds checking
- **Iterative Parser**: No recursion, safe for limited stack space
- **Lightweight Errors**: 4-byte enum vs 24-byte String
- **Binary Size Control**: Optional format conversions save ~28KB

### For General Use ⭐⭐
- **Format Conversions**: Built-in JSON, TOML, XML, YAML support
- **Validation Helpers**: Ergonomic required/optional field extraction
- **Multiple Parser Options**: Default (recursive), iterative, zero-copy

### For Production ⭐
- **Configuration**: Fine-grained control over parsing/encoding behavior
- **Safety Features**: Depth limiting, canonicalization enforcement
- **Error Handling**: Multiple error types for different use cases

## API Examples

### bencode_lib Validation API (NEW)
```rust
// Clean, ergonomic validation
let name = node.get_string_required("name")?;
let age = node.get_int_required("age")?;
let email = node.get_string_optional("email");

// Compare with manual approach:
let name = node.get("name")
    .ok_or("Missing name")?
    .as_string()
    .ok_or("Name must be string")?;
```

### bendy Trait-based API
```rust
impl ToBencode for MyStruct {
    fn encode(&self, encoder: SingleItemEncoder) -> Result<(), Error> {
        encoder.emit_dict(|mut e| {
            e.emit_pair(b"name", &self.name)?;
            e.emit_pair(b"age", &self.age)
        })
    }
}
```

## Performance Characteristics

| Metric | bencode_lib | bendy |
|--------|-------------|-------|
| Parse speed | Fast | Fast |
| Encode speed | Fast | Fast |
| Memory usage (default) | Medium | Low (zero-copy) |
| Memory usage (borrowed) | **Minimal** | Low |
| Memory usage (iterative) | **Predictable** | Variable |
| Binary size (minimal) | **166 KB** | ~190 KB |
| Binary size (full) | 194 KB | ~210 KB |

## Use Case Recommendations

**Choose bencode_lib if you need:**
- ✅ Embedded systems support
- ✅ Format conversions (JSON/TOML/XML/YAML)
- ✅ Memory management control
- ✅ Iterative parsing for deep nesting
- ✅ Minimal binary size
- ✅ Lightweight error handling

**Choose bendy if you need:**
- ✅ Trait-based encoding/decoding
- ✅ Serde integration
- ✅ Reflection/inspection API
- ✅ Mature, battle-tested codebase
- ✅ Large community (537K downloads)

**Both libraries are excellent for:**
- ✅ no_std environments
- ✅ Zero-copy parsing
- ✅ Canonical bencode
- ✅ Production use

## Conclusion

**bencode_lib** excels in embedded systems and provides unique memory management features, format conversions, and binary size optimization. It's ideal for resource-constrained environments.

**bendy** provides a more ergonomic trait-based API, serde integration, and is the mature, community-trusted choice for general use.

Both libraries complement each other well - bencode_lib fills gaps in embedded/memory-constrained use cases that bendy doesn't target.
