use std::fs;
use std::path::Path;
pub fn get_torrent_files() -> Vec<String> {
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
