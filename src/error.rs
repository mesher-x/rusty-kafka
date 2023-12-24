use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KafkaError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("JSON serialization/deserialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Invalid message format")]
    InvalidMessage,

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
}

pub type Result<T> = std::result::Result<T, KafkaError>;
