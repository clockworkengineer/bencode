//! This module provides functionality to convert torrent files from bencode format to XML.
//! It processes files from a specified directory and creates corresponding XML outputs.

use std::path::Path;
use bencode_lib::{FileSource, parse, FileDestination, to_xml};
use bencode_utility_lib::get_torrent_file_list;

/// Converts a single torrent file from bencode format to XML format.
///
/// # Arguments
/// * `file_path` - Path to the input torrent file
///
/// # Returns
/// * `Ok(())` if conversion was successful
/// * `Err(String)` containing an error message if conversion failed
fn process_torrent_file(file_path: &str) -> Result<(), String> {
    // Create a file source for reading the torrent file
    let mut source = FileSource::new(file_path).map_err(|e| e.to_string())?;
    // Parse the bencode content into a node structure
    let node = parse(&mut source).map_err(|e| e.to_string())?;
    // Create a destination file with .xml extension
    let mut destination = FileDestination::new(Path::new(file_path).with_extension("xml").to_string_lossy().as_ref()).map_err(|e| e.to_string())?;
    // Convert the node structure to XML and write to destination
    to_xml(&node, &mut destination)?;
    Ok(())
}

fn main() {
    // Get a list of torrent files from the 'files' directory
    let torrent_files = get_torrent_file_list("files");
    // Process each torrent file and convert to XML
    for file_path in torrent_files {
        match process_torrent_file(&file_path) {
            Ok(()) => println!("Successfully converted {}", file_path),
            Err(e) => eprintln!("Failed to convert {}: {}", file_path, e),
        }
    }
}

