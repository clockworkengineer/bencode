use crate::bencode_lib::io::traits::ISource;

pub struct Buffer {  buffer : Vec<u8>, position : usize}

impl Buffer {
    pub fn new(to_add: &[u8]) -> Self {
        Self { buffer : to_add.to_vec(), position: 0 }
    }
    pub fn to_string(&self) -> String{
        String::from_utf8_lossy(&self.buffer).into_owned()
    }
}

impl ISource for Buffer {
    fn next(&mut self) {
        self.position += 1;
    }
    fn current(&mut self) -> Option<char> {
        if self.more() {
            Some(self.buffer[self.position] as char)
        } else {
            None
        }
    }
    fn more(&mut self) -> bool {
        self.position < self.buffer.len()
    }
    fn reset(&mut self) {
        self.position = 0;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_source_buffer_works() {
        let  source = Buffer::new(String::from("i32e").as_bytes());
        assert_eq!(source.to_string(), "i32e");
    }
    #[test]
    fn read_character_from_source_buffer_works() {
        let  mut source = Buffer::new(String::from("i32e").as_bytes());
        match source.current() { Some('i') => assert!(true), _ => assert!(false)}
    }
    #[test]
    fn move_to_next_character_in_source_buffer_works() {
        let  mut source = Buffer::new(String::from("i32e").as_bytes());
        source.next();
        match source.current() { Some('3') => assert!(true), _ => assert!(false)}
    }
    #[test]
    fn move_to_last_character_in_source_buffer_works() {
        let  mut source = Buffer::new(String::from("i32e").as_bytes());
        while source.more() { source.next()}
        match source.current() { None => assert!(true), _ => assert!(false)}
    }
    #[test]
    fn reset_in_source_buffer_works() {
        let  mut source = Buffer::new(String::from("i32e").as_bytes());
        while source.more() { source.next()}
        source.reset();
        match source.current() { Some('i') => assert!(true), _ => assert!(false)}
    }
}