use crate::cli::output::messages::error::NOT_FOUND_USER_AUTH;
use crate::core::requester::{build_request, request, Method, AUTHENTICATOR_URL};
use crate::core::user_account::{get_user_account_data, user_auth_exists};
use crate::paintln;
use crate::utils::errors::{not_found_error, std_error, other_error};
use serde_derive::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use std::io::Error;
use std::time::Instant;

#[derive(Serialize, Deserialize)]
struct ChangePassRequest {
    username: String,
}

#[derive(Serialize, Deserialize)]
struct ChangePassResponse {
    ok: bool,
    message: String,
}

pub async fn run() -> Result<(), Error> {
    if !user_auth_exists() {
        return Err(not_found_error(NOT_FOUND_USER_AUTH));
    }

    let start = Instant::now(); //start timing process

    let username = get_user_account_data()?.username;
    let url = format!("{}/user/changePassword/sendLink", AUTHENTICATOR_URL);
    let body = ChangePassRequest { username };
    let req = build_request(&url, Method::POST, std_error(to_string(&body))?);

    paintln!("{gray}", "[sending form link]");
    let res: ChangePassResponse = from_str(&request(req).await?)?;

    if !res.ok {
        return Err(other_error(&res.message));
    }

    println!("A link was invited to your email by \"prottern.mailer@gmail.com\".");
    println!("That link will redirect you to a form for you to change your password.");

    let end = Instant::now(); // stop timing process
    println!("Done in {:.2?}", end.duration_since(start));

    Ok(())
}
