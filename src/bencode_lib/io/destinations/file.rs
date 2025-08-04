use std::fs::File as StdFile;
use std::io::{Write, Read, Seek};
use crate::bencode_lib::io::traits::IDestination;

pub struct File {
    file: StdFile,
    file_name: String,
    file_length: usize,
}

impl File {
    pub fn new(path: &str) -> std::io::Result<Self> {
        Ok(Self {
            file: StdFile::create(path)?,
            file_name: path.to_string(),
            file_length: 0
        })
    }

    pub fn file_length(&self) -> usize {
        self.file_length
    }
    pub fn file_name(&self) -> &str {
        &self.file_name.as_str()
    }
}

impl IDestination for File {
    fn add_byte(&mut self, b: u8) {
        self.file.write_all(&[b]).unwrap();
        self.file_length += 1
    }

    fn add_bytes(&mut self, s: &str) {
        self.file.write_all(s.as_bytes()).unwrap();
        self.file_length = self.file_length + s.len();
    }

    fn clear(&mut self) {
        self.file = StdFile::create(&self.file_name).unwrap();
        self.file_length = 0;
    }

    fn last(&self) -> Option<u8> {
        if self.file_length == 0 {
            None
        } else {
            let mut buf = vec![0];
            let mut file = StdFile::open(&self.file_name).unwrap();
            file.seek(std::io::SeekFrom::End(-1)).unwrap();
            file.read_exact(&mut buf).unwrap();
            Some(buf[0])
        }
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
    #[test]
    fn file_length_works() -> std::io::Result<()> {
        let path = "test_length.txt";
        let mut file = File::new(path)?;
        assert_eq!(file.file_length(), 0);

        file.add_byte(b'A');
        assert_eq!(file.file_length(), 1);

        file.add_bytes("test");
        assert_eq!(file.file_length(), 5);

        file.clear();
        assert_eq!(file.file_length(), 0);

        fs::remove_file(path)?;
        Ok(())
    }
    #[test]
    fn file_name_works() -> std::io::Result<()> {
        let path = "test_name.txt";
        let file = File::new(path)?;
        assert_eq!(file.file_name(), path);
        fs::remove_file(path)?;
        Ok(())
    }
    #[test]
    fn last_works() -> std::io::Result<()> {
        let path = "test_last.txt";
        let mut file = File::new(path)?;
        assert_eq!(file.last(), None);

        file.add_byte(b'1');
        assert_eq!(file.last(), Some(b'1'));

        file.add_byte(b'2');
        assert_eq!(file.last(), Some(b'2'));

        file.clear();
        assert_eq!(file.last(), None);

        fs::remove_file(path)?;
        Ok(())
    }
}

