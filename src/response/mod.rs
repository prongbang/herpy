use hyper::{Body, Response};

pub fn not_found() -> Result<Response<Body>, hyper::Error> {
    let mut response = Response::new(Body::from("404 Not Found"));
    *response.status_mut() = hyper::StatusCode::NOT_FOUND;
    Ok(response)
}

pub fn service_unavailable<T>(reason: T) -> Result<Response<Body>, hyper::Error>
    where
        T: Into<Body>,
{
    let mut response = Response::new(reason.into());
    *response.status_mut() = hyper::StatusCode::SERVICE_UNAVAILABLE;
    Ok(response)
}