use torrent_file::TorrentFile;
use std::fs;

pub fn print_details(torrent: &TorrentFile) {
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
    println!("Pieces: {}", torrent.pieces.as_bytes().iter().map(|b| format!("{:02x}", b)).collect::<String>());
    println!("Private Bit Mask: {}", torrent.private_flag);
    println!("Source: {}", torrent.source);
    println!("Files:");
    for file in &torrent.files {
        println!("  - {} ({} bytes)", file.path, file.length);
    }
}

fn main() {
    let entries = fs::read_dir("files").unwrap();
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "torrent") {
                println!("\nProcessing {:?}:", path);
                match TorrentFile::from_file(&path) {
                    Ok(torrent) => {
                        print_details(&torrent);
                    }
                    Err(e) => eprintln!("Error reading torrent file: {}", e),
                }
            }
        }
    }
}