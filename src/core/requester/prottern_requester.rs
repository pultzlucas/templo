use hyper::{body::to_bytes, Body, Client, Method, Request};
use std::io::{Error, ErrorKind};

const BASE_URL: &str = "http://localhost:8081";

pub struct ProtternRequester;

impl ProtternRequester {
    pub fn build_request(route: &str, method: Method, body: String) -> Request<hyper::Body> {
        let url = format!("{}{}", BASE_URL, route);

        let req = Request::builder();
        let req = req
            .method(method)
            .uri(url)
            .header("content-type", "application/json")
            .body(Body::from(body))
            .expect("Internal error when building request.");

        req
    }

    pub async fn request(req: hyper::Request<hyper::Body>) -> Result<String, Error> {
        let client = Client::new();
        let res = match client.request(req).await {
            Err(e) => {
                let err = Error::new(ErrorKind::ConnectionAborted, e.to_string());
                return Err(err)
            },
            Ok(r) => r,
        };
        let bytes = to_bytes(res.into_body())
            .await
            .expect("Internal error when converting body response.");
        let data = String::from_utf8(bytes.into_iter().collect())
            .expect("Internal error when converting body response.");
        Ok(data)
    }
}
