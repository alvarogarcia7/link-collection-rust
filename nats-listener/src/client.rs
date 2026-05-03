/// NATS client for subscribing to messages.20.hn topic
use crate::error::{ListenerError, Result};
use log::{debug, info};

/// NATS client configuration
#[derive(Debug, Clone)]
pub struct NatsConfig {
    pub url: String,
    pub topic: String,
    pub cert_path: Option<String>,
    pub key_path: Option<String>,
    pub ca_path: Option<String>,
}

impl NatsConfig {
    /// Create a new NATS configuration
    pub fn new(url: String, topic: String) -> Self {
        Self {
            url,
            topic,
            cert_path: None,
            key_path: None,
            ca_path: None,
        }
    }

    /// Set TLS certificate paths
    pub fn with_tls(mut self, cert: String, key: String, ca: String) -> Self {
        self.cert_path = Some(cert);
        self.key_path = Some(key);
        self.ca_path = Some(ca);
        self
    }

    /// Load from environment variables
    pub fn from_env() -> Result<Self> {
        let url = std::env::var("NATS_URL")
            .unwrap_or_else(|_| "tls://localhost:4222".to_string());
        let topic = "messages.20.hn".to_string();
        let cert_path = std::env::var("CERTS_DIR")
            .ok()
            .map(|dir| format!("{}/client.pem", dir));
        let key_path = std::env::var("CERTS_DIR")
            .ok()
            .map(|dir| format!("{}/client.key", dir));
        let ca_path = std::env::var("CERTS_DIR")
            .ok()
            .map(|dir| format!("{}/rootCA.pem", dir));

        Ok(Self {
            url,
            topic,
            cert_path,
            key_path,
            ca_path,
        })
    }
}

/// NATS client wrapper
pub struct NatsClient {
    connection: nats::Connection,
    topic: String,
}

impl NatsClient {
    /// Create a new NATS client
    pub fn connect(config: NatsConfig) -> Result<Self> {
        info!("Connecting to NATS at {}", config.url);

        // Create connection options
        let mut options = nats::Options::new();

        // Add TLS if configured
        if let (Some(cert), Some(key), Some(ca)) = (config.cert_path, config.key_path, config.ca_path) {
            debug!("Using TLS with certificates: cert={}, key={}, ca={}", cert, key, ca);
            options = options
                .client_cert(&cert, &key)
                .add_root_certificate(&ca);
        }

        // Connect to NATS
        let connection = options
            .connect(&config.url)
            .map_err(|e| ListenerError::ConnectionError(format!("Failed to connect: {:?}", e)))?;

        info!("✓ Connected to NATS");

        Ok(Self {
            connection,
            topic: config.topic,
        })
    }

    /// Subscribe to messages on the configured topic
    pub fn subscribe<F>(&self, mut handler: F) -> Result<()>
    where
        F: FnMut(&[u8]) -> Result<()>,
    {
        info!("Subscribing to topic: {}", self.topic);

        let subscriber = self.connection.subscribe(&self.topic)
            .map_err(|e| ListenerError::ConnectionError(format!("Subscribe failed: {}", e)))?;

        info!("✓ Subscribed to {}", self.topic);

        for message in subscriber.iter() {
            match handler(message.data.as_ref()) {
                Ok(_) => {
                    debug!("Message processed successfully");
                }
                Err(e) => {
                    log::error!("Error handling message: {}", e);
                    // Continue processing other messages
                }
            }
        }

        Ok(())
    }

    /// Publish a message to the topic
    pub fn publish(&self, subject: &str, data: &[u8]) -> Result<()> {
        self.connection
            .publish(subject, data)
            .map_err(|e| ListenerError::ConnectionError(format!("Publish failed: {}", e)))
    }

    /// Get the topic being subscribed to
    pub fn topic(&self) -> &str {
        &self.topic
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nats_config_creation() {
        let config = NatsConfig::new(
            "tls://localhost:4222".to_string(),
            "messages.20.hn".to_string(),
        );

        assert_eq!(config.url, "tls://localhost:4222");
        assert_eq!(config.topic, "messages.20.hn");
        assert!(config.cert_path.is_none());
    }

    #[test]
    fn test_nats_config_with_tls() {
        let config = NatsConfig::new(
            "tls://localhost:4222".to_string(),
            "messages.20.hn".to_string(),
        )
        .with_tls(
            "cert.pem".to_string(),
            "key.pem".to_string(),
            "ca.pem".to_string(),
        );

        assert_eq!(config.cert_path, Some("cert.pem".to_string()));
        assert_eq!(config.key_path, Some("key.pem".to_string()));
        assert_eq!(config.ca_path, Some("ca.pem".to_string()));
    }
}
