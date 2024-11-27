use grami_sdk::{
    agent::{Agent, AgentConfig},
    memory::Memory,
    provider::{Provider, gemini::GeminiProvider},
    tools::{EnableLights, SetLightColor, ToolRegistry},
};
use std::sync::Arc;
use tokio::sync::RwLock;
use dotenv::dotenv;

#[derive(Default)]
struct SimpleMemory {
    data: std::collections::HashMap<String, String>,
}

#[async_trait::async_trait]
impl Memory for SimpleMemory {
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
    let memory: Arc<RwLock<Box<dyn Memory>>> = Arc::new(RwLock::new(Box::new(SimpleMemory::default())));
    let state = Arc::new(grami_sdk::state::SharedState::new());

    // Create and register tools
    let mut registry = ToolRegistry::new();
    registry.register(Box::new(EnableLights));
    registry.register(Box::new(SetLightColor));

    let tool_config = registry.get_tool_definitions();
    
    let config = AgentConfig {
        name: "LightingAgent".to_string(),
        task: "Lighting Control".to_string(),
        skills: vec!["lighting".to_string()],
        initial_prompt: "You are a helpful lighting system bot.".to_string(),
    };

    let agent = Agent::builder()
        .with_config(config)
        .with_provider(provider.clone())
        .with_memory(memory)
        .with_state(state)
        .build()?;

    // Test the agent with tools
    let system_prompt = Some("You are a helpful lighting system bot. You can turn lights on and off, and you can set the color. Do not perform any other tasks.".to_string());
    
    match provider
        .generate_text_with_tools(
            "What can you do?",
            Some(tool_config),
            system_prompt,
        )
        .await
    {
        Ok(response) => println!("Agent Response: {}", response),
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}
