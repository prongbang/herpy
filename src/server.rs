use std::net::SocketAddr;

use std::convert::Infallible;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Context as _;
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Server};
use reqwest::Client;
use crate::config::GatewayConfig;
use crate::gateway;

pub async fn run_server(
    config: Arc<GatewayConfig>,
    client: Arc<Client>,
    addr: SocketAddr,
) -> Result<(), anyhow::Error> {
    let make_service = make_service_fn(move |_: &AddrStream| {
        let config = Arc::clone(&config);
        let client = Arc::clone(&client);

        // Create a `Service` for responding to the request.
        let service = service_fn(move |req|
            gateway::handler::request(req, Arc::clone(&config), Arc::clone(&client))
        );

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