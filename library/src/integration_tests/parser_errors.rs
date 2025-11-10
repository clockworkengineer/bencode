//! Integration tests for parser error handling.


#[cfg(test)]
mod tests {
    use crate::BufferSource;
    use crate::error::messages::*;
    use crate::parser::default::parse;
    
    #[test]
    fn test_empty_input() {
        let mut source = BufferSource::new(b"");
        assert!(matches!(parse(&mut source), Err(s) if s == ERR_EMPTY_INPUT));
    }
    
    #[test]
    fn test_invalid_character() {
        let mut source = BufferSource::new(b"x123");
        assert!(matches!(parse(&mut source), Err(s) if s.contains("Unexpected character")));
    }
    
    #[test]
    fn test_invalid_start_character_z() {
        let mut source = BufferSource::new(b"z");
        assert!(matches!(parse(&mut source), Err(s) if s.contains("Unexpected character")));
    }
    
    #[test]
    fn test_invalid_start_character_dash() {
        let mut source = BufferSource::new(b"-42");
        assert!(matches!(parse(&mut source), Err(s) if s.contains("Unexpected character")));
    }
    
    #[test]
    fn test_malformed_nested_structure() {
        let mut source = BufferSource::new(b"ld3:keyi42e");
        assert!(
            matches!(parse(&mut source), Err(s) if s == ERR_UNTERMINATED_DICTIONARY || s == ERR_UNTERMINATED_LIST)
        );
    }
    
    #[test]
    fn test_dictionary_missing_value() {
        let mut source = BufferSource::new(b"d3:keye");
        assert!(matches!(parse(&mut source), Err(_)));
    }
    
    #[test]
    fn test_list_with_invalid_element() {
        let mut source = BufferSource::new(b"lxe");
        assert!(matches!(parse(&mut source), Err(s) if s.contains("Unexpected character")));
    }
    
    #[test]
    fn test_string_with_colon_separator_only() {
        // Test the case where ':' is encountered as the first character
        let mut source = BufferSource::new(b":test");
        assert!(matches!(parse(&mut source), Err(s) if s == ERR_INVALID_STRING_LENGTH));
    }
    
}
