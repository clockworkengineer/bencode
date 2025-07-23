use super::node::Node;
use std::collections::HashMap;

struct Integer {
    pub value: u32,
}

impl Integer {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}

struct Str {
    value: String,
}

impl Str {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
    
}

struct List {
    value: Vec<Node>,
}

impl List {
    pub fn new(value: Vec<Node>) -> Self {
        Self { value }
    }
}

struct Dictionary {
    value: HashMap<String, Node>,
}

impl Dictionary {
    pub fn new(value: HashMap<String, Node>) -> Self {
        Self { value }
    }
}

