# Developer Guide for bencode_lib

This guide provides instructions and best practices for contributing to and maintaining the bencode_lib project.

## Project Structure
- `library/`: Core bencode library implementation.
- `examples/`: Demonstrations and usage examples.
- `docs/`: Documentation and guides.
- `files/`: Sample .torrent files for testing.

## Getting Started
1. **Clone the repository**
2. **Install Rust toolchain**: [https://rustup.rs/](https://rustup.rs/)
3. **Build the project**:
   ```sh
   cargo build --workspace
   ```
4. **Run tests**:
   ```sh
   cargo test --workspace
   ```
5. **Run examples**:
   ```sh
   cargo run --example <example_name>
   ```

## Coding Standards
- Use `rustfmt` for formatting.
- Run `clippy` for lint checks.
- Prefer zero-copy, no_std, and embedded-friendly patterns.
- Document all public APIs.

## Adding Features
- Open an issue or discuss in PR before major changes.
- Write tests for new features in `library/src/integration_tests/`.
- Update relevant example in `examples/`.

## Documentation
- Update `README.md` in root and relevant folders.
- Add new guides to `docs/`.
- Document public types and functions with Rustdoc comments.

## Testing
- Ensure all tests pass before submitting PRs.
- Add tests for edge cases and embedded scenarios.

## Release Process
- Update version in `Cargo.toml`.
- Document changes in `CHANGELOG.md` (add if missing).
- Tag release in git.

## Contact & Support
- For questions, open an issue or contact maintainers via GitHub.

---

For more details, see the API documentation and example READMEs.
