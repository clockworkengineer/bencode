# Binary Size Comparison

This document compares binary sizes with different feature configurations to demonstrate the space savings from optional format conversions.

## Test Configurations

### bencode_minimal
- **Features**: `std` only (no format conversions)
- **Binary Size**: 169,984 bytes (166 KB)
- **Description**: Only bencode parsing and stringification
- **Use Case**: Embedded systems with strict size constraints

### bencode_full
- **Features**: `std`, `json`, `toml`, `xml`, `yaml` (all default features)
- **Binary Size**: 198,656 bytes (194 KB)
- **Description**: Complete library with all format conversions
- **Use Case**: Full-featured applications

## Size Savings

**Space Saved**: 28,672 bytes (28 KB)
**Reduction**: ~14.4%

By disabling unused format conversions, embedded systems can save significant flash memory. This is particularly important for microcontrollers with limited storage.

## Feature Flags

The library supports fine-grained control over which format converters are compiled:

```toml
# Minimal build - only bencode
bencode_lib = { version = "0.1.5", default-features = false, features = ["std"] }

# Only JSON support
bencode_lib = { version = "0.1.5", default-features = false, features = ["std", "json"] }

# Custom combination
bencode_lib = { version = "0.1.5", default-features = false, features = ["std", "json", "yaml"] }

# Full build (default)
bencode_lib = { version = "0.1.5" }
```

## Available Features

- `std` - Standard library support (enables file I/O)
- `json` - JSON format conversion
- `toml` - TOML format conversion
- `xml` - XML format conversion
- `yaml` - YAML format conversion

Default features: `["std", "json", "toml", "xml", "yaml"]`

## Build Commands

```bash
# Minimal build
cargo build --release -p bencode_minimal

# Full build
cargo build --release -p bencode_full

# Library with specific features
cargo build --release --no-default-features --features "std,json"
```

## Notes

- Sizes measured on Windows x64 with release optimizations
- Actual savings may vary by platform and optimization level
- no_std builds can achieve even smaller sizes by removing std dependency
- Consider using LTO (Link Time Optimization) for additional size reduction
