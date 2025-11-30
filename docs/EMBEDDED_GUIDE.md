# Embedded Systems Guide

This guide explains how to use bencode_lib in embedded and resource-constrained environments.

## Features for Embedded
- `no_std` support (enable via Cargo features)
- Zero-copy parsing
- Memory pool/arena allocation
- Lightweight error handling
- Stack-based iterative parser
- Const generics for compile-time configuration

## Usage Tips
- Enable `no_std` in `Cargo.toml`:
  ```toml
  [dependencies]
  bencode_lib = { version = "...", default-features = false, features = ["no_std"] }
  ```
- Use memory pool APIs for predictable allocation.
- Prefer lightweight error handling for small binaries.
- Use validation helpers for safe field extraction.

## Example
See `examples/bencode_minimal` and `examples/bencode_memory_pool` for embedded-friendly usage.
