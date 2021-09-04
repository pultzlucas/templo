use std::io::Error;
use crate::utils::errors::invalid_input_error;

pub fn validate_url(url: &str) -> Result<&str, Error> {
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err(invalid_input_error("The url must be HTTP or HTTPS."))
    }

    Ok(url)
}