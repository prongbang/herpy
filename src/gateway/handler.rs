use std::convert::Infallible;
use std::sync::Arc;

use hyper::{Body, Request, Response, StatusCode};
use hyper::body::to_bytes;
use crate::config::GatewayConfig;
use crate::{rewriter, modsec, response, middleware};
use crate::gateway::parse_query_string;

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
    if let Some(res) = middleware::modsec::process(&req, mod_config) {
        return Ok(res);
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

                let res = rewriter::reqwest::rewrite(headers.as_ref().clone(), Body::from(body_bytes.clone()), query.as_ref().clone(), version, client, &backend).await.unwrap_or_else(|_| response::bad_gateway());
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