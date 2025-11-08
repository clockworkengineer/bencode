use crate::io::traits::IDestination;

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
    use crate::io::destinations::buffer::Buffer;

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

    #[test]
    fn test_escape_null_byte() {
        let mut destination = Buffer::new();
        escape_string("hello\x00world", &mut destination);
        assert_eq!(destination.to_string(), "hello\\u0000world");
    }

    #[test]
    fn test_escape_carriage_return() {
        let mut destination = Buffer::new();
        escape_string("line1\rline2", &mut destination);
        assert_eq!(destination.to_string(), "line1\\u000dline2");
    }

    #[test]
    fn test_escape_backspace() {
        let mut destination = Buffer::new();
        escape_string("hello\x08world", &mut destination);
        assert_eq!(destination.to_string(), "hello\\u0008world");
    }

    #[test]
    fn test_escape_form_feed() {
        let mut destination = Buffer::new();
        escape_string("page1\x0Cpage2", &mut destination);
        assert_eq!(destination.to_string(), "page1\\u000cpage2");
    }

    #[test]
    fn test_escape_vertical_tab() {
        let mut destination = Buffer::new();
        escape_string("line1\x0Bline2", &mut destination);
        assert_eq!(destination.to_string(), "line1\\u000bline2");
    }

    #[test]
    fn test_escape_delete_char() {
        let mut destination = Buffer::new();
        escape_string("text\x7Fmore", &mut destination);
        assert_eq!(destination.to_string(), "text\\u007fmore");
    }

    #[test]
    fn test_escape_mixed_special_chars() {
        let mut destination = Buffer::new();
        escape_string("\"test\"\n\t\r\x00", &mut destination);
        assert_eq!(
            destination.to_string(),
            "\\\"test\\\"\\u000a\\u0009\\u000d\\u0000"
        );
    }

    #[test]
    fn test_escape_all_printable_ascii() {
        let mut destination = Buffer::new();
        escape_string("abc123!@#$%^&*()", &mut destination);
        assert_eq!(destination.to_string(), "abc123!@#$%^&*()");
    }
}
