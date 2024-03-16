use hyper::header::HeaderValue;
use hyper::http::request::Parts;
use hyper::{Body, Client, HeaderMap, Request, Response};
use hyper::header::AUTHORIZATION;
use crate::config::config::{Backend, GatewayConfig, Service};

pub async fn request(
    req: Request<Body>,
    config: GatewayConfig,
) -> Result<Response<Body>, hyper::Error> {
    let path = req.uri().path();
    let service_config = match get_service_config(path, &config.services) {
        Some(service_config) => service_config,
        None => {
            return not_found();
        }
    };

    // let auth_token = match authorize_user(&req.headers(), &config.authorization_api_url).await {
    //     Ok(header) => header,
    //     Err(_) => {
    //         return service_unavailable("Failed to connect to Authorization API {}");
    //     }
    // };

    let backend = service_config.backends.first().unwrap();

    let (parts, body) = req.into_parts();
    let downstream_req = build_downstream_request(parts, body, &backend, "auth_token".to_string()).await?;

    match forward_request(downstream_req).await {
        Ok(res) => Ok(res),
        Err(_) => service_unavailable("Failed to connect to downstream service"),
    }
}

fn get_service_config<'a>(path: &str, services: &'a [Service]) -> Option<&'a Service> {
    services.iter().find(|c| path.starts_with(&c.endpoint))
}

async fn authorize_user(headers: &HeaderMap, auth_api_url: &str) -> Result<String, ()> {
    let auth_header_value = match headers.get(AUTHORIZATION) {
        Some(value) => value.to_str().unwrap_or_default(),
        None => "",
    };

    let auth_request = reqwest::Client::new()
        .get(auth_api_url)
        .header(AUTHORIZATION, auth_header_value);

    println!("{}", auth_api_url);

    match auth_request.send().await {
        Ok(res) if res.status().is_success() => Ok(auth_header_value.to_string()),
        _ => Err(()),
    }
}

async fn build_downstream_request(
    parts: Parts,
    body: Body,
    backend: &Backend,
    auth_token: String,
) -> Result<Request<Body>, hyper::Error> {
    let req = Request::from_parts(parts, body);
    let uri = format!("{}{}", backend.host, backend.path);

    let mut downstream_req_builder = Request::builder()
        .uri(uri)
        .method(req.method())
        .version(req.version());

    *downstream_req_builder.headers_mut().unwrap() = req.headers().clone();

    downstream_req_builder
        .headers_mut()
        .unwrap()
        .insert("Authorization", HeaderValue::from_str(&auth_token).unwrap());

    let body_bytes = hyper::body::to_bytes(req.into_body()).await?;

    let downstream_req = downstream_req_builder.body(Body::from(body_bytes));

    Ok(downstream_req.unwrap())
}

async fn forward_request(req: Request<Body>) -> Result<Response<Body>, ()> {
    match Client::new().request(req).await {
        Ok(res) => Ok(res),
        Err(_) => Err(()),
    }
}

fn not_found() -> Result<Response<Body>, hyper::Error> {
    let mut response = Response::new(Body::from("404 Not Found"));
    *response.status_mut() = hyper::StatusCode::NOT_FOUND;
    Ok(response)
}

fn service_unavailable<T>(reason: T) -> Result<Response<Body>, hyper::Error>
    where
        T: Into<Body>,
{
    let mut response = Response::new(reason.into());
    *response.status_mut() = hyper::StatusCode::SERVICE_UNAVAILABLE;
    Ok(response)
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