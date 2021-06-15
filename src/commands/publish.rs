use crate::core::{
    repository::TemplateManager,
    requester::{Method, ProtternRequester, HeaderValue},
    user_account::UserAccountManager,
};
use std::io::{Error, ErrorKind};
use serde_derive::{Deserialize, Serialize};

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
    let template = TemplateManager::get_template(template_name).unwrap();
    let owner = UserAccountManager::get_user_account_data().unwrap();
    let template_as_string = serde_json::to_string(&template).unwrap();

    // Publishing template
    let mut req = ProtternRequester::build_request("/templates/pub", Method::POST, template_as_string);

    let headers = req.headers_mut();
    headers.insert("authorization", HeaderValue::from_str(owner.key.as_str()).unwrap());

    match ProtternRequester::request(req).await {
        Err(e) => return Err(e),
        Ok(res) => {
            let res_json: PublishResponse = serde_json::from_str(&res).unwrap();

            if !res_json.published {
                let err = Error::new(ErrorKind::PermissionDenied, res_json.message);
                return Err(err);
            }

            println!("{}", res_json.message);
        }
    };

    Ok(())
}
