use grami_sdk::{
    agent::Agent,
    memory::SimpleMemory,
    provider::gemini::GeminiProvider,
};
use std::sync::Arc;
use parking_lot::RwLock;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv().ok();

    // Get API key from environment
    let api_key = std::env::var("GEMINI_API_KEY")
        .expect("GEMINI_API_KEY not found in environment or .env file");

    let provider = GeminiProvider::new(api_key);
    let memory = Arc::new(RwLock::new(Box::new(SimpleMemory::new()) as Box<dyn grami_sdk::memory::Memory>));
    
    let mut agent = Agent::new(provider, memory);

    // Add multiply function
    agent.add_function(Arc::new(|numbers: &[f64]| numbers.iter().product()));

    // Test the agent with different prompts
    let prompts = vec![
        "What is 2 * 3 * 4?",
        "Can you multiply 5, 6, and 7 together?",
        "Multiply 10 and 20",
    ];

    for prompt in prompts {
        println!("\nUser: {}", prompt);
        let response = agent.generate_response(prompt).await?;
        println!("Assistant: {}", response);
    }

    Ok(())
}
