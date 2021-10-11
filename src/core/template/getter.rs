use serde_json::from_str;

use crate::utils::errors::{invalid_data_error, other_error, std_error};
use std::io::Error;

use super::http::{build_request, get_reponse_body, request, HeaderValue, Method};
use super::Template;
use serde_derive::{Deserialize, Serialize};

pub struct TemplateGettedData {
    pub template: Template,
    pub message: Option<String>,
}

#[derive(Deserialize, Serialize)]
struct Extra {
    message: String,
    is_error: bool,
}

#[derive(Deserialize, Serialize)]
struct ResponseData {
    extra: Option<Extra>,
    template: Option<Template>,
}

pub async fn get_remote_template(
    url: &str,
    key: Option<String>,
) -> Result<TemplateGettedData, Error> {
    let mut req = build_request(url, Method::GET, None);

    if let Some(key) = key {
        req.headers_mut().insert(
            "authorization",
            HeaderValue::from_str(&key).expect("Error when set headers."),
        );
    }

    let mut res = request(req).await?;
    let res_body = get_reponse_body(&mut res).await;

    if from_str::<ResponseData>(&res_body).is_err() {
        return Err(invalid_data_error("The server returns an invalid data."));
    }

    let ResponseData { extra, template } = std_error(from_str(&res_body))?;

    if let Some(extra) = &extra {
        if extra.is_error {
            return Err(other_error(&extra.message));
        }

        if !extra.is_error && template.is_none() {
            return Err(invalid_data_error(
                "Could not find a template in server response.",
            ));
        }
    }

    let message = if extra.is_some() {
        Some(extra.unwrap().message)
    } else {
        None
    };

    let template = template.unwrap();

    Ok(TemplateGettedData { template, message })
}
