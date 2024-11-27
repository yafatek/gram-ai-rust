use std::sync::Arc;
use parking_lot::RwLock;

use crate::{
    error::Result,
    memory::Memory,
    provider::LLMProvider,
    tool::{Function, RustFunction},
};

pub struct Agent<P: LLMProvider> {
    provider: P,
    memory: Arc<RwLock<Box<dyn Memory>>>,
    functions: Vec<Arc<dyn RustFunction>>,
}

impl<P: LLMProvider> Agent<P> {
    pub fn new(provider: P, memory: Arc<RwLock<Box<dyn Memory>>>) -> Self {
        Self {
            provider,
            memory,
            functions: Vec::new(),
        }
    }

    pub fn add_function(&mut self, function: Arc<dyn RustFunction>) {
        self.functions.push(function);
    }

    pub async fn generate_response(&self, prompt: &str) -> Result<String> {
        // Convert RustFunctions to Functions for the provider
        let functions: Vec<Function> = self.functions.iter()
            .enumerate()
            .map(|(i, f)| Function {
                name: format!("function_{}", i),
                description: "A mathematical function".to_string(),
                function: f.clone(),
            })
            .collect();

        self.provider.generate_response(prompt, &None, &functions).await
    }
}
