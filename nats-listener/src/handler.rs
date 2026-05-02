/// Message handler for processing HackerNews messages
use crate::error::Result;
use crate::message::HackerNewsMessage;
use log::{debug, info, warn};

/// Handles incoming HackerNews messages from NATS
pub struct MessageHandler {
    stats: MessageStats,
}

/// Statistics about processed messages
#[derive(Debug, Clone, Default)]
pub struct MessageStats {
    pub total_received: u64,
    pub successfully_processed: u64,
    pub validation_errors: u64,
    pub decode_errors: u64,
}

impl MessageHandler {
    /// Create a new message handler
    pub fn new() -> Self {
        Self {
            stats: MessageStats::default(),
        }
    }

    /// Process a message from NATS
    pub fn handle_message(&mut self, data: &[u8]) -> Result<()> {
        self.stats.total_received += 1;

        // Decode the message
        let message: HackerNewsMessage = match serde_json::from_slice(data) {
            Ok(msg) => msg,
            Err(e) => {
                warn!("Failed to decode message: {}", e);
                self.stats.decode_errors += 1;
                return Err(crate::error::ListenerError::DecodeError(e.to_string()));
            }
        };

        // Validate the message
        if !message.validate() {
            warn!("Message validation failed: {:?}", message);
            self.stats.validation_errors += 1;
            return Err(crate::error::ListenerError::ValidationError(
                "Invalid message structure".to_string(),
            ));
        }

        // Process the message
        self.process_message(&message)?;
        self.stats.successfully_processed += 1;

        Ok(())
    }

    /// Process a validated message
    fn process_message(&self, message: &HackerNewsMessage) -> Result<()> {
        debug!("Processing message: {}", message);

        // Extract link from message
        let link = message.to_link();
        info!("✓ Received HackerNews link: {}", link);

        // In a real implementation, this would store the link
        // in a database or send it to another service
        debug!(
            "Link details - ID: {}, Title: {}, URL: {}",
            link.id, link.title, link.url
        );

        if let Some(date) = &link.date {
            debug!("Link date: {}", date);
        }

        Ok(())
    }

    /// Get current statistics
    pub fn stats(&self) -> &MessageStats {
        &self.stats
    }

    /// Reset statistics
    pub fn reset_stats(&mut self) {
        self.stats = MessageStats::default();
    }
}

impl Default for MessageHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::Note;

    #[test]
    fn test_message_handler_creation() {
        let handler = MessageHandler::new();
        assert_eq!(handler.stats.total_received, 0);
        assert_eq!(handler.stats.successfully_processed, 0);
    }

    #[test]
    fn test_handle_valid_message() {
        let mut handler = MessageHandler::new();

        let message = HackerNewsMessage {
            id: "test-id".to_string(),
            message_type: "hackernews".to_string(),
            note: Note {
                id: "123".to_string(),
                title: "Rust News".to_string(),
                text: Some("A great story".to_string()),
                url: Some("https://example.com".to_string()),
                date: Some("2026-05-02".to_string()),
            },
            source: "hn".to_string(),
        };

        let json = serde_json::to_vec(&message).unwrap();
        let result = handler.handle_message(&json);

        assert!(result.is_ok());
        assert_eq!(handler.stats.total_received, 1);
        assert_eq!(handler.stats.successfully_processed, 1);
    }

    #[test]
    fn test_handle_invalid_json() {
        let mut handler = MessageHandler::new();
        let invalid_json = b"invalid json";

        let result = handler.handle_message(invalid_json);
        assert!(result.is_err());
        assert_eq!(handler.stats.total_received, 1);
        assert_eq!(handler.stats.decode_errors, 1);
    }

    #[test]
    fn test_handle_invalid_message_structure() {
        let mut handler = MessageHandler::new();

        let invalid_message = serde_json::json!({
            "id": "test",
            "note": {
                "title": "Missing id field"
            }
        });

        let json = serde_json::to_vec(&invalid_message).unwrap();
        let result = handler.handle_message(&json);

        assert!(result.is_err());
        assert_eq!(handler.stats.validation_errors, 1);
    }

    #[test]
    fn test_stats_reset() {
        let mut handler = MessageHandler::new();
        handler.stats.total_received = 10;
        handler.stats.successfully_processed = 8;

        handler.reset_stats();
        assert_eq!(handler.stats.total_received, 0);
        assert_eq!(handler.stats.successfully_processed, 0);
    }
}
