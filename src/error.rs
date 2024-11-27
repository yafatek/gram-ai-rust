use thiserror::Error;
use std::fmt;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Provider error: {0}")]
    Provider(String),
    
    #[error("Tool error: {0}")]
    Tool(String),
    
    #[error("Tool error: {0}")]
    ToolError(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error(transparent)]
    Request(#[from] reqwest::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
