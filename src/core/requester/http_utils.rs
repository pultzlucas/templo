use std::io::Error;
use hyper::{Body, Response, body::to_bytes};

use crate::utils::errors::invalid_input_error;

pub fn validate_url(url: &str) -> Result<(), Error> {
    if !str_is_url(url) {
        return Err(invalid_input_error("The url must be HTTP or HTTPS."))
    }

    Ok(())
}

pub fn str_is_url(string: &str) -> bool {
    string.starts_with("http://") || string.starts_with("https://")
}

pub async fn get_reponse_body(res: &mut Response<Body>) -> String {
    let res = res.body_mut();
    let bytes = to_bytes(res)
            .await
            .expect("Internal error when converting body response.");
        String::from_utf8(bytes.into_iter().collect())
            .expect("Internal error when converting body response.")
}