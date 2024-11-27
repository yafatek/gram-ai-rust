use crate::{error::Result, tool::Function};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationConfig {
    pub temperature: f32,
    pub top_p: f32,
    pub max_output_tokens: i32,
}

#[async_trait]
pub trait LLMProvider: Send + Sync {
    fn name(&self) -> &str;
    
    async fn generate_response(
        &self,
        prompt: &str,
        config: &Option<GenerationConfig>,
        functions: &[Function],
    ) -> Result<String>;
}

pub mod gemini;
pub mod openai;
