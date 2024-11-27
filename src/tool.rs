use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// A pure Rust function that can be called by the LLM
pub type RustFunction = Arc<dyn Fn(&[f64]) -> f64 + Send + Sync>;

/// A function with its metadata (generated by the LLM)
pub struct Function {
    pub name: String,
    pub description: String,
    pub function: RustFunction,
}

impl Function {
    pub fn new(name: String, description: String, function: RustFunction) -> Self {
        Self {
            name,
            description,
            function,
        }
    }
}

impl Clone for Function {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            description: self.description.clone(),
            function: self.function.clone(),
        }
    }
}
