use std::net::SocketAddr;

use std::convert::Infallible;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Context as _;
use hyper::server::conn::{AddrStream, Http};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Server};
use modsecurity::Transaction;
use reqwest::{Client, Version};
use crate::config::GatewayConfig;
use crate::gateway;

pub fn http_version(version: Version) -> &'static str {
    match version {
        Version::HTTP_09 => "0.9",
        Version::HTTP_10 => "1.0",
        Version::HTTP_11 => "1.1",
        Version::HTTP_2 => "2.0",
        Version::HTTP_3 => "3.0",
        _ => "Unknown",
    }
}

pub async fn run_server(
    config: Arc<GatewayConfig>,
    mod_tx: Option<Transaction>,
    client: Arc<Client>,
    addr: SocketAddr,
) -> Result<(), anyhow::Error> {
    let make_service = make_service_fn(move |_: &AddrStream| {
        let config = Arc::clone(&config);
        let client = Arc::clone(&client);

        // Create a `Service` for responding to the request.
        let service = service_fn(move |req| {
            if let Some(mut tx) = mod_tx {
                let uri = req.uri().clone();
                let method = req.method().clone();
                let version = req.version().clone();
                let mut headers = req.headers().clone();
                tx.process_uri(uri.path(), method.as_str(), http_version(version)).expect("Error processing URI");
                for header in headers {
                    if let Some(key) = header.0 {
                        if key.as_str() != "host" {
                            let value = header.1.to_str().unwrap_or("");
                            tx.add_request_header(key.as_str(), value).unwrap_or(());
                        }
                    }
                }
                tx.process_request_headers().expect("Error processing request headers");
                let intervention = tx.intervention().expect("Expected intervention");
            }
            gateway::handler::request(req, Arc::clone(&config), Arc::clone(&client))
        });

        // Return the service to hyper.
        async move { Ok::<_, Infallible>(service) }
    });

    tracing::info!(listen=%addr, "starting server on '{addr}'");

    Server::bind(&addr)
        .http1_keepalive(true)
        .http2_keep_alive_timeout(Duration::from_secs(120))
        .serve(make_service)
        .await
        .context("hyper server failed")
}