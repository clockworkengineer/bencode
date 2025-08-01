use std::fs::File as StdFile;
use std::io::Write;
use crate::bencode_lib::io::traits::IDestination;

pub struct File {
    file: StdFile,
    path: String,
}

impl File {
    pub fn new(path: &str) -> std::io::Result<Self> {
        Ok(Self {
            file: StdFile::create(path)?,
            path: path.to_string(),
        })
    }
}

impl IDestination for File {
    fn add_byte(&mut self, b: u8) {
        self.file.write_all(&[b]).unwrap();
    }

    fn add_bytes(&mut self, s: &str) {
        self.file.write_all(s.as_bytes()).unwrap();
    }

    fn clear(&mut self) {
        self.file = StdFile::create(&self.path).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Read;

    #[test]
    fn create_file_destination_works() -> std::io::Result<()> {
        let path = "test_create.txt";
        let _file = File::new(path)?;
        assert!(fs::metadata(path).is_ok());
        fs::remove_file(path)?;
        Ok(())
    }

    #[test]
    fn add_byte_works() -> std::io::Result<()> {
        let path = "test_byte.txt";
        let mut file = File::new(path)?;
        file.add_byte(b'A');

        let mut content = String::new();
        StdFile::open(path)?.read_to_string(&mut content)?;
        assert_eq!(content, "A");

        fs::remove_file(path)?;
        Ok(())
    }

    #[test]
    fn add_bytes_works() -> std::io::Result<()> {
        let path = "test_bytes.txt";
        let mut file = File::new(path)?;
        file.add_bytes("test");

        let mut content = String::new();
        StdFile::open(path)?.read_to_string(&mut content)?;
        assert_eq!(content, "test");

        fs::remove_file(path)?;
        Ok(())
    }

    #[test]
    fn clear_works() -> std::io::Result<()> {
        let path = "test_clear.txt";
        let mut file = File::new(path)?;
        file.add_bytes("test");
        file.clear();

        let mut content = String::new();
        StdFile::open(path)?.read_to_string(&mut content)?;
        assert_eq!(content, "");

        fs::remove_file(path)?;
        Ok(())
    }
}

