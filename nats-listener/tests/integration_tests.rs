/// Integration tests for HackerNews NATS listener
use nats_listener::{HackerNewsMessage, Link, MessageHandler};
use serde_json::json;

#[test]
fn test_parse_hackernews_message() {
    let json_str = r#"{
        "id": "550e8400-e29b-41d4-a716-446655440000",
        "message_type": "hackernews",
        "note": {
            "id": "37962234",
            "title": "Ask HN: Recommendations for learning Rust?",
            "text": "I want to learn Rust for systems programming",
            "url": "https://news.ycombinator.com/item?id=37962234",
            "date": "2026-05-02"
        },
        "source": "hn"
    }"#;

    let message: HackerNewsMessage =
        serde_json::from_str(json_str).expect("Failed to parse message");

    assert_eq!(message.id, "550e8400-e29b-41d4-a716-446655440000");
    assert_eq!(message.note.id, "37962234");
    assert_eq!(
        message.note.title,
        "Ask HN: Recommendations for learning Rust?"
    );
    assert!(message.validate());
}

#[test]
fn test_extract_link_from_message() {
    let message = HackerNewsMessage {
        id: "test-id".to_string(),
        message_type: "hackernews".to_string(),
        note: nats_listener::message::Note {
            id: "123".to_string(),
            title: "Rust Release 1.80".to_string(),
            text: Some("A new stable release".to_string()),
            url: Some("https://blog.rust-lang.org".to_string()),
            date: Some("2026-05-02".to_string()),
        },
        source: "hn".to_string(),
    };

    let link = message.to_link();
    assert_eq!(link.id, "123");
    assert_eq!(link.title, "Rust Release 1.80");
    assert_eq!(link.url, "https://blog.rust-lang.org");
    assert_eq!(link.description, Some("A new stable release".to_string()));
    assert_eq!(link.date, Some("2026-05-02".to_string()));
}

#[test]
fn test_message_handler_processes_multiple_messages() {
    let mut handler = MessageHandler::new();

    // Create multiple test messages
    let messages = vec![
        HackerNewsMessage {
            id: "id-1".to_string(),
            message_type: "hackernews".to_string(),
            note: nats_listener::message::Note {
                id: "1".to_string(),
                title: "Story 1".to_string(),
                text: None,
                url: Some("https://example.com/1".to_string()),
                date: None,
            },
            source: "hn".to_string(),
        },
        HackerNewsMessage {
            id: "id-2".to_string(),
            message_type: "hackernews".to_string(),
            note: nats_listener::message::Note {
                id: "2".to_string(),
                title: "Story 2".to_string(),
                text: Some("Description".to_string()),
                url: Some("https://example.com/2".to_string()),
                date: Some("2026-05-02".to_string()),
            },
            source: "hn".to_string(),
        },
    ];

    // Process each message
    for message in messages {
        let json = serde_json::to_vec(&message).expect("serialization failed");
        let result = handler.handle_message(&json);
        assert!(result.is_ok());
    }

    // Verify statistics
    assert_eq!(handler.stats().total_received, 2);
    assert_eq!(handler.stats().successfully_processed, 2);
    assert_eq!(handler.stats().validation_errors, 0);
}

#[test]
fn test_invalid_message_handling() {
    let mut handler = MessageHandler::new();

    // Test with invalid JSON
    let invalid_json = br#"{"invalid": json}"#;
    let result = handler.handle_message(invalid_json);
    assert!(result.is_err());

    // Test with incomplete message structure
    let incomplete = serde_json::json!({
        "id": "test-id",
        "note": {
            "title": "Missing required id field"
        }
    });

    let json = serde_json::to_vec(&incomplete).expect("serialization failed");
    let result = handler.handle_message(&json);
    assert!(result.is_err());
}

#[test]
fn test_message_with_all_fields() {
    let json_obj = json!({
        "id": "550e8400-e29b-41d4-a716-446655440000",
        "message_type": "hackernews",
        "note": {
            "id": "37962234",
            "title": "Show HN: Awesome Rust Project",
            "text": "I built this cool thing in Rust",
            "url": "https://github.com/example/project",
            "date": "2026-05-02"
        },
        "source": "hn"
    });

    let message: HackerNewsMessage =
        serde_json::from_value(json_obj).expect("deserialization failed");

    assert!(message.validate());
    assert_eq!(message.source, "hn");
    assert_eq!(message.message_type, "hackernews");

    let link = message.to_link();
    assert_eq!(
        link.description,
        Some("I built this cool thing in Rust".to_string())
    );
}

#[test]
fn test_message_serialization_roundtrip() {
    let original = HackerNewsMessage {
        id: "test-id".to_string(),
        message_type: "hackernews".to_string(),
        note: nats_listener::message::Note {
            id: "999".to_string(),
            title: "Test Story".to_string(),
            text: Some("Test content".to_string()),
            url: Some("https://test.com".to_string()),
            date: Some("2026-05-02".to_string()),
        },
        source: "hn".to_string(),
    };

    // Serialize to JSON
    let json = serde_json::to_string(&original).expect("serialization failed");

    // Deserialize back
    let deserialized: HackerNewsMessage =
        serde_json::from_str(&json).expect("deserialization failed");

    // Verify all fields match
    assert_eq!(deserialized.id, original.id);
    assert_eq!(deserialized.note.id, original.note.id);
    assert_eq!(deserialized.note.title, original.note.title);
    assert_eq!(deserialized.note.text, original.note.text);
    assert_eq!(deserialized.note.url, original.note.url);
    assert_eq!(deserialized.note.date, original.note.date);
}

#[test]
fn test_message_display_formatting() {
    let message = HackerNewsMessage {
        id: "test-id".to_string(),
        message_type: "hackernews".to_string(),
        note: nats_listener::message::Note {
            id: "123".to_string(),
            title: "Interesting Story".to_string(),
            text: None,
            url: None,
            date: None,
        },
        source: "hn".to_string(),
    };

    let display_string = format!("{}", message);
    assert!(display_string.contains("Interesting Story"));
    assert!(display_string.contains("123"));
}

#[test]
fn test_link_display_formatting() {
    let link = Link {
        id: "456".to_string(),
        title: "Great Article".to_string(),
        url: "https://example.com/very/long/url/that/should/be/truncated".to_string(),
        description: Some("A description".to_string()),
        date: Some("2026-05-02".to_string()),
    };

    let display_string = format!("{}", link);
    assert!(display_string.contains("456"));
    assert!(display_string.contains("Great Article"));
}
