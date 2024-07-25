use std::collections::HashMap;
use std::str::FromStr;
use std::time::Duration;

use hyper::{Body, HeaderMap, Response, Version};
use hyper::body::Bytes;
use hyper::header::HeaderName;
use reqwest::Method;

use crate::config::Backend;

pub async fn forward(
    headers: HeaderMap,
    body: Body,
    query: HashMap<String, String>,
    version: Version,
    client: &reqwest::Client,
    backend: &Backend,
) -> Result<Response<Body>, ()> {
    let backend_uri = format!("{}{}", &backend.host, &backend.path);
    let uri = reqwest::Url::from_str(backend_uri.as_str()).unwrap();
    let method = Method::from_str(&backend.method.as_str()).unwrap();
    let mut headers = headers;
    headers.remove(HeaderName::from_static("host"));

    let request = client
        .request(method, uri)
        .timeout(Duration::from_secs(backend.timeout.unwrap_or(30)))
        .query(&query)
        .headers(headers)
        .version(version)
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
            tracing::error!(error = format!("{:?}", e));
            Err(())
        }
    }
}