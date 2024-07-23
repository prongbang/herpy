use std::convert::Infallible;
use std::sync::Arc;

use hyper::{Body, Request, Response, StatusCode};
use hyper::body::to_bytes;
use hyper::http::request::Parts;
use crate::config::GatewayConfig;
use crate::{forwarder, response};
use crate::gateway::parse_query_string;

pub async fn request(
    req: Request<Body>,
    config: Arc<GatewayConfig>,
    client: Arc<reqwest::Client>,
) -> Result<Response<Body>, Infallible> {
    let res = handle_inner(req, &config, &client).await.unwrap_or_else(|err| {
        tracing::error!(error = format!("{err:#?}"), "could not process request");

        hyper::Response::builder()
            .status(StatusCode::BAD_GATEWAY)
            .body(hyper::Body::from(err.to_string()))
            .unwrap()
    });

    Ok(res)
}

async fn handle_inner(
    req: Request<Body>,
    config: &GatewayConfig,
    client: &reqwest::Client,
) -> Result<Response<Body>, anyhow::Error> {
    let uri = req.uri().clone();
    let query = parse_query_string(uri.query().unwrap_or(""));
    let path = uri.path();

    if let Some(service_map) = &config.services_map {
        if let Some(service) = service_map.get(path) {
            let headers = req.headers().clone();
            let version = req.version().clone();
            let body = req.into_parts();

            // Convert the body to bytes
            let body_bytes = match to_bytes(body.1).await {
                Ok(bytes) => bytes,
                Err(_) => return Ok(response::bad_gateway()),
            };

            let mut resp: Option<Response<Body>> = None;
            for backend in &service.backends {
                let headers = headers.clone();
                let query = query.clone();

                // Convert bytes to String
                let body = Body::from(body_bytes.to_vec());

                let res = forwarder::reqwest::forward(headers, body, query, version, client, &backend).await.unwrap_or_else(|_| response::bad_gateway());
                if resp.is_none() {
                    resp = Some(res);
                }
            }
            return Ok(resp.unwrap());
        }
    }

    Ok(response::not_found())
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