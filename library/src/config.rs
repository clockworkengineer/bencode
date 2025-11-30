//! Configuration options for bencode parsing and encoding

/// Configuration for the bencode parser
#[derive(Debug, Clone, Copy)]
pub struct ParserConfig {
    /// Maximum depth of nested structures (default: 100)
    /// Set to prevent stack overflow from malicious deeply nested data
    pub max_depth: usize,
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self { max_depth: 100 }
    }
}

impl ParserConfig {
    /// Create a new parser configuration with default settings
    pub const fn new() -> Self {
        Self { max_depth: 100 }
    }
    
    /// Set the maximum nesting depth
    pub const fn with_max_depth(mut self, max_depth: usize) -> Self {
        self.max_depth = max_depth;
        self
    }
}

/// Configuration for the bencode encoder
#[derive(Debug, Clone, Copy)]
pub struct EncoderConfig {
    /// Enforce canonical bencode format (default: true)
    /// - Dictionary keys must be sorted
    /// - No leading zeros in integers (except "0")
    pub enforce_canonical: bool,
    
    /// Verify dictionary key ordering during encoding (default: true)
    pub verify_dict_order: bool,
}

impl Default for EncoderConfig {
    fn default() -> Self {
        Self {
            enforce_canonical: true,
            verify_dict_order: true,
        }
    }
}

impl EncoderConfig {
    /// Create a new encoder configuration with default settings
    pub const fn new() -> Self {
        Self {
            enforce_canonical: true,
            verify_dict_order: true,
        }
    }
    
    /// Set whether to enforce canonical bencode format
    pub const fn with_canonical(mut self, enforce: bool) -> Self {
        self.enforce_canonical = enforce;
        self
    }
    
    /// Set whether to verify dictionary key ordering
    pub const fn with_dict_order_verification(mut self, verify: bool) -> Self {
        self.verify_dict_order = verify;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_config_default() {
        let config = ParserConfig::default();
        assert_eq!(config.max_depth, 100);
    }

    #[test]
    fn parser_config_with_max_depth() {
        let config = ParserConfig::new().with_max_depth(50);
        assert_eq!(config.max_depth, 50);
    }

    #[test]
    fn encoder_config_default() {
        let config = EncoderConfig::default();
        assert!(config.enforce_canonical);
        assert!(config.verify_dict_order);
    }

    #[test]
    fn encoder_config_custom() {
        let config = EncoderConfig::new()
            .with_canonical(false)
            .with_dict_order_verification(false);
        assert!(!config.enforce_canonical);
        assert!(!config.verify_dict_order);
    }
}
