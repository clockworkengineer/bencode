//! This example demonstrates how to create a complex Bencode data structure at runtime.
//! It shows various nested structures commonly found in torrent files.

use bencode_lib::bencode_lib::nodes::node::Node;
use std::collections::HashMap;

/// Creates a complex Bencode tree structure that resembles a typical torrent file,
/// including announce information, file lists, and nested metadata.
fn main() {
    // Root dictionary
    let mut root = HashMap::new();

    // Simple string and integer nodes
    root.insert("announce".to_string(), Node::Str("udp://tracker.example.com:80".to_string()));
    root.insert("created by".to_string(), Node::Str("AI Assistant Example Generator".to_string()));
    root.insert("creation date".to_string(), Node::Integer(1_725_000_000));

    // Announce list (list of lists)
    let announce_list = Node::List(vec![
        Node::List(vec![
            Node::Str("udp://tracker.example.com:80".to_string()),
            Node::Str("http://tracker.example.com/announce".to_string()),
        ]),
        Node::List(vec![
            Node::Str("udp://backup-tracker.example.org:1337".to_string()),
        ]),
    ]);
    root.insert("announce-list".to_string(), announce_list);

    // Info dictionary (nested)
    root.insert("info".to_string(), build_info_dict());

    // Optional: top-level "url-list" as a list or single string
    root.insert(
        "url-list".to_string(),
        Node::List(vec![
            Node::Str("https://mirror1.example.org/file".to_string()),
            Node::Str("https://mirror2.example.org/file".to_string()),
        ]),
    );

    // Optional: top-level comment
    root.insert("comment".to_string(), Node::Str("Multi-file torrent with complex structure".to_string()));

    // Wrap into the root Node
    let tree = Node::Dictionary(root);

    // For demonstration, pretty-print the structure
    println!("{:#?}", tree);
}

/// Builds the 'info' dictionary section of the torrent structure.
/// Contains essential torrent metadata like name, piece length, and file information.
fn build_info_dict() -> Node {
    let mut info = HashMap::new();

    // Common fields
    info.insert("name".to_string(), Node::Str("example-project".to_string()));
    info.insert("piece length".to_string(), Node::Integer(256 * 1024)); // 256 KiB

    // Pieces: normally a raw byte string; here a placeholder hex-like string
    info.insert(
        "pieces".to_string(),
        Node::Str("abcdef012345...abcdef012345".to_string()),
    );

    // Private flag
    info.insert("private".to_string(), Node::Integer(1));

    // Multi-file mode: "files" is a list of dictionaries
    info.insert("files".to_string(), build_files_list());

    // Optional: 'meta' nested dictionary to show deeper nesting
    info.insert("meta".to_string(), build_meta_dict());

    Node::Dictionary(info)
}

/// Creates a list of file entries for a multi-file torrent.
/// Each file entry contains length, path, and optional attributes.
fn build_files_list() -> Node {
    let file1 = {
        let mut f = HashMap::new();
        f.insert("length".to_string(), Node::Integer(1_048_576)); // 1 MiB
        f.insert(
            "path".to_string(),
            Node::List(vec![
                Node::Str("src".to_string()),
                Node::Str("main.rs".to_string()),
            ]),
        );
        f.insert(
            "md5sum".to_string(),
            Node::Str("d41d8cd98f00b204e9800998ecf8427e".to_string()),
        );
        Node::Dictionary(f)
    };

    let file2 = {
        let mut f = HashMap::new();
        f.insert("length".to_string(), Node::Integer(2_621_440)); // 2.5 MiB
        f.insert(
            "path".to_string(),
            Node::List(vec![
                Node::Str("assets".to_string()),
                Node::Str("images".to_string()),
                Node::Str("logo.png".to_string()),
            ]),
        );
        // Demonstrate optional per-file attributes
        f.insert(
            "attr".to_string(),
            Node::Dictionary({
                let mut attr = HashMap::new();
                attr.insert("read_only".to_string(), Node::Integer(0));
                attr.insert("lang".to_string(), Node::Str("en-US".to_string()));
                attr
            }),
        );
        Node::Dictionary(f)
    };

    let file3 = {
        let mut f = HashMap::new();
        f.insert("length".to_string(), Node::Integer(512_000)); // ~500 KiB
        f.insert(
            "path".to_string(),
            Node::List(vec![
                Node::Str("docs".to_string()),
                Node::Str("guide.md".to_string()),
            ]),
        );
        Node::Dictionary(f)
    };

    Node::List(vec![file1, file2, file3])
}

/// Constructs a deeply nested metadata dictionary containing
/// author information, tags, and build configuration details.
fn build_meta_dict() -> Node {
    // Deeply nested structure to demonstrate complex trees:
    // meta -> authors (list of dicts), tags (list), build (dict -> list)
    let authors = Node::List(vec![
        Node::Dictionary({
            let mut a = HashMap::new();
            a.insert("name".to_string(), Node::Str("Alice".to_string()));
            a.insert("email".to_string(), Node::Str("alice@example.com".to_string()));
            a
        }),
        Node::Dictionary({
            let mut a = HashMap::new();
            a.insert("name".to_string(), Node::Str("Bob".to_string()));
            a.insert("email".to_string(), Node::Str("bob@example.com".to_string()));
            a
        }),
    ]);

    let tags = Node::List(vec![
        Node::Str("rust".to_string()),
        Node::Str("bencode".to_string()),
        Node::Str("example".to_string()),
    ]);

    let build = Node::Dictionary({
        let mut b = HashMap::new();
        b.insert(
            "targets".to_string(),
            Node::List(vec![
                Node::Str("x86_64-unknown-linux-gnu".to_string()),
                Node::Str("aarch64-apple-darwin".to_string()),
            ]),
        );
        b.insert(
            "features".to_string(),
            Node::List(vec![
                Node::Str("serde".to_string()),
                Node::Str("cli".to_string()),
            ]),
        );
        b
    });

    Node::Dictionary({
        let mut meta = HashMap::new();
        meta.insert("version".to_string(), Node::Str("1.2.3".to_string()));
        meta.insert("authors".to_string(), authors);
        meta.insert("tags".to_string(), tags);
        meta.insert("build".to_string(), build);
        meta
    })
}

