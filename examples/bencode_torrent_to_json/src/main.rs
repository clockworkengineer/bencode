use std::fs;
use std::path::Path;
use bencode_lib::{FileDestination, FileSource, Node, parse, stringify, BufferDestination, to_json};

fn get_torrent_files() -> Vec<String> {
    let files_dir = Path::new("files");
    if !files_dir.exists() {
        fs::create_dir("files").expect("Failed to create files directory");
        return vec![];
    }

    fs::read_dir(files_dir)
        .expect("Failed to read directory")
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()? == "torrent" {
                Some(path.to_string_lossy().into_owned())
            } else {
                None
            }
        })
        .collect()
}

fn process_torrent_file(file_path: &str) -> Result<(), String> {
    let mut source = FileSource::new(file_path).map_err(|e| e.to_string())?;
    let node = parse(&mut source).map_err(|e| e.to_string())?;

    let mut destination = BufferDestination::new();
    to_json(&node, &mut destination);

    let json_path = Path::new(file_path).with_extension("json");
    fs::write(json_path, destination.to_string())
        .map_err(|e| e.to_string())?;

    Ok(())
}

fn main() {
    let torrent_files = get_torrent_files();

    for file_path in torrent_files {
        match process_torrent_file(&file_path) {
            Ok(()) => println!("Successfully converted {}", file_path),
            Err(e) => eprintln!("Failed to convert {}: {}", file_path, e),
        }
    }
}
