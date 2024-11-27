use grami_sdk::{
    agent::{Agent, AgentConfig},
    memory::Memory,
    provider::{GenerationConfig, LLMProvider, ChatProvider, gemini::GeminiProvider},
};
use std::sync::Arc;
use tokio::sync::RwLock;
use dotenv::dotenv;

#[derive(Default)]
struct ChatMemory {
    data: std::collections::HashMap<String, String>,
}

#[async_trait::async_trait]
impl Memory for ChatMemory {
    async fn store(&mut self, key: &str, value: &str) -> grami_sdk::error::Result<()> {
        self.data.insert(key.to_string(), value.to_string());
        Ok(())
    }

    async fn retrieve(&self, key: &str) -> grami_sdk::error::Result<Option<String>> {
        Ok(self.data.get(key).cloned())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv().ok();

    // Get API key from environment
    let api_key = std::env::var("GEMINI_API_KEY")
        .expect("GEMINI_API_KEY not found in environment or .env file");

    let provider = Arc::new(GeminiProvider::new(api_key));
    let memory: Arc<RwLock<Box<dyn Memory>>> = Arc::new(RwLock::new(Box::new(ChatMemory::default())));
    let state = Arc::new(grami_sdk::state::SharedState::new());

    let config = AgentConfig {
        name: "ChatAgent".to_string(),
        task: "Conversation".to_string(),
        skills: vec!["chat".to_string(), "conversation".to_string()],
        initial_prompt: "You are a friendly and knowledgeable AI assistant.".to_string(),
        system_prompt: Some("You are a friendly AI assistant focused on having natural conversations.".to_string()),
        generation_config: Some(GenerationConfig {
            temperature: 0.8,
            top_p: 1.0,
            max_output_tokens: 1024,
        }),
    };

    let agent = Agent::<GeminiProvider>::builder()
        .with_config(config)
        .with_provider(provider.clone())
        .with_memory(memory)
        .with_state(state)
        .build()?;

    // Start a chat conversation
    println!("Starting chat conversation...\n");

    let messages = vec![
        ("user", "Hi! How are you today?"),
        ("assistant", "I'm doing great, thank you for asking! How can I help you today?"),
        ("user", "Can you tell me about the benefits of using Rust for systems programming?"),
    ];

    let response = agent.chat(&messages).await?;
    println!("Agent Response: {}\n", response);

    // Continue the conversation
    let mut updated_messages = messages;
    updated_messages.push(("assistant", &response));
    updated_messages.push(("user", "What about memory safety in Rust?"));

    let response = agent.chat(&updated_messages).await?;
    println!("Agent Response: {}", response);

    Ok(())
}
