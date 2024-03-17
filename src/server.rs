use std::net::SocketAddr;

use std::convert::Infallible;

use anyhow::Context as _;
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Server};
use crate::config::config::GatewayConfig;
use crate::gateway;

pub async fn run_server(
    config: GatewayConfig,
    addr: SocketAddr,
) -> Result<(), anyhow::Error> {
    let make_service = make_service_fn(move |_: &AddrStream| {
        let config = config.clone();

        // Create a `Service` for responding to the request.
        let service = service_fn(move |req| gateway::handler::request(config.clone(), req));

        // Return the service to hyper.
        async move { Ok::<_, Infallible>(service) }
    });

    tracing::info!(listen=%addr, "starting server on '{addr}'");

    Server::bind(&addr)
        .serve(make_service)
        .await
        .context("hyper server failed")
}