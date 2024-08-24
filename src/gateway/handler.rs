use std::convert::Infallible;
use std::sync::Arc;

use hyper::{Body, Request, Response, StatusCode, Version};
use hyper::body::to_bytes;
use crate::config::GatewayConfig;
use crate::{forwarder, modsec, response};
use crate::gateway::parse_query_string;

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

pub async fn request(
    req: Request<Body>,
    config: Arc<GatewayConfig>,
    mod_config: Arc<modsec::config::Config>,
    client: Arc<reqwest::Client>,
) -> Result<Response<Body>, Infallible> {
    let res = handle_inner(req, &config, &mod_config, &client).await.unwrap_or_else(|err| {
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
    mod_config: &modsec::config::Config,
    client: &reqwest::Client,
) -> Result<Response<Body>, anyhow::Error> {
    let uri = req.uri().clone();
    let query = Arc::new(parse_query_string(uri.query().unwrap_or("")));
    let path = uri.path();

    // ModSecurity WAF
    if mod_config.enabled {
        let mut tx = mod_config.mod_security.transaction_builder()
            .with_rules(&mod_config.rules)
            .build()
            .expect("Error building transaction");

        let method = req.method().clone();
        let version = req.version().clone();
        let headers = req.headers().clone();
        for header in headers {
            if let Some(key) = header.0 {
                if key.as_str() != "host" {
                    let value = header.1.to_str().unwrap_or("");
                    tx.add_request_header(key.as_str(), value).unwrap_or(());
                }
            }
        }

        let url = urlencoding::decode(uri.to_string().as_str()).expect("UTF-8").to_string();
        _ = tx.process_uri(url.as_str(), method.as_str(), http_version(version));
        _ = tx.process_request_headers();
        let intervention = tx.intervention();
        _ = tx.process_logging();
        if let Some(it) = intervention {
            if it.status() >= 300 {
                let logs = it.log().unwrap_or("");
                let status = StatusCode::from_u16(it.status() as u16).unwrap();
                let res = hyper::Response::builder()
                    .status(status)
                    .body(hyper::Body::from(status.to_string()))
                    .unwrap();
                tracing::error!(error = format!("{:?}", logs));
                return Ok(res);
            }
        }
    }

    // Rewrite
    if let Some(service_map) = &config.services_map {
        if let Some(service) = service_map.get(path) {
            let headers = Arc::new(req.headers().clone());
            let version = req.version();
            let body_bytes = to_bytes(req.into_body()).await?;

            let is_single = service.backends.len() == 1;
            let mut resp: Option<Response<Body>> = None;
            for backend in &service.backends {
                let headers = Arc::clone(&headers);
                let query = Arc::clone(&query);

                let res = forwarder::reqwest::forward(headers.as_ref().clone(), Body::from(body_bytes.clone()), query.as_ref().clone(), version, client, &backend).await.unwrap_or_else(|_| response::bad_gateway());
                if is_single {
                    return Ok(res);
                }

                if resp.is_none() {
                    resp = Some(res);
                }
            }
            return Ok(resp.unwrap());
        }
    }

    Ok(response::not_found())
}
