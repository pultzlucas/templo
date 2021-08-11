use crate::core::requester::{build_request, request, HeaderValue, Method};
use crate::core::template::{TempMetadata, Template};
use crate::core::user_account::get_user_account_data;
use crate::core::utils::errors::{other_error, permission_denied_error, std_error};
use serde_derive::{Deserialize, Serialize};
use std::io::Error;

#[derive(Deserialize, Serialize)]
struct PublishResponse {
    published: bool,
    message: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct GetRequestBody {
    templates_name: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct GetResponseBody {
    message: String,
    getted: bool,
    templates: Vec<Template>,
}

#[derive(Deserialize, Serialize)]
struct UnpubRequestBody {
    templates_name: Vec<String>,
    user: String,
}

#[derive(Deserialize, Serialize)]
struct UnpubResponse {
    unpublished: bool,
    message: String,
}

pub async fn publish_templates(templates: Vec<Template>) -> Result<String, Error> {
    let current_user = get_user_account_data()?;

    let req = {
        let body = std_error(serde_json::to_string(&templates))?;
        let mut req = build_request("/templates/pub", Method::POST, body);
        let headers = req.headers_mut();
        headers.insert(
            "authorization",
            std_error(HeaderValue::from_str(current_user.key.as_str()))?,
        );
        req
    };

    let response: PublishResponse = {
        let res = request(req).await?;
        std_error(serde_json::from_str(&res))?
    };
    if !response.published {
        return Err(permission_denied_error(&response.message));
    }
    Ok(response.message)
}

pub async fn get_templates(temps_name: Vec<String>) -> Result<Vec<Template>, Error> {
    let body_string: String = std_error(serde_json::to_string(&temps_name))?;
    let req = build_request("/templates/get", Method::GET, body_string);
    let res_string = request(req).await?;
    let res: GetResponseBody = std_error(serde_json::from_str(&res_string))?;
    Ok(res.templates)
}

pub async fn unpub_templates(temps_name: Vec<String>) -> Result<String, Error> {
    let req = {
        let current_user = get_user_account_data()?;
        let body = {
            let body = UnpubRequestBody {
                templates_name: temps_name.to_vec(),
                user: current_user.username,
            };
            std_error(serde_json::to_string(&body))?
        };
        let mut req = build_request("/templates/unpub", Method::POST, body);

        // set headers
        req.headers_mut().insert(
            "authorization",
            std_error(HeaderValue::from_str(current_user.key.as_str()))?,
        );
        req
    };

    let response = request(req).await?;
    let res_json: UnpubResponse = std_error(serde_json::from_str(&response))?;

    if !res_json.unpublished {
        return Err(other_error(&res_json.message));
    }

    Ok(res_json.message)
}

pub async fn get_all_templates() -> Result<Vec<TempMetadata>, Error> {
    let req = build_request("/templates", Method::GET, "".to_owned());
    let response = request(req).await?;
    let templates: Vec<Template> = std_error(serde_json::from_str(&response))?;
    let temps_metadata: Vec<TempMetadata> =
        templates.into_iter().map(|temp| temp.metadata).collect();
    Ok(temps_metadata)
}
