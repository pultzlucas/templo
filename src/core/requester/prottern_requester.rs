use hyper::{
    body::to_bytes,
    Body, Client, Method, Request,
};

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

    pub async fn request(req: hyper::Request<hyper::Body>) -> Result<String, std::io::Error> {
        let client = Client::new();
        let res = client
            .request(req)
            .await
            .expect("Internal error when requesting rest api.");
        let bytes = to_bytes(res.into_body())
            .await
            .expect("Internal error when converting body response.");
        let data = String::from_utf8(bytes.into_iter().collect())
            .expect("Internal error when converting body response.");
        Ok(data)
    }
}
