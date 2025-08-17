/// Module implementing the default recursive bencode parser.
/// Provides standard bencode parsing functionality using recursive descent approach.
pub mod default;
/// Module implementing a non-recursive bencode parser variant.
/// Provides an alternative parsing implementation that avoids recursion for better memory efficiency.
pub mod non_recursive;