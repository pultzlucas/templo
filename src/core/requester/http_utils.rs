use std::io::Error;
use crate::utils::errors::invalid_input_error;

pub fn validate_url(url: &str) -> Result<&str, Error> {
    if !str_is_url(url) {
        return Err(invalid_input_error("The url must be HTTP or HTTPS."))
    }

    Ok(url)
}

pub fn str_is_url(string: &str) -> bool {
    string.starts_with("http://") || string.starts_with("https://")
}