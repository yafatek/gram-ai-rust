# Grami SDK: Rust AI Agent Framework

## 🚀 Project Overview
Grami SDK is a cutting-edge, asynchronous AI agent framework written in Rust, designed to simplify the development of intelligent, context-aware applications with multi-provider AI support.

## 🌟 Key Features
- **Flexible AI Agent Architecture**
  - Modular design for easy customization
  - Support for multiple AI providers
  - Async-first implementation

- **Advanced Memory Management**
  - Stateful conversation tracking
  - Customizable memory interfaces
  - Context preservation across interactions

- **Dynamic Tool Integration**
  - Seamless function calling
  - Runtime tool addition and management
  - Type-safe tool registration

## 🔧 Supported AI Providers
- Google Gemini
- OpenAI
- Ollama

## 📦 Installation

Add to your `Cargo.toml`:
```toml
[dependencies]
grami-sdk = { git = "https://github.com/yafatek/gram-ai-rust" }
```

## 💡 Quick Start Example

```rust
use grami_sdk::{
    agent::Agent,
    memory::SimpleMemory,
    provider::gemini::GeminiProvider,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize AI provider
    let provider = GeminiProvider::new(
        std::env::var("GEMINI_API_KEY")?
    );

    // Create agent with simple memory
    let mut agent = Agent::new(
        provider, 
        SimpleMemory::new()
    );

    // Add custom function
    agent.add_function(|nums: &[f64]| nums.iter().product());

    // Generate response
    let response = agent.generate_response(
        "Calculate 2 * 3 * 4"
    ).await?;

    println!("Result: {}", response);
    Ok(())
}
```

## 📂 Project Structure
- `src/`: Core SDK implementation
  - `agent.rs`: Agent core logic
  - `memory.rs`: Memory management
  - `provider/`: AI provider implementations
- `examples/`: Usage demonstrations
  - `basic.rs`: Basic agent setup
  - `chat.rs`: Conversational AI
  - `tools.rs`: Advanced tool integration

## 🛠 Development Requirements
- Rust 2021 Edition
- Tokio
- Async runtime support

## 🤝 Contributing
1. Fork the repository
2. Create your feature branch
3. Commit changes
4. Push to the branch
5. Create a Pull Request

## 📜 License
MIT License

## 🔗 Dependencies
- Tokio
- Async Trait
- Serde
- Reqwest
- Tracing
- And more (see `Cargo.toml`)

## 📞 Contact
Open an issue on GitHub for questions or support.
