/// Error types for NATS listener
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ListenerError {
    #[error("NATS connection failed: {0}")]
    ConnectionError(String),

    #[error("Failed to decode message: {0}")]
    DecodeError(String),

    #[error("Message validation failed: {0}")]
    ValidationError(String),

    #[error("Failed to process message: {0}")]
    ProcessingError(String),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("TLS error: {0}")]
    TlsError(String),

    #[error("Channel error: {0}")]
    ChannelError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, ListenerError>;
