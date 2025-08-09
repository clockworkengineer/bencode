use bencode_lib::{FileDestination, FileSource, Node, parse, stringify};
use std::path::Path;
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct TorrentFile {
    name: String,
    length: u64,
    piece_length: u64,
    pieces: Vec<u8>,
    announce: String,
    info_hash: Vec<u8>,
}

fn read_torrent_file(path: &Path) -> Result<TorrentFile, String> {
    match FileSource::new(path.to_str().unwrap()) {
        Ok(mut file) => match parse(&mut file) {
            Ok(Node::Dictionary(dict)) => {
                let announce = match dict.get("announce") {
                    Some(Node::Str(s)) => s.clone(),
                    _ => return Err("Missing or invalid announce URL".to_string()),
                };

                let info = match dict.get("info") {
                    Some(Node::Dictionary(info)) => info,
                    _ => return Err("Missing or invalid info dictionary".to_string()),
                };

                let name = match info.get("name") {
                    Some(Node::Str(s)) => s.clone(),
                    _ => return Err("Missing or invalid name".to_string()),
                };

                let length = match info.get("length") {
                    Some(Node::Integer(n)) => *n as u64,
                    _ => return Err("Missing or invalid length".to_string()),
                };

                let piece_length = match info.get("piece length") {
                    Some(Node::Integer(n)) => *n as u64,
                    _ => return Err("Missing or invalid piece length".to_string()),
                };

                let pieces = match info.get("pieces") {
                    Some(Node::Str(s)) => s.as_bytes().to_vec(),
                    _ => return Err("Missing or invalid pieces".to_string()),
                };

                Ok(TorrentFile {
                    name,
                    length,
                    piece_length,
                    pieces,
                    announce,
                    info_hash: vec![], // In a real implementation, calculate SHA1 of info dict
                })
            }
            _ => Err("Invalid torrent file format".to_string()),
        },
        Err(e) => Err(format!("Failed to open file: {}", e)),
    }
}

fn main() {
    let entries = fs::read_dir("files").unwrap();
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "torrent") {
                println!("\nProcessing {:?}:", path);
                match read_torrent_file(&path) {
                    Ok(torrent) => {
                        println!("Successfully parsed torrent file:");
                        println!("Name: {}", torrent.name);
                        println!("Length: {} bytes", torrent.length);
                        println!("Piece Length: {} bytes", torrent.piece_length);
                        println!("Announce URL: {}", torrent.announce);
                    }
                    Err(e) => eprintln!("Error reading torrent file: {}", e),
                }
            }
        }
    }
}