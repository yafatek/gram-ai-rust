use std::sync::RwLock;
use std::collections::HashMap;

pub struct SharedState {
    data: RwLock<HashMap<String, String>>,
}

impl SharedState {
    pub fn new() -> Self {
        Self {
            data: RwLock::new(HashMap::new()),
        }
    }

    pub fn set(&self, key: &str, value: String) -> Result<(), String> {
        self.data
            .write()
            .map_err(|e| e.to_string())?
            .insert(key.to_string(), value);
        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<Option<String>, String> {
        Ok(self.data
            .read()
            .map_err(|e| e.to_string())?
            .get(key)
            .cloned())
    }
}
