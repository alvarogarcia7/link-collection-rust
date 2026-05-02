/// HackerNews NATS Listener
///
/// Subscribes to messages.20.hn topic and processes HackerNews link entries.
///
/// Environment variables:
/// - NATS_URL: NATS server URL (default: tls://localhost:4222)
/// - CERTS_DIR: Directory containing TLS certificates (for mTLS)

use nats_listener::{NatsClient, NatsConfig, MessageHandler};
use log::{info, error};
use std::env;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "HackerNews NATS Listener")]
#[command(about = "Listen to HackerNews messages on NATS", long_about = None)]
struct Args {
    /// NATS server URL
    #[arg(long, default_value = "tls://localhost:4222")]
    nats_url: String,

    /// NATS topic to subscribe to
    #[arg(long, default_value = "messages.20.hn")]
    topic: String,

    /// Directory containing TLS certificates
    #[arg(long)]
    certs_dir: Option<String>,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> nats_listener::Result<()> {
    // Initialize logging
    let env_var = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    env::set_var("RUST_LOG", env_var);
    env_logger::init();

    // Parse command line arguments
    let args = Args::parse();

    // Load configuration
    let mut config = NatsConfig::new(args.nats_url, args.topic);

    if let Some(certs_dir) = args.certs_dir {
        let cert_path = format!("{}/client.pem", certs_dir);
        let key_path = format!("{}/client.key", certs_dir);
        let ca_path = format!("{}/rootCA.pem", certs_dir);
        config = config.with_tls(cert_path, key_path, ca_path);
    }

    // Connect to NATS
    let client = NatsClient::connect(config)?;
    info!("✓ Connected to NATS on topic: {}", client.topic());

    // Create message handler
    let mut handler = MessageHandler::new();
    info!("Starting to listen for messages...");

    // Subscribe and handle messages
    client.subscribe(|data| handler.handle_message(data))?;

    Ok(())
}
