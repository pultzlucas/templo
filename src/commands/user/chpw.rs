use std::io::Error;
use crate::core::requester::{build_request, request, Method, AUTHENTICATOR_URL};
use crate::core::user_account::get_user_account_data;
use crate::utils::errors::std_error;
use serde_json::to_string;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct ChangePassRequest {
    username: String
}

pub async fn run() -> Result<(), Error> {
    let username = get_user_account_data()?.username;
    let url = format!("{}/user/changePassword", AUTHENTICATOR_URL);
    let body = ChangePassRequest {username};
    let req = build_request(&url, Method::POST, std_error(to_string(&body))?);
    let res = request(req).await?;
    println!("{}", res);
    Ok(())
}