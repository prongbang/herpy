use std::str::FromStr;
use hyper::{Body, Response};
use hyper::http::request::Parts;
use reqwest::{Client, Method};
use crate::config::config::Backend;

pub async fn forward(
    parts: Parts,
    body: Body,
    reqwest: &Client,
    backend: &Backend,
) -> Result<Response<Body>, ()> {
    let backend_uri = format!("{}{}", &backend.host, &backend.path);
    let uri = reqwest::Url::from_str(backend_uri.as_str()).unwrap();
    let method = Method::from_str(&backend.method.as_str()).unwrap();
    let request = reqwest
        .request(method, uri)
        .headers(parts.headers)
        .body(body);

    let response = request.send();

    match response.await {
        Ok(res) => {
            let resp = Response::builder()
                .status(res.status())
                .body(Body::from(res.bytes().await.unwrap()))
                .unwrap();
            Ok(resp)
        }
        _ => Err(()),
    }
}