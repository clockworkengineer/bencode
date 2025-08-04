use crate::bencode_lib::io::traits::IDestination;
pub struct Buffer {
    pub buffer: Vec<u8>,
}

impl Buffer {
    pub fn new() -> Self {
        Self { buffer: vec![] }
    }
    pub fn to_string(&self) -> String {
        String::from_utf8_lossy(&self.buffer).into_owned()
    }
    pub fn last(&self) -> Option<u8> {
        self.buffer.last().copied()
    }
}

impl IDestination for Buffer {
    fn add_byte(&mut self, byte: u8) {
        self.buffer.push(byte);
    }
    fn add_bytes(&mut self, bytes: &str) {
        self.buffer.extend_from_slice(bytes.as_bytes());
    }
    fn clear(&mut self) {
        self.buffer.clear();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_byte_to_destination_buffer_works() {
        let mut destination = Buffer::new();
        destination.add_byte(b'i');
        destination.add_byte(b'3');
        destination.add_byte(b'2');
        destination.add_byte(b'e');
        assert_eq!(destination.to_string(), "i32e");
    }
    #[test]
    fn add_bytes_to_destination_buffer_works() {
        let mut destination = Buffer::new();
        destination.add_bytes("i3");
        assert_eq!(destination.to_string(), "i3");
        destination.add_bytes("2e");
        assert_eq!(destination.to_string(), "i32e");
    }
    #[test]
    fn clear_destination_buffer_works() {
        let mut destination = Buffer::new();
        destination.add_bytes("i32e");
        assert_eq!(destination.to_string(), "i32e");
        destination.clear();
        assert_eq!(destination.to_string(), "");
    }
    #[test]
    fn last_returns_none_for_empty_buffer() {
        let buffer = Buffer::new();
        assert_eq!(buffer.last(), None);
    }
    #[test]
    fn last_returns_last_byte_for_non_empty_buffer() {
        let mut buffer = Buffer::new();
        buffer.buffer.push(b'A');
        buffer.buffer.push(b'B');
        assert_eq!(buffer.last(), Some(b'B'));
    }
}