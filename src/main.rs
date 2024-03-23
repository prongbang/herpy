use std::net::SocketAddr;
use std::sync::Arc;

use herpy::config::config::{GatewayConfig, load_config};

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        println!("{e:?}");
    }
}

async fn run() -> Result<(), anyhow::Error> {
    // Initialize logging.
    if std::env::var("RUST_LOG").is_err() {
        // Set default log level.
        std::env::set_var("RUST_LOG", "herpy=info,warn");
    }
    tracing_subscriber::fmt::init();

    let config: GatewayConfig = load_config("herpy.yaml");
    let client = reqwest::Client::new();

    let client = Arc::new(client);
    let config = Arc::new(config);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port.clone()));

    herpy::server::run_server(config, client, addr).await
}