use crate::core::requester::{build_request, get_reponse_body, request, HeaderValue, Method};
use crate::utils::errors::{invalid_data_error, other_error, std_error};
use crate::utils::string::str_to_bool;
use std::io::Error;

use super::Template;

pub struct GetTemplateResponse {
    pub template: Template,
    pub message: Option<String>,
}

pub async fn get_remote_template(
    url: &str,
    key: Option<String>,
) -> Result<GetTemplateResponse, Error> {
    let mut req = build_request(url, Method::GET, None);

    if let Some(key) = key {
        req.headers_mut().insert(
            "authorization",
            HeaderValue::from_str(&key).expect("Error when set headers."),
        );
    }

    let mut res = request(req).await?;
    let res_body = get_reponse_body(&mut res).await;

    // check if template data is valid
    if serde_json::from_str::<Template>(&res_body).is_err() {
        return Err(invalid_data_error(
            "The remote repo returned an invalid template.",
        ));
    }

    // catch response message
    let message = if res.headers().contains_key("message") {
        let is_error = if res.headers().contains_key("isError") {
            let err = std_error(res.headers().get("isError").unwrap().to_str())?;
            str_to_bool(err)
        } else {
            false
        };

        let msg = std_error(res.headers().get("message").unwrap().to_str())?;

        if is_error {
            return Err(other_error(msg));
        }

        Some(msg.to_string())
    } else {
        None
    };

    Ok(GetTemplateResponse {
        template: std_error(serde_json::from_str(&res_body))?,
        message,
    })
}
