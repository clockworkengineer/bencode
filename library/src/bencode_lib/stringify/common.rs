use crate::bencode_lib::io::traits::IDestination;

/// Escapes and writes a string value to the destination, handling special characters
/// and converting unprintable characters to \u escape sequences.
///
/// # Arguments
/// * `value` - The string value to escape and write
/// * `destination` - The destination to write the escaped string to
pub(crate) fn escape_string(value: &str, destination: &mut dyn IDestination) {
    for &byte in value.as_bytes() {
        if byte == b'"' || byte == b'\\' {
            destination.add_byte(b'\\');
            destination.add_byte(byte);
        } else if byte.is_ascii_graphic() || byte == b' ' {
            destination.add_byte(byte);
        } else {
            // Convert unprintable characters to \u escape sequence
            let escaped = format!("\\u{:04x}", byte);
            destination.add_bytes(&escaped);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bencode_lib::io::destinations::buffer::Buffer;

    #[test]
    fn test_escape_regular_string() {
        let mut destination = Buffer::new();
        escape_string("hello world", &mut destination);
        assert_eq!(destination.to_string(), "hello world");
    }

    #[test]
    fn test_escape_special_characters() {
        let mut destination = Buffer::new();
        escape_string("hello\"world\\test", &mut destination);
        assert_eq!(destination.to_string(), "hello\\\"world\\\\test");
    }

    #[test]
    fn test_escape_unprintable_characters() {
        let mut destination = Buffer::new();
        escape_string("hello\nworld\t", &mut destination);
        assert_eq!(destination.to_string(), "hello\\u000aworld\\u0009");
    }

    #[test]
    fn test_escape_empty_string() {
        let mut destination = Buffer::new();
        escape_string("", &mut destination);
        assert_eq!(destination.to_string(), "");
    }
}
