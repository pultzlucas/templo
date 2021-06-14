use crate::core::{
    io::ProtternInput,
    requester::{AuthResponse, Method, ProtternRequester},
    user_account::{UserAccountData, UserAccountManager},
};
use serde_derive::{Deserialize, Serialize};
use std::io::{Error, ErrorKind};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct UserAuth {
    username: String,
    password: String,
}

impl UserAuth {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }
}

pub async fn login() -> Result<(), Error> {
    let (username, password) = (
        ProtternInput::get("Username: ").unwrap(),
        ProtternInput::get("Password: ").unwrap(),
    );

    let user_auth = UserAuth::new(username.clone(), password.clone());

    // Authenticating user account

    let body = serde_json::to_string(&user_auth).unwrap();
    let response = ProtternRequester::request("/user/login", Method::POST, body)
        .await
        .unwrap();
    let response_json: AuthResponse = serde_json::from_str(&response).unwrap();

    if !response_json.authenticated {
        let err = Error::new(ErrorKind::AlreadyExists, response_json.message);
        return Err(err);
    }

    let user_account = UserAccountData::new(username, "".to_string(), password);
    UserAccountManager::log_user_account(user_account, response_json.key)?;

    println!("\nAccount was authenticated.");

    Ok(())
}
