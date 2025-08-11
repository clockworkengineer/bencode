use bencode_lib::{parse, FileSource, Node};
use std::fs;
use std::path::Path;

#[derive(Debug)]
struct FileDetails {
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
    piece_length: u64,
    pieces: String,
    private_flag: u64,
    source: String,
    files: Vec<FileDetails>,
}

fn get_integer(dict: &std::collections::HashMap<String, Node>, key: &str) -> u64 {
    if let Some(Node::Integer(n)) = dict.get(key) {
        return *n as u64;
    }
    0
}

fn get_string(dict: &std::collections::HashMap<String, Node>, key: &str) -> String {
    if let Some(Node::Str(s)) = dict.get(key) {
        return s.clone();
    }
    String::new()
}

fn get_info_integer(dict: &std::collections::HashMap<String, Node>, key: &str) -> u64 {
    if let Some(Node::Dictionary(info_dict)) = dict.get("info") {
        if let Some(Node::Integer(n)) = info_dict.get(key) {
            return *n as u64;
        }
    }
    0
}

fn get_info_string(dict: &std::collections::HashMap<String, Node>, key: &str) -> String {
    if let Some(Node::Dictionary(info_dict)) = dict.get("info") {
        if let Some(Node::Str(s)) = info_dict.get(key) {
            return s.clone();
        }
    }
    String::new()
}

fn read_torrent_file(path: &Path) -> Result<TorrentFile, String> {
    match FileSource::new(path.to_str().unwrap()) {
        Ok(mut file) => match parse(&mut file) {
            Ok(Node::Dictionary(dict)) => {
                let announce = get_string(&dict, "announce");
                let announce_list = match dict.get("announce-list") {
                    Some(Node::List(list)) => list
                        .iter()
                        .filter_map(|item| match item {
                            Node::List(sublist) => sublist.first().and_then(|url| match url {
                                Node::Str(s) => Some(s.clone()),
                                _ => None,
                            }),
                            _ => None,
                        })
                        .collect(),
                    _ => Vec::new(),
                };

                let encoding = match dict.get("encoding") {
                    Some(Node::Str(s)) => s.clone(),
                    _ => "UTF-8".to_string(),
                };

                let comment = get_string(&dict, "comment");
                let creation_date = get_integer(&dict, "creation date");
                let created_by = get_string(&dict, "created by");
                let attribute = get_info_integer(&dict, "attribute");
                let length = get_info_integer(&dict, "length");
                let name = get_info_string(&dict, "name");
                let piece_length = get_info_integer(&dict, "piece length");
                let pieces = get_info_string(&dict, "pieces");
                let private_flag = get_info_integer(&dict, "private");
                let source = get_info_string(&dict, "source");

                let files = if let Some(Node::Dictionary(info_dict)) = dict.get("info") {
                    if let Some(Node::List(files_list)) = info_dict.get("files") {
                        files_list
                            .iter()
                            .filter_map(|file| {
                                if let Node::Dictionary(file_dict) = file {
                                    let length = get_integer(file_dict, "length");
                                    let path = match file_dict.get("path") {
                                        Some(Node::List(path_list)) => path_list
                                            .iter()
                                            .filter_map(|p| match p {
                                                Node::Str(s) => Some(s.clone()),
                                                _ => None,
                                            })
                                            .collect::<Vec<String>>()
                                            .join("/"),
                                        _ => return None,
                                    };
                                    Some(FileDetails { path, length })
                                } else {
                                    None
                                }
                            })
                            .collect()
                    } else {
                        Vec::new()
                    }
                } else {
                    Vec::new()
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
                    private_flag,
                    source,
                    files,
                })
            }
            Err(s)=> Err(s),
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
                        println!("Private Bit Mask: {}", torrent.private_flag);
                        println!("Source: {}", torrent.source);
                        println!("Files:");
                        for file in &torrent.files {
                            println!("  - {} ({} bytes)", file.path, file.length);
                        }
                    }
                    Err(e) => eprintln!("Error reading torrent file: {}", e),
                }
            }
        }
    }
}
