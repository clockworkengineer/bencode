
pub struct Buffer {  pub buffer : Vec<u8>}

impl Buffer {
    pub fn new(to_add: &[u8]) -> Self {
        Self { buffer : to_add.to_vec() }
    }
    // pub fn add_vec(&mut self, to_add: Vec<u8>) {
    // }
    // pub fn add_byte(&mut self, to_add: u8) {
    // }
    pub fn to_string(&self) -> String{
        String::from_utf8_lossy(&self.buffer).into_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::Buffer;
    #[test]
    fn create_source_buffer_works() {
        let  source = Buffer::new(String::from("i32e").as_bytes());
        assert_eq!(source.to_string(), "i32e");
    }
}