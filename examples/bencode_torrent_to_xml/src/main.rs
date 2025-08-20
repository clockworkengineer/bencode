use std::path::Path;
use bencode_lib::{FileSource, parse, FileDestination, to_xml};
use bencode_utility_lib::get_torrent_file_list;

fn process_torrent_file(file_path: &str) -> Result<(), String> {
    let mut source = FileSource::new(file_path).map_err(|e| e.to_string())?;
    let node = parse(&mut source).map_err(|e| e.to_string())?;
    let mut destination = FileDestination::new(Path::new(file_path).with_extension("xml").to_string_lossy().as_ref()).map_err(|e| e.to_string())?;
    to_xml(&node, &mut destination);
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

