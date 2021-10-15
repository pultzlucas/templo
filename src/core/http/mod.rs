mod http_utils;

pub use http_utils::*;
pub use hyper::http::HeaderValue;
pub use hyper::Method;

use hyper::{Body, Client, Request as Req, Response};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};
use std::io::{Error, ErrorKind};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct NotFoundResponse {
    message: String,
}

pub fn build_request(url: &str, method: Method, body_op: Option<String>) -> Req<hyper::Body> {
    let mut body = String::new();
    
    if let Some(body_op) = body_op {
        body = body_op;
    }

    let req = Req::builder();
    req.method(method)
        .uri(url)
        .header("content-type", "application/json")
        .body(Body::from(body))
        .expect("Internal error when building request.")
}


pub async fn request(req: Req<hyper::Body>) -> Result<Response<Body>, Error> {
    let response = {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        match client.request(req).await {
            Err(e) => return Err(Error::new(ErrorKind::ConnectionAborted, e.to_string())),
            Ok(r) => r,
        }
    };

    Ok(response)
}