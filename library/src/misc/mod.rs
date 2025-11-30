/// Returns the current version of the package as specified in Cargo.toml.
/// Uses CARGO_PKG_VERSION environment variable that is set during compilation
/// from the version field in Cargo.toml.
pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

// File I/O functions are only available with std feature
#[cfg(feature = "std")]
mod file_io {
    use std::fs;
    use std::path::Path;

    /// Writes bencode string to a file
    ///
    /// # Arguments
    /// * `path` - The file path where the content will be written
    /// * `content` - The bencode string content to write to the file
    pub fn write_bencode_file(path: &str, content: &str) -> Result<(), std::io::Error> {
        fs::write(Path::new(path), content)
    }

    /// Reads bencode string from a file
    ///
    /// # Arguments
    /// * `path` - The file path to read from
    ///
    /// # Returns
    /// * `Ok(String)` - The content of the file as a string if successful
    /// * `Err(std::io::Error)` - The error if reading fails
    pub fn read_bencode_file(path: &str) -> Result<String, std::io::Error> {
        fs::read_to_string(Path::new(path))
    }
}

#[cfg(feature = "std")]
pub use file_io::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;

    #[test]
    fn test_get_version() {
        assert_eq!(get_version(), "0.1.7");
    }

    #[test]
    fn test_read_bencode_file_success() {
        let test_content = "d8:announce15:http://test.come";
        let test_file = "test.torrent";

        File::create(test_file)
            .and_then(|mut file| file.write_all(test_content.as_bytes()))
            .expect("Failed to create test file");

        let result = read_bencode_file(test_file);
        fs::remove_file(test_file).expect("Failed to cleanup test file");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), test_content);
    }

    #[test]
    fn test_read_bencode_file_error() {
        let result = read_bencode_file("nonexistent.torrent");
        assert!(result.is_err());
    }

    #[test]
    fn test_write_bencode_file() {
        let test_content = "d8:announce15:http://test.come";
        let test_file = "test_write.torrent";

        let write_result = write_bencode_file(test_file, test_content);
        assert!(write_result.is_ok());

        let read_result = read_bencode_file(test_file);
        fs::remove_file(test_file).expect("Failed to cleanup test file");

        assert!(read_result.is_ok());
        assert_eq!(read_result.unwrap(), test_content);
    }
}
