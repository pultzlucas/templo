use hyper::{body::to_bytes, Body, Client, Method, Request};

const BASE_URL: &str = "http://localhost:8081";

pub struct ProtternRequester;

impl ProtternRequester {
    pub async fn request(
        route: &str,
        method: Method,
        body: String,
    ) -> Result<String, std::io::Error> {
        let url = format!("{}{}", BASE_URL, route);
        let req = Request::builder()
            .method(method)
            .uri(url)
            .header("content-type", "application/json")
            .body(Body::from(body))
            .expect("Internal error when building request.");

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
