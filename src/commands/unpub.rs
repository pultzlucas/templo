use crate::{
    core::{
        requester::{HeaderValue, Method, ProtternRequester},
        user_account::UserAccountManager,
    },
    paintln
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
If you already have a user account created, type "prottern login" to authenticate it."#,
        );
        return Err(err);
    }

    if args.len() < 1 {
        let err = Error::new(ErrorKind::InvalidInput, "Template name must be specified.");
        return Err(err);
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

    match ProtternRequester::request(request).await {
        Ok(res) => {
            let res_json: UnpubResponse =
                serde_json::from_str(&res).expect("Error when parsing JSON.");
            if !res_json.unpublished {
                return Err(Error::new(ErrorKind::Other, res_json.message));
            }

            println!("{}", res_json.message);
        }

        Err(e) => return Err(e),
    }

    Ok(())
}
