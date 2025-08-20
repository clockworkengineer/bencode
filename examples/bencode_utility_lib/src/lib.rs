//! Utility library for handling torrent files and related operations.
//! Provides functionality for file system operations specific to torrent files.

use std::fs;
use std::path::Path;

/// Returns a list of torrent file paths from the specified directory.
///
/// # Arguments
///
/// * `file_path` - Path to the directory containing torrent files
///
/// # Returns
///
/// A vector of strings containing paths to all .torrent files in the directory
pub fn get_torrent_file_list(file_path: &str) -> Vec<String> {
    // Convert the input path string to a Path
    let files_dir = Path::new(file_path);
    // Create directory if it doesn't exist
    if !files_dir.exists() {
        fs::create_dir("files").expect("Failed to create files directory");
        return vec![];
    }

    // Read directory and collect all .torrent files
    fs::read_dir(files_dir)
        .expect("Failed to read directory")
        .filter_map(|entry| {
            // Extract entry and convert to path
            let entry = entry.ok()?;
            let path = entry.path();
            // Check if file has .torrent extension
            if path.extension()? == "torrent" {
                Some(path.to_string_lossy().into_owned())
            } else {
                None
            }
        })
        .collect()
}
