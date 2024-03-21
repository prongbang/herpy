use hyper::http::request::Parts;
use crate::config::config::Backend;
use hyper::{Body, Client, Request, Response};

async fn build_request(
    parts: Parts,
    body: Body,
    backend: &Backend,
) -> Result<Request<Body>, hyper::Error> {
    let req = Request::from_parts(parts, body);
    let uri = format!("{}{}", backend.host, backend.path);

    let mut builder = Request::builder()
        .uri(uri)
        .method(req.method())
        .version(req.version());

    *builder.headers_mut().unwrap() = req.headers().clone();

    let body_bytes = hyper::body::to_bytes(req.into_body()).await?;

    let downstream_req = builder.body(Body::from(body_bytes));

    Ok(downstream_req.unwrap())
}

pub async fn forward(
    parts: Parts,
    body: Body,
    backend: &Backend,
) -> Result<Response<Body>, ()> {
    let req = build_request(parts, body, backend);
    if let Ok(req) = req.await {
        return match Client::new().request(req).await {
            Ok(res) => Ok(res),
            Err(_) => Err(()),
        };
    }
    return Err(());
}
