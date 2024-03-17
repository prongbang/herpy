use std::convert::Infallible;
use hyper::{Body, Request, Response};

use crate::{forwarder};
use crate::config::config::GatewayConfig;
use std::net::SocketAddr;

pub async fn request(
    config: GatewayConfig,
    addr: SocketAddr,
    req: Request<Body>,
) -> Result<Response<Body>, Infallible> {
    let res = handle_inner(config, addr, req).await.unwrap_or_else(|err| {
        //tracing::error!(error = format!("{err:#?}"), "could not process request");

        hyper::Response::builder()
            .status(hyper::StatusCode::INTERNAL_SERVER_ERROR)
            .body(hyper::Body::from(err.to_string()))
            .unwrap()
    });

    Ok(res)
}

async fn handle_inner(
    config: GatewayConfig,
    addr: SocketAddr,
    req: Request<Body>,
) -> Result<Response<Body>, anyhow::Error> {
    let path = req.uri().path();

    if let Some(service_map) = &config.services_map {
        if let Some(service) = service_map.get(path) {
            let (parts, body) = req.into_parts();
            for backend in &service.backends {
                return match forwarder::hyper::forward(parts, body, &backend).await {
                    Ok(res) => Ok(res),
                    Err(_) => {
                        let response = hyper::Response::builder()
                            .status(hyper::StatusCode::NOT_FOUND)
                            .body(hyper::Body::from("Failed to connect to downstream service".to_string()))
                            .unwrap();
                        Ok(response)
                    }
                };
            }
        }
    }

    let response = hyper::Response::builder()
        .status(hyper::StatusCode::NOT_FOUND)
        .body(hyper::Body::from("Not Found".to_string()))
        .unwrap();

    Ok(response)

    // let (parts, body) = req.into_parts();
    // context
    //     .runner
    //     .handle(addr, parts, body)
    //     .await
    //     .context("JavaScript failed")
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use hyper::{StatusCode, Body};
//
//     #[tokio::test]
//     async fn test_handle_request_not_found() {
//         let req = Request::builder().uri("/unknown").body(Body::empty()).unwrap();
//         let config = GatewayConfig {
//             authorization_api_url: "http://auth.example.com".to_string(),
//             services: vec![ServiceConfig {
//                 path: "/service".to_string(),
//                 target_service: "http://service.example.com".to_string(),
//                 target_port: "80".to_string(),
//             }],
//         };
//         let res = handle_request(req, config).await.unwrap();
//         assert_eq!(res.status(), StatusCode::NOT_FOUND);
//     }
//
//     #[tokio::test]
//     async fn test_handle_request_authorize_user_error() {
//         let req = Request::builder().uri("/service").body(Body::empty()).unwrap();
//         let config = GatewayConfig {
//             authorization_api_url: "http://auth.example.com".to_string(),
//             services: vec![ServiceConfig {
//                 path: "/service".to_string(),
//                 target_service: "http://service.example.com".to_string(),
//                 target_port: "80".to_string(),
//             }],
//         };
//         let res = handle_request(req, config).await.unwrap();
//         assert_eq!(res.status(), StatusCode::SERVICE_UNAVAILABLE);
//     }
//
//     #[tokio::test]
//     async fn test_handle_request_forward_request_error() {
//         let req = Request::builder().uri("/service").body(Body::empty()).unwrap();
//         let config = GatewayConfig {
//             authorization_api_url: "http://auth.example.com".to_string(),
//             services: vec![ServiceConfig {
//                 path: "/service".to_string(),
//                 target_service: "http://unknown.example.com".to_string(),
//                 target_port: "80".to_string(),
//             }],
//         };
//         let res = handle_request(req, config).await.unwrap();
//         assert_eq!(res.status(), StatusCode::SERVICE_UNAVAILABLE);
//     }
// }