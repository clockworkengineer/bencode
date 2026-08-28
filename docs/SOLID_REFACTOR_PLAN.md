# SOLID Architectural Refactor Plan for `bencode`

This document outlines a concrete, phased refactoring plan to transform the `bencode` library into a clean, fully SOLID-compliant codebase.

---

## Executive Summary of SOLID Violations & Target Architecture

| SOLID Principle | Current Architectural Violation | Proposed Solution |
| :--- | :--- | :--- |
| **Single Responsibility (SRP)** | `Node` handles AST data representation, type conversion, collection mutation, display formatting, AND dictionary schema validation. Serializers contain duplicate number formatting and raw buffer manipulation. | Segregate schema validation into `NodeValidator` / `NodeQueryExt`. Delegate serialization & display to visitor/formatter abstractions. Keep `Node` strictly an AST data container. |
| **Open / Closed (OCP)** | Format converters (`json`, `yaml`, `xml`, `toml`) match directly on concrete `Node` enum. Adding a new format or supporting `BorrowedNode` requires rewriting output logic. | Introduce `BencodeVisitor` / `BencodeSerializer` traits. AST nodes implement `AcceptVisitor`. New formats simply implement `BencodeVisitor` without altering AST or core engines. |
| **Liskov Substitution (LSP)** | `ISource::reset()` breaks non-rewindable streams. `IDestination::clear()` and `last()` break unbuffered file/socket output. `IDestination::add_bytes(&str)` assumes UTF-8 strings for binary data. | Segregate `reset()`, `clear()`, and `last()` into optional extension traits (`RewindableSource`, `BufferedDestination`). Change byte slice parameter from `&str` to `&[u8]`. |
| **Interface Segregation (ISP)** | Readers and writers are forced to implement fat traits (`ISource`, `IDestination`) with unnecessary methods. `Node` enum is bloated with schema helper methods. | Split I/O into lean, role-focused traits (`BencodeRead`, `BencodeWrite`). Extract dictionary schema query helpers into `NodeQueryExt`. |
| **Dependency Inversion (DIP)** | High-level APIs (`parse_bytes`, `stringify_to_string`) and format converters depend on concrete `BufferSource` / `BufferDestination` and concrete `Node` enum. | High-level APIs depend strictly on `BencodeRead` and `BencodeWrite` trait abstractions. Serializers depend on `BencodeVisitor` rather than concrete `Node` types. |

---

## Technical Refactoring Details by Component

### Component 1: I/O Module (`io`)
- **[traits.rs](file:///c:/Projects/bencode/library/src/io/traits.rs)**
  - Define `BencodeRead`: `peek_byte(&mut self) -> Option<u8>`, `read_byte(&mut self) -> Option<u8>`, `has_more(&mut self) -> bool`.
  - Define `BencodeWrite`: `write_byte(&mut self, byte: u8)`, `write_bytes(&mut self, bytes: &[u8])`.
  - Extension traits: `RewindableRead` (`reset(&mut self)`), `BufferedWrite` (`clear(&mut self)`, `last_byte(&self) -> Option<u8>`).
  - Maintain `ISource` & `IDestination` as backward-compatible trait aliases.

### Component 2: AST Nodes (`nodes`)
- **[node.rs](file:///c:/Projects/bencode/library/src/nodes/node.rs)**
  - Retain core `Node` enum variants and basic type constructors (`From<T>`).
- **[query.rs](file:///c:/Projects/bencode/library/src/nodes/query.rs)**
  - Extract schema querying and field validation methods (`get_int_required`, `get_string_optional`, etc.) into `NodeQueryExt`.

### Component 3: Visitors & Format Serializers (`stringify`)
- **[visitor.rs](file:///c:/Projects/bencode/library/src/stringify/visitor.rs)**
  - Trait `BencodeVisitor` for format serialization.
  - Trait `BencodeVisitable` for AST traversing (`Node`, `BorrowedNode`).
- **[default.rs](file:///c:/Projects/bencode/library/src/stringify/default.rs)**, **[json.rs](file:///c:/Projects/bencode/library/src/stringify/json.rs)**, **[yaml.rs](file:///c:/Projects/bencode/library/src/stringify/yaml.rs)**, **[xml.rs](file:///c:/Projects/bencode/library/src/stringify/xml.rs)**, **[toml.rs](file:///c:/Projects/bencode/library/src/stringify/toml.rs)**
  - Re-implement format serializations against `BencodeVisitor`.

### Component 4: High-Level Public API Facades (`lib.rs` & `misc`)
- **[lib.rs](file:///c:/Projects/bencode/library/src/lib.rs)**
  - Expose new SOLID trait abstractions (`BencodeRead`, `BencodeWrite`, `BencodeVisitor`, `NodeQueryExt`).
