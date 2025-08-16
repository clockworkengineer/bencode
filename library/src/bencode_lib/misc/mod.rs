/// Returns the current version of the package as specified in Cargo.toml
pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_version() {
        assert_eq!(get_version(), "0.1.0");
    }
}