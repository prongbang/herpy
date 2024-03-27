use std::str::FromStr;
use std::time::Duration;
use hyper::{Body, Response};
use hyper::body::Bytes;
use hyper::http::request::Parts;
use reqwest::{Method};
use crate::config::Backend;

pub async fn forward(
    parts: Parts,
    body: Body,
    client: &reqwest::Client,
    backend: &Backend,
) -> Result<Response<Body>, ()> {
    let backend_uri = format!("{}{}", &backend.host, &backend.path);
    let uri = reqwest::Url::from_str(backend_uri.as_str()).unwrap();
    let method = Method::from_str(&backend.method.as_str()).unwrap();
    let request = client
        .request(method, uri)
        .timeout(Duration::from_secs(backend.timeout.unwrap_or(30)))
        .headers(parts.headers)
        .version(parts.version)
        .body(body);

    let response = request.send();

    match response.await {
        Ok(res) => {
            let headers = res.headers().clone();
            let mut resp = Response::builder()
                .status(res.status())
                .body(Body::from(res.bytes().await.unwrap_or(Bytes::new())))
                .unwrap();
            *resp.headers_mut() = headers;

            Ok(resp)
        }
        Err(e) => {
            println!("[Herpy] {:?}", e);
            Err(())
        }
    }
}