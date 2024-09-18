use hyper::{Body, Request, Response, StatusCode};
use crate::config::http_version;
use crate::modsec::config::Config;

pub fn process(req: &Request<Body>, mod_config: &Config) -> Option<Response<Body>> {
    if mod_config.enabled {
        let mut tx = mod_config.mod_security.transaction_builder()
            .with_rules(&mod_config.rules)
            .with_logging(|msg| {
                if let Some(msg) = msg {
                    tracing::error!(error = msg);
                }
            })
            .build()
            .expect("Error building transaction");

        let method = req.method().clone();
        let version = req.version().clone();
        let headers = req.headers().clone();
        for header in headers {
            if let Some(key) = header.0 {
                let value = header.1.to_str().unwrap_or("");
                tx.add_request_header(key.as_str(), value).unwrap_or(());
            }
        }

        let uri = req.uri().clone();
        let url = urlencoding::decode(uri.to_string().as_str()).expect("UTF-8").to_string();
        _ = tx.process_uri(url.as_str(), method.as_str(), http_version(version));
        _ = tx.process_request_headers();
        let intervention = tx.intervention();
        _ = tx.process_logging();
        if let Some(it) = intervention {
            if it.status() >= 300 {
                let status = StatusCode::from_u16(it.status() as u16).unwrap();
                let res = hyper::Response::builder()
                    .status(status)
                    .body(hyper::Body::from(status.to_string())).unwrap();
                return Some(res);
            }
        }
    }

    None
}
