//! This program converts BitTorrent files from bencode format to YAML format.
//! It processes all .torrent files in the "files" directory and creates corresponding .yaml files.

use std::path::Path;
use bencode_lib::{FileSource, parse, FileDestination, to_yaml};
use bencode_utility_lib::get_torrent_file_list;

/// Converts a single torrent file from bencode to YAML format
///
/// # Arguments
///
/// * `file_path` - Path to the input .torrent file
///
/// # Returns
///
/// * `Ok(())` if conversion was successful
/// * `Err(String)` containing an error message if conversion failed
fn process_torrent_file(file_path: &str) -> Result<(), String> {
    // Open and read the source torrent file
    let mut source = FileSource::new(file_path).map_err(|e| e.to_string())?;
    // Parse the bencode data into an internal representation
    let node = parse(&mut source).map_err(|e| e.to_string())?;
    // Create output YAML file with same name but .yaml extension
    let mut destination = FileDestination::new(Path::new(file_path).with_extension("yaml").to_string_lossy().as_ref()).map_err(|e| e.to_string())?;
    // Convert and write the data in YAML format
    to_yaml(&node, &mut destination)?;
    Ok(())
}

fn main() {
    let torrent_files = get_torrent_file_list("files");
    for file_path in torrent_files {
        match process_torrent_file(&file_path) {
            Ok(()) => println!("Successfully converted {}", file_path),
            Err(e) => eprintln!("Failed to convert {}: {}", file_path, e),
        }
    }
}

