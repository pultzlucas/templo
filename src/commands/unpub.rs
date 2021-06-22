use crate::core::{
    requester::{HeaderValue, Method, ProtternRequester},
    user_account::UserAccountManager,
};
use serde_derive::{Deserialize, Serialize};
use std::io::{Error, ErrorKind};

#[derive(Deserialize, Serialize)]
struct UnpubRequestBody {
    template_name: String,
    user: String,
}

#[derive(Deserialize, Serialize)]
struct UnpubResponse {
    unpublished: bool,
    message: String,
}

pub async fn unpub(args: &[String]) -> Result<(), Error> {

    if !UserAccountManager::user_auth_exists() {
        let err = Error::new(
            ErrorKind::NotFound, 
            r#"This process cannot be runned because You dont has an authenticated user account.
Please type "prottern register" to register one.
If you already have a user account created, type "prottern login" to authenticate it."#
        );
        return Err(err);
    }

    if args.len() < 1 {
        let err = Error::new(ErrorKind::InvalidInput, "Template name must be specified.");
        return Err(err);
    }

    let template_name = &args[0];
    let current_user = match UserAccountManager::get_user_account_data() {
        Err(e) => return Err(e),
        Ok(u) => u,
    };

    let body = UnpubRequestBody {
        template_name: template_name.to_string(),
        user: current_user.username,
    };

    let body_as_string = match serde_json::to_string(&body) {
        Err(e) => {
            let err = Error::new(ErrorKind::Other, e.to_string());
            return Err(err);
        }
        Ok(t) => t,
    };

    let mut req =
        ProtternRequester::build_request("/templates/unpub", Method::POST, body_as_string);
    let headers = req.headers_mut();
    headers.insert(
        "authorization",
        HeaderValue::from_str(current_user.key.as_str()).expect("Error when set headers."),
    );
    match ProtternRequester::request(req).await {
        Ok(res) => {
            let res_json: UnpubResponse =
                serde_json::from_str(&res).expect("Error when parsing JSON.");
            if !res_json.unpublished {
                let err = Error::new(ErrorKind::Other, res_json.message);
                return Err(err);
            }

            println!("{}", res_json.message);
        }

        Err(e) => return Err(e),
    }

    Ok(())
}
