use std::convert::Infallible;
use hyper::service::{make_service_fn, service_fn};
use std::net::SocketAddr;
use hyper::server::conn::AddrStream;
use herpy::config::config::{GatewayConfig, load_config};
use herpy::gateway;

use std::time::Duration;

use anyhow::Context as _;
use async_trait::async_trait;
use hyper::{Body, Request, Response, Server};

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        println!("{e:?}");
    }
}

async fn run() -> Result<(), anyhow::Error> {
    std::env::set_var("TOKIO_WORKER_THREADS", format!("{}", num_cpus::get()));

    let config: GatewayConfig = load_config("herpy.yaml");


    #[cfg_attr(target_os = "wasi", allow(unused))]
        let (tx, rx) = tokio::sync::oneshot::channel();

    // There are two main points to consider here:
    // * ctrlc is not available on WASIX and signal handling in
    //   general is not stable and fully wired up yet.
    // * When running under WASIX, we expect 99% of usage to be
    //   either local development or running on Wasmer Edge.
    //   Wasmer Edge already keeps instances alive while they're
    //   processing a request, and dropping requests during local
    //   development isn't likely to cause problems.
    // Given the two points above, clean shutdown is implemented
    // for native builds only.
    // #[cfg(not(target_os = "wasi"))]
    // {
    //     let timeout = cmd
    //         .shutdown_timeout
    //         .map(Duration::from_secs)
    //         .unwrap_or_else(|| Duration::from_secs(60));
    //     let timeout = if timeout.is_zero() {
    //         None
    //     } else {
    //         Some(timeout)
    //     };
    //
    //     let runner_clone = runner.clone();
    //     let mut shutdown_future = Some(async move {
    //         runner_clone.shutdown(timeout).await;
    //         _ = tx.send(());
    //     });
    //     ctrlc::set_handler(move || {
    //         if let Some(f) = shutdown_future.take() {
    //             tokio::runtime::Builder::new_current_thread()
    //                 .enable_all()
    //                 .build()
    //                 .unwrap()
    //                 .block_on(f);
    //         }
    //     })
    //         .expect("Failed to set Ctrl-C handler");
    // }

    run_server(config, rx).await
}

pub async fn run_server(
    config: GatewayConfig,
    //handler: Box<dyn Runner + Send + Sync>,
    shutdown_signal: tokio::sync::oneshot::Receiver<()>,
) -> Result<(), anyhow::Error> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    let make_service = make_service_fn(move |conn: &AddrStream| {
        let config = config.clone();

        let addr = conn.remote_addr();

        // Create a `Service` for responding to the request.
        let service = service_fn(move |req| gateway::handler::request(config.clone(), addr, req));

        // Return the service to hyper.
        async move { Ok::<_, Infallible>(service) }

        // async {
        //     Ok::<_, hyper::Error>(service_fn(move |req| {
        //         let config: GatewayConfig = config.clone();
        //         gateway::handler::request(req, config)
        //     }))
        // }
    });

    Server::bind(&addr)
        .serve(make_service)
        .with_graceful_shutdown(async move { _ = shutdown_signal.await })
        .await
        .context("hyper server failed")
}