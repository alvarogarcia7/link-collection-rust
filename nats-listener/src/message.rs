/// Message types for HackerNews NATS messages
use serde::{Deserialize, Serialize};
use std::fmt;

/// A HackerNews link entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    pub id: String,
    pub title: String,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
}

/// Parsed message from messages.20.hn topic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HackerNewsMessage {
    /// Unique message ID (UUID)
    pub id: String,

    /// Message type (should be "hackernews")
    #[serde(default)]
    pub message_type: String,

    /// The note/entry containing HackerNews data
    pub note: Note,

    /// Source of the message
    #[serde(default)]
    pub source: String,
}

/// Note object within the message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
}

impl HackerNewsMessage {
    /// Validate the message has required fields
    pub fn validate(&self) -> bool {
        !self.id.is_empty() && !self.note.id.is_empty() && !self.note.title.is_empty()
    }

    /// Extract link from the message
    pub fn to_link(&self) -> Link {
        Link {
            id: self.note.id.clone(),
            title: self.note.title.clone(),
            url: self.note.url.clone().unwrap_or_default(),
            description: self.note.text.clone(),
            date: self.note.date.clone(),
        }
    }
}

impl fmt::Display for HackerNewsMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "HackerNews message: {} ({})",
            self.note.title, self.note.id
        )
    }
}

impl fmt::Display for Link {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {} ({})",
            self.id,
            self.title,
            self.url.chars().take(50).collect::<String>()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hackernews_message_validation() {
        let msg = HackerNewsMessage {
            id: "test-id".to_string(),
            message_type: "hackernews".to_string(),
            note: Note {
                id: "123".to_string(),
                title: "Rust Release".to_string(),
                text: None,
                url: Some("https://example.com".to_string()),
                date: None,
            },
            source: "hn".to_string(),
        };

        assert!(msg.validate());
    }

    #[test]
    fn test_message_to_link() {
        let msg = HackerNewsMessage {
            id: "msg-id".to_string(),
            message_type: "hackernews".to_string(),
            note: Note {
                id: "456".to_string(),
                title: "Awesome Article".to_string(),
                text: Some("A great read".to_string()),
                url: Some("https://example.com/article".to_string()),
                date: Some("2026-05-02".to_string()),
            },
            source: "hn".to_string(),
        };

        let link = msg.to_link();
        assert_eq!(link.id, "456");
        assert_eq!(link.title, "Awesome Article");
        assert_eq!(link.url, "https://example.com/article");
    }

    #[test]
    fn test_message_serialization() {
        let msg = HackerNewsMessage {
            id: "uuid-123".to_string(),
            message_type: "hackernews".to_string(),
            note: Note {
                id: "789".to_string(),
                title: "Test Story".to_string(),
                text: Some("Test description".to_string()),
                url: Some("https://test.com".to_string()),
                date: Some("2026-05-02".to_string()),
            },
            source: "hn".to_string(),
        };

        let json = serde_json::to_string(&msg).expect("serialization failed");
        let deserialized: HackerNewsMessage =
            serde_json::from_str(&json).expect("deserialization failed");

        assert_eq!(deserialized.id, msg.id);
        assert_eq!(deserialized.note.title, msg.note.title);
    }
}
