# bencode_lib

A Rust library for parsing, constructing, and converting Bencode data. In addition to round‑tripping Bencode, it can render parsed data to JSON, YAML, and XML.

Bencode is a compact serialization format commonly used by BitTorrent. It supports four types: integers, byte strings, lists, and dictionaries.

## Features

- Parse Bencode into a typed tree (Node)
- Serialize Node back to canonical Bencode
- Convert Node to JSON, YAML, or XML
- File and in‑memory sources/destinations
- Small and focused API

## Installation

Add to your Cargo.toml:

- If published on crates.io:
