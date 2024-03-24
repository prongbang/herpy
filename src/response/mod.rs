use hyper::{Body, Response};

pub fn not_found() -> Response<Body> {
    let resp = Response::builder()
        .status(hyper::StatusCode::NOT_FOUND)
        .body(hyper::Body::from("Not Found".to_string()))
        .unwrap();
    resp
}

pub fn bad_gateway() -> Response<Body> {
    let resp = Response::builder()
        .status(hyper::StatusCode::BAD_GATEWAY)
        .body(hyper::Body::from("Failed to connect to downstream service".to_string()))
        .unwrap();
    resp
}

pub fn service_unavailable<T>(reason: T) -> Response<Body>
    where
        T: Into<Body>,
{
    let mut response = Response::new(reason.into());
    *response.status_mut() = hyper::StatusCode::SERVICE_UNAVAILABLE;
    response
}