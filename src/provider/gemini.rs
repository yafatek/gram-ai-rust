use crate::{
    error::Result,
    provider::{GenerationConfig, LLMProvider},
    tool::Function,
};
use async_trait::async_trait;
use serde_json::{json, Value};

pub struct GeminiProvider {
    api_key: String,
    client: reqwest::Client,
}

impl GeminiProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl LLMProvider for GeminiProvider {
    fn name(&self) -> &str {
        "gemini"
    }

    async fn generate_response(&self, prompt: &str, _config: &Option<GenerationConfig>, functions: &[Function]) -> Result<String> {
        // Create function descriptions for the prompt
        let functions_desc = functions.iter()
            .map(|f| format!("- {}: {}", f.name, f.description))
            .collect::<Vec<_>>()
            .join("\n");

        let full_prompt = format!(
            "You are a helpful AI assistant that can perform calculations.\n\n\
            Available functions:\n{}\n\n\
            To use a function, extract the numbers from the query and format them as: number1 number2 number3...\n\n\
            User: {}",
            functions_desc,
            prompt
        );

        let request = json!({
            "contents": [{
                "role": "user",
                "parts": [{
                    "text": full_prompt
                }]
            }],
            "generationConfig": {
                "temperature": 0.7,
                "topP": 1.0,
                "maxOutputTokens": 1024
            }
        });

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key={}",
            self.api_key
        );

        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await?
            .json::<Value>()
            .await?;

        let text = response["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .unwrap_or("No response")
            .to_string();

        // Extract numbers and execute the function
        let numbers: Vec<f64> = text
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if let Some(function) = functions.first() {
            let result = (function.function)(&numbers);
            Ok(format!("The result is: {}", result))
        } else {
            Ok(text)
        }
    }
}
