pub trait ISource {
    fn next(&mut self);
    fn current(&mut self) -> Option<char>;
    fn more(&mut self) -> bool;
    fn reset(&mut self);
}

pub trait IDestination {
    fn add_byte(&mut self, byte: u8);
    fn add_bytes(&mut self, bytes: &str);
    fn clear(&mut self);
}