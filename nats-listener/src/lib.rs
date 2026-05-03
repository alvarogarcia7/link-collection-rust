/// NATS listener for HackerNews messages on messages.20.hn topic
///
/// Subscribes to messages.20.hn from the NATS message broker,
/// parses HackerNews link entries, and processes them for storage.

pub mod message;
pub mod error;
pub mod handler;
pub mod client;

pub use message::{HackerNewsMessage, Link};
pub use error::ListenerError;
pub use handler::MessageHandler;
pub use client::{NatsClient, NatsConfig};

pub type Result<T> = std::result::Result<T, ListenerError>;
