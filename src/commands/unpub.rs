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
    if args.len() < 1 {
        let err = Error::new(ErrorKind::InvalidInput, "Template name must be specified.");
        return Err(err);
    }

    let template_name = &args[0];
    let current_user = UserAccountManager::get_user_account_data().unwrap();

    let body = UnpubRequestBody {
        template_name: template_name.to_string(),
        user: current_user.username,
    };

    let body_as_string = serde_json::to_string(&body).unwrap();

    let mut req =
        ProtternRequester::build_request("/templates/unpub", Method::POST, body_as_string);
    let headers = req.headers_mut();
    headers.insert(
        "authorization",
        HeaderValue::from_str(current_user.key.as_str()).unwrap(),
    );
    match ProtternRequester::request(req).await {
        Ok(res) => {
            let res_json: UnpubResponse = serde_json::from_str(&res).unwrap();
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
