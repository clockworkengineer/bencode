//! Example demonstrating conversion of torrent files from bencode format to JSON format.
//! Takes torrent files from the "files" directory and creates corresponding JSON files.

use std::path::Path;
use bencode_lib::{FileSource, parse, FileDestination, to_toml};
use bencode_utility_lib::get_torrent_file_list;

/// Converts a single torrent file from bencode format to JSON format
///
/// # Arguments
/// * `file_path` - Path to the input torrent file
///
/// # Returns
/// * `Ok(())` if conversion was successful
/// * `Err(String)` containing the error message if conversion failed
fn process_torrent_file(file_path: &str) -> Result<(), String> {
    // Create a source reader for the torrent file
    let mut source = FileSource::new(file_path).map_err(|e| e.to_string())?;
    // Parse the bencode data into an in-memory node structure
    let node = parse(&mut source).map_err(|e| e.to_string())?;
    // Create a destination writer for the JSON file with the same name but .json extension
    let mut destination = FileDestination::new(Path::new(file_path).with_extension("toml").to_string_lossy().as_ref()).map_err(|e| e.to_string())?;
    // Write the parsed data as JSON to the destination file
    to_toml(&node, &mut destination);
    Ok(())
}

/// Main function that processes all torrent files in the "files" directory
fn main() {
    // Get a list of torrent files from the "files" directory
    let torrent_files = get_torrent_file_list("files");
    // Process each torrent file
    for file_path in torrent_files {
        match process_torrent_file(&file_path) {
            Ok(()) => println!("Successfully converted {}", file_path),
            Err(e) => eprintln!("Failed to convert {}: {}", file_path, e),
        }
    }
}

