use bencode_lib::{FileSource, Node, parse};
use std::path::Path;
use std::fs;

#[derive(Debug)]
struct FileDetails  {
    path: String,
    length: u64,
}
#[derive(Debug)]
struct TorrentFile {
    announce: String,
    announce_list: Vec<String>,
    encoding: String,
    attribute: u64,
    comment: String,
    creation_date: u64,
    created_by: String,
    length: u64,
    name: String,
    piece_length:u64,
    pieces: String,
}

fn read_torrent_file(path: &Path) -> Result<TorrentFile, String> {
    match FileSource::new(path.to_str().unwrap()) {
        Ok(mut file) => match parse(&mut file) {
            Ok(Node::Dictionary(dict)) => {
                let announce = match dict.get("announce") {
                    Some(Node::Str(s)) => s.clone(),
                    _ => return Err("Missing or invalid announce URL".to_string()),
                };
                
                let announce_list = match dict.get("announce-list") {
                    Some(Node::List(list)) => {
                        list.iter()
                            .filter_map(|item| match item {
                                Node::List(sublist) => sublist.first().and_then(|url| match url {
                                    Node::Str(s) => Some(s.clone()),
                                    _ => None,
                                }),
                                _ => None,
                            })
                            .collect()
                    }
                    _ => Vec::new(),
                };

                let encoding = match dict.get("encoding") {
                    Some(Node::Str(s)) => s.clone(),
                    _ => "UTF-8".to_string(),
                };

                let comment = match dict.get("comment") {
                    Some(Node::Str(s)) => s.clone(),
                    _ => String::new(),
                };

                let creation_date = match dict.get("creation date") {
                    Some(Node::Integer(n)) => *n as u64,
                    _ => 0,
                };

                let created_by = match dict.get("created by") {
                    Some(Node::Str(s)) => s.clone(),
                    _ => String::new(),
                };

                let attribute = if dict.contains_key("info") {
                    match dict.get("info").unwrap() {
                        Node::Dictionary(info_dict) => {
                            match info_dict.get("attribute") {
                                Some(Node::Integer(n)) => *n as u64,
                                _ => 0,
                            }
                        },
                        _ => 0,
                    }
                } else {
                    0
                };

                let length = if dict.contains_key("info") {
                    match dict.get("info").unwrap() {
                        Node::Dictionary(info_dict) => {
                            match info_dict.get("length") {
                                Some(Node::Integer(n)) => *n as u64,
                                _ => 0,
                            }
                        },
                        _ => 0,
                    }
                } else {
                    0
                };

                let name:String = if dict.contains_key("info") {
                    match dict.get("info").unwrap() {
                        Node::Dictionary(info_dict) => {
                            match info_dict.get("name") {
                                Some(Node::Str(name)) => name.clone(),
                                _ => String::from(""),
                            }
                        },
                        _ => String::from(""),
                    }
                } else {
                    String::from("")
                };

                let piece_length = if dict.contains_key("info") {
                    match dict.get("info").unwrap() {
                        Node::Dictionary(info_dict) => {
                            match info_dict.get("piece length") {
                                Some(Node::Integer(n)) => *n as u64,
                                _ => 0,
                            }
                        },
                        _ => 0,
                    }
                } else {
                    0
                };

                let pieces = if dict.contains_key("info") {
                    match dict.get("info").unwrap() {
                        Node::Dictionary(info_dict) => {
                            match info_dict.get("pieces") {
                                Some(Node::Str(s)) => s.clone(),
                                _ => String::new(),
                            }
                        },
                        _ => String::new(),
                    }
                } else {
                    String::new()
                };

                Ok(TorrentFile {
                    announce,
                    announce_list,
                    encoding,
                    attribute,
                    comment,
                    creation_date,
                    created_by,
                    length,
                    name,
                    piece_length,
                    pieces,
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
                        println!("Announce URL: {}", torrent.announce);
                        println!("Announce List URLs:");
                        for url in &torrent.announce_list {
                            println!("  - {}", url);
                        }
                        println!("Encoding: {}", torrent.encoding);
                        println!("Attribute: {}", torrent.attribute);
                        println!("Comment: {}", torrent.comment);
                        println!("Creation Date: {}", torrent.creation_date);
                        println!("Created By: {}", torrent.created_by);
                        println!("Length: {} bytes", torrent.length);
                        println!("Name: {}", torrent.name);
                        println!("Piece Length: {}", torrent.piece_length);
                        // println!("Pieces: {}", torrent.pieces);

                    }
                    Err(e) => eprintln!("Error reading torrent file: {}", e),
                }
            }
        }
    }
}