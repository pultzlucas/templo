use super::user_account_data::UserAccountData;
use crate::core::{
    file_system::paths::USER_ACCOUNT_PATH,
    requester::{Method, ProtternRequester},
};
use serde_derive::{Deserialize, Serialize};
use std::{fs, io::Error};

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterResponse {
    pub registered: bool,
    pub key: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthResponse {
    pub authenticated: bool,
    pub key: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserAccountKey {
    pub username: String,
    pub email: String,
    password: String,
    pub key: String,
}

impl UserAccountKey {
    pub fn new(username: String, email: String, password: String, key: String) -> Self {
        Self {
            username: username,
            email: email,
            password: password,
            key,
        }
    }
}

pub struct UserAccountManager {}

impl UserAccountManager {
    pub fn log_user_account(fields: UserAccountData, key: String) -> Result<(), Error> {
        let user_account =
            UserAccountKey::new(fields.username, fields.email, fields.password, key);
        let content = serde_json::to_string(&user_account)?;
        fs::write(USER_ACCOUNT_PATH, content)
    }

    pub fn get_user_account_data() -> Result<UserAccountKey, serde_json::Error> {
        let user_account = fs::read_to_string(USER_ACCOUNT_PATH).unwrap();
        serde_json::from_str(&user_account)
    }

    pub async fn register_user_account(
        user_account: &UserAccountData,
    ) -> Result<RegisterResponse, Error> {
        let body = serde_json::to_string(user_account).unwrap();
        let req = ProtternRequester::build_request("/user/register", Method::POST, body);
        let response = ProtternRequester::request(req).await.unwrap();
        Ok(serde_json::from_str(&response).unwrap())
    }

    pub async fn authenticate_user_account(
        user_account: &UserAccountData,
    ) -> Result<AuthResponse, Error> {
        let body = serde_json::to_string(user_account).unwrap();
        let req = ProtternRequester::build_request("/user/login", Method::POST, body);
        let response = ProtternRequester::request(req).await.unwrap();

        Ok(serde_json::from_str(&response).unwrap())
    }
}
