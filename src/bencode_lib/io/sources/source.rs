pub enum Source { Buffer(String), File(String)}

pub struct Buffer {  pub buffer : Vec<u8>}

impl Buffer {
    pub fn new(to_add: Vec<u8>) -> Self {
        Self { buffer : to_add }
    }
    pub fn add_vec(&self, to_add: Vec<u8>)  {
    }
    pub fn add_byte(&self, to_add: u8)  {
    }
}

#[cfg(test)]
mod tests {
    use super::Source;
    #[test]
    fn create_source_buffer_works() {
        let source = Source::Buffer(String::from("i32e"));
        match source {
            Source::Buffer(value) => {
                assert_eq!(value, "i32e");
            }
            _ => { assert_eq!(false, true); }
        }
    }
    #[test]
    fn create_add_byte_to_source_buffer_works() {
        let mut source = Source::Buffer(String::from("i32"));
        match source {
            _ => { assert_eq!(false, true); }
        }
    }
}