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
    fn write_to_file_works() {
        let test_path = "test_output.txt";
        {
            let mut file = File::new(test_path).unwrap();
            file.add_bytes("test");
        }

        let mut content = String::new();
        StdFile::open(test_path).unwrap().read_to_string(&mut content).unwrap();
        assert_eq!(content, "test");

        fs::remove_file(test_path).unwrap();
    }
}