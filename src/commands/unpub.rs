use crate::{
    core::{
        io::messages::error::{INVALID_TEMPLATE_NAME, NOT_FOUND_USER_AUTH},
        requester::{HeaderValue, Method, ProtternRequester},
        user_account::UserAccountManager,
    },
    paintln,
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
        return Err(Error::new(ErrorKind::NotFound, NOT_FOUND_USER_AUTH));
    }

    if args.len() < 1 {
        return Err(Error::new(ErrorKind::InvalidInput, INVALID_TEMPLATE_NAME));
    }

    let current_user = UserAccountManager::get_user_account_data()?;

    let body = {
        let body = UnpubRequestBody {
            template_name: args[0].to_string(),
            user: current_user.username,
        };
        match serde_json::to_string(&body) {
            Err(e) => return Err(Error::new(ErrorKind::Other, e.to_string())),
            Ok(t) => t,
        }
    };

    let request = {
        let mut request = ProtternRequester::build_request("/templates/unpub", Method::POST, body);
        request.headers_mut().insert(
            "authorization",
            HeaderValue::from_str(current_user.key.as_str()).expect("Error when set headers."),
        );

        request
    };

    paintln!("{gray}", "[Unpublishing Template]");

    let response = ProtternRequester::request(request).await?;

    let res_json: UnpubResponse =
        serde_json::from_str(&response).expect("Error when parsing JSON.");
    if !res_json.unpublished {
        return Err(Error::new(ErrorKind::Other, res_json.message));
    }

    println!("{}", res_json.message);

    Ok(())
}
