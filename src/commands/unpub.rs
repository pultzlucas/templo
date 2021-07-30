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
    templates_name: Vec<String>,
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
    let templates_name = &args[0..];
    let requester = ProtternRequester::new();


    // Unpub templates
    let current_user = UserAccountManager::get_user_account_data()?;
    let body = {
        let body = UnpubRequestBody {
            templates_name: templates_name.to_vec(),
            user: current_user.username,
        };
        serde_json::to_string(&body).expect("Error when parsing request body to string.")
    };

    let req = {
        let mut req = requester.build_request("/templates/unpub", Method::POST, body);
        req.headers_mut().insert(
            "authorization",
            HeaderValue::from_str(current_user.key.as_str()).expect("Error when set headers."),
        );
        req
    };
    paintln!("{gray}", "[Unpublishing Templates]");
    let response = requester.request(req).await?;

    let res_json: UnpubResponse =
        serde_json::from_str(&response).expect("Error when parsing JSON.");

    if !res_json.unpublished {
        return Err(Error::new(ErrorKind::Other, res_json.message));
    }

    println!("{}", res_json.message);

    Ok(())
}
