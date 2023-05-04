use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use reqwest::header::{HeaderMap, AUTHORIZATION};
use std::net::SocketAddr;
use herpy::config::config::{GatewayConfig, load_config, ServiceConfig};
use herpy::gateway;

#[tokio::main]
async fn main() {
    let config: GatewayConfig = load_config("config.yaml");

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    let make_svc = make_service_fn(move |_conn| {
        let config: GatewayConfig = config.clone();

        async {
            Ok::<_, hyper::Error>(service_fn(move |req| {
                let config: GatewayConfig = config.clone();
                gateway::handler::handle_request(req, config)
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
