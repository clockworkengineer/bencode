//! Lightweight error types for embedded systems
//!
//! This module provides error types that don't require heap allocation,
//! making them suitable for no_std environments with limited memory.

use core::fmt;

/// Lightweight error type for bencode parsing in embedded systems.
/// Uses no heap allocation - all error information is in the enum variant itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BencodeError {
    /// Input was empty or exhausted unexpectedly
    EmptyInput,

    /// Invalid integer format (e.g., non-numeric characters, -0, etc.)
    InvalidInteger,

    /// Integer not properly terminated with 'e'
    UnterminatedInteger,

    /// String length prefix is invalid or not a valid number
    InvalidStringLength,

    /// String content is shorter than declared length
    StringTooShort,

    /// List not properly terminated with 'e'
    UnterminatedList,

    /// Dictionary not properly terminated with 'e'
    UnterminatedDictionary,

    /// Dictionary keys must be in lexicographic order
    DictKeysOutOfOrder,

    /// Dictionary key must be a string type
    DictKeyMustBeString,

    /// Encountered an unexpected character (stores the character)
    UnexpectedCharacter(char),

    /// File not found (for std environments)
    FileNotFound,

    /// Generic IO error
    IoError,
}

impl BencodeError {
    /// Returns a static string describing the error.
    /// This avoids allocation and is suitable for no_std environments.
    pub const fn as_str(&self) -> &'static str {
        match self {
            BencodeError::EmptyInput => "Empty input",
            BencodeError::InvalidInteger => "Invalid integer",
            BencodeError::UnterminatedInteger => "Unterminated integer",
            BencodeError::InvalidStringLength => "Invalid string length",
            BencodeError::StringTooShort => "String too short",
            BencodeError::UnterminatedList => "Unterminated list",
            BencodeError::UnterminatedDictionary => "Unterminated dictionary",
            BencodeError::DictKeysOutOfOrder => "Dictionary keys must be in order",
            BencodeError::DictKeyMustBeString => "Dictionary key must be string",
            BencodeError::UnexpectedCharacter(_) => "Unexpected character",
            BencodeError::FileNotFound => "File not found",
            BencodeError::IoError => "IO error",
        }
    }

    /// Returns the error code as a u8 for compact error reporting
    pub const fn code(&self) -> u8 {
        match self {
            BencodeError::EmptyInput => 1,
            BencodeError::InvalidInteger => 2,
            BencodeError::UnterminatedInteger => 3,
            BencodeError::InvalidStringLength => 4,
            BencodeError::StringTooShort => 5,
            BencodeError::UnterminatedList => 6,
            BencodeError::UnterminatedDictionary => 7,
            BencodeError::DictKeysOutOfOrder => 8,
            BencodeError::DictKeyMustBeString => 9,
            BencodeError::UnexpectedCharacter(_) => 10,
            BencodeError::FileNotFound => 11,
            BencodeError::IoError => 12,
        }
    }
}

impl fmt::Display for BencodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BencodeError::UnexpectedCharacter(c) => {
                write!(f, "Unexpected character: {}", c)
            }
            _ => f.write_str(self.as_str()),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for BencodeError {}

/// Conversion from String errors (for backward compatibility)
#[cfg(feature = "std")]
impl From<BencodeError> for String {
    fn from(err: BencodeError) -> String {
        err.to_string()
    }
}

/// Conversion to String errors (for backward compatibility)
impl From<String> for BencodeError {
    fn from(s: String) -> BencodeError {
        // Try to map common error strings back to enum variants
        match s.as_str() {
            "Empty input" => BencodeError::EmptyInput,
            "Invalid integer" => BencodeError::InvalidInteger,
            "Unterminated integer" => BencodeError::UnterminatedInteger,
            "Invalid string length" => BencodeError::InvalidStringLength,
            "String too short" => BencodeError::StringTooShort,
            "Unterminated list" => BencodeError::UnterminatedList,
            "Unterminated dictionary" => BencodeError::UnterminatedDictionary,
            "Dictionary keys must be in order" => BencodeError::DictKeysOutOfOrder,
            "Dictionary key must be string" => BencodeError::DictKeyMustBeString,
            "File not found" => BencodeError::FileNotFound,
            _ => BencodeError::IoError,
        }
    }
}

impl From<&str> for BencodeError {
    fn from(s: &str) -> BencodeError {
        match s {
            "Empty input" => BencodeError::EmptyInput,
            "Invalid integer" => BencodeError::InvalidInteger,
            "Unterminated integer" => BencodeError::UnterminatedInteger,
            "Invalid string length" => BencodeError::InvalidStringLength,
            "String too short" => BencodeError::StringTooShort,
            "Unterminated list" => BencodeError::UnterminatedList,
            "Unterminated dictionary" => BencodeError::UnterminatedDictionary,
            "Dictionary keys must be in order" => BencodeError::DictKeysOutOfOrder,
            "Dictionary key must be string" => BencodeError::DictKeyMustBeString,
            "File not found" => BencodeError::FileNotFound,
            _ => BencodeError::IoError,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_as_str_works() {
        assert_eq!(BencodeError::EmptyInput.as_str(), "Empty input");
        assert_eq!(BencodeError::InvalidInteger.as_str(), "Invalid integer");
    }

    #[test]
    fn error_code_is_unique() {
        let errors = [
            BencodeError::EmptyInput,
            BencodeError::InvalidInteger,
            BencodeError::UnterminatedInteger,
            BencodeError::InvalidStringLength,
            BencodeError::StringTooShort,
            BencodeError::UnterminatedList,
            BencodeError::UnterminatedDictionary,
            BencodeError::DictKeysOutOfOrder,
            BencodeError::DictKeyMustBeString,
            BencodeError::UnexpectedCharacter('x'),
            BencodeError::FileNotFound,
            BencodeError::IoError,
        ];

        for i in 0..errors.len() {
            for j in (i + 1)..errors.len() {
                assert_ne!(errors[i].code(), errors[j].code());
            }
        }
    }

    #[test]
    fn error_display_works() {
        assert_eq!(format!("{}", BencodeError::EmptyInput), "Empty input");
        assert_eq!(
            format!("{}", BencodeError::UnexpectedCharacter('z')),
            "Unexpected character: z"
        );
    }

    #[test]
    fn error_from_string_works() {
        let err: BencodeError = "Invalid integer".into();
        assert_eq!(err, BencodeError::InvalidInteger);
    }

    #[cfg(feature = "std")]
    #[test]
    fn error_to_string_works() {
        let s: String = BencodeError::InvalidInteger.into();
        assert_eq!(s, "Invalid integer");
    }
}
