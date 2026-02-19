# Bencode Library: Concrete Refactor Plan for Size and Performance

## 1. Feature Minimization
- Audit all Cargo.toml files.
- Remove unused features and dependencies.
- Set `default-features = false` where possible.
- Document minimal build commands in README.

## 2. Zero-Copy Parsing
- Refactor all parsing in examples and library to use `parse_borrowed()` where possible.
- Update documentation to recommend zero-copy parsing for embedded/size-sensitive use.

## 3. Stack-Allocated Buffers
- Replace heap-allocated buffers with `FixedSizeBuffer<N>` in all relevant examples.
- Add compile-time assertions using `assert_buffer_size!`.
- Document buffer sizing best practices.

## 4. Iterative Parsing
- Use `parse_iterative` or `parse_bytes_iterative` for deeply nested data.
- Update examples to demonstrate iterative parsing.

## 5. Lightweight Error Handling
- Use `BencodeError` in embedded/size-constrained builds.
- Refactor error handling in examples to avoid heap allocation.

## 6. Memory Pool and Arena Allocators
- Use `Arena` and `MemoryTracker` for predictable allocation in examples and docs.
- Add example for batch deallocation and memory tracking.

## 7. General Rust Optimizations
- Enable LTO in `[profile.release]` in Cargo.toml.
- Always build with `--release` for production/embedded.
- Remove unused dependencies from all Cargo.toml files.

## 8. Documentation
- Update all READMEs to reflect new best practices.
- Add a section on size/performance tradeoffs and configuration.

---

## Implementation Steps
1. Audit and update Cargo.toml files (features, dependencies, LTO).
2. Refactor parsing in library and examples to use zero-copy and stack buffers.
3. Update error handling to use lightweight types.
4. Add/expand examples for iterative parsing, arena allocators, and memory tracking.
5. Update documentation and READMEs.
6. Test minimal and full builds for size and performance.
7. Review and merge changes.
