pub mod agent;
pub mod error;
pub mod memory;
pub mod provider;
pub mod tool;

pub use crate::{
    agent::Agent,
    error::{Error, Result},
    memory::Memory,
    provider::LLMProvider,
    tool::{Function, RustFunction},
};

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_basic_agent_creation() {
        // Test implementation will be added
    }
}
