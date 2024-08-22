use std::net::SocketAddr;
use std::sync::Arc;
use clap::Parser;

use herpy::config::{Args, GatewayConfig};
use herpy::modsec;

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

    let args = Args::parse();
    let config: GatewayConfig = herpy::config::load(args);
    let config = Arc::new(config);

    // Mod Security
    let mod_tx = modsec::initial(&config.waf);

    let client = reqwest::Client::new();
    let client = Arc::new(client);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.metadata.port.clone()));

    herpy::server::run_server(config, mod_tx, client, addr).await
}