use crate::core::{
    repository::TemplateManager,
    requester::{HeaderValue, Method, ProtternRequester},
    user_account::UserAccountManager,
};
use serde_derive::{Deserialize, Serialize};
use std::io::{Error, ErrorKind};

#[derive(Deserialize, Serialize)]
struct PublishResponse {
    published: bool,
    message: String,
}

pub async fn publish(args: &[String]) -> Result<(), Error> {
    if args.len() < 1 {
        let err = Error::new(ErrorKind::InvalidInput, "Template name must be specified.");
        return Err(err);
    }

    let template_name = &args[0];

    // Getting template
    let template = match TemplateManager::get_template(template_name) {
        Err(e) => return Err(e),
        Ok(t) => t,
    };

    let current_user = match UserAccountManager::get_user_account_data() {
        Err(e) => return Err(e),
        Ok(o) => o,
    };

    
    let template_as_string = match serde_json::to_string(&template) {
        Err(e) => {
            let err = Error::new(ErrorKind::Other, e.to_string());
            return Err(err)
        },
        Ok(t) => t,
    };
    
    // Publishing template
    let mut req =
    ProtternRequester::build_request("/templates/pub", Method::POST, template_as_string);
    
    let headers = req.headers_mut();
    headers.insert(
        "authorization",
        HeaderValue::from_str(current_user.key.as_str()).expect("Error when set headers."),
    );

    match ProtternRequester::request(req).await {
        Err(e) => return Err(e),
        Ok(res) => {
            let res_json: PublishResponse = serde_json::from_str(&res).expect("Error when parsing JSON.");

            if !res_json.published {
                let err = Error::new(ErrorKind::PermissionDenied, res_json.message);
                return Err(err);
            }

            println!("{}", res_json.message);
        }
    };

    Ok(())
}
