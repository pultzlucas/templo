use hyper::{body::to_bytes, Body, Client, Method, Request as Req};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};
use std::io::{Error, ErrorKind};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct NotFoundResponse {
    message: String,
}

//const BASE_URL: &str = "https://protternio.herokuapp.com"; 
//const BASE_URL: &str = "http://localhost:8081"; 

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


pub async fn request(req: Req<hyper::Body>) -> Result<String, Error> {
    let response = {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        match client.request(req).await {
            Err(e) => return Err(Error::new(ErrorKind::ConnectionAborted, e.to_string())),
            Ok(r) => r,
        }
    };
    let is_404_error = response.status() == 404;

    let data = {
        let bytes = to_bytes(response.into_body())
            .await
            .expect("Internal error when converting body response.");
        String::from_utf8(bytes.into_iter().collect())
            .expect("Internal error when converting body response.")
    };

    if is_404_error {
        return Err({
            let err_msg: NotFoundResponse =
                serde_json::from_str(&data).expect("Error when parsing JSON.");
            Error::new(ErrorKind::NotFound, err_msg.message)
        });
    }

    Ok(data)
}