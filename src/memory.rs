use std::collections::HashMap;

/// Memory trait for storing and retrieving data
pub trait Memory: Send + Sync {
    fn get(&self, key: &str) -> Option<String>;
    fn set(&mut self, key: &str, value: String);
}

/// Default in-memory implementation using a HashMap
#[derive(Default)]
pub struct DefaultMemory {
    data: HashMap<String, String>,
}

impl DefaultMemory {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}

impl Memory for DefaultMemory {
    fn get(&self, key: &str) -> Option<String> {
        self.data.get(key).cloned()
    }

    fn set(&mut self, key: &str, value: String) {
        self.data.insert(key.to_string(), value);
    }
}

/// Helper functions to create different types of memory
pub fn memory() -> DefaultMemory {
    DefaultMemory::new()
}
