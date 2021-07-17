use super::user_account_data::UserAccountData;
use crate::core::{
    file_system::{paths::USER_ACCOUNT_AUTH_PATH, ProtternFileSystem},
    requester::{Method, ProtternRequester},
};
use serde_derive::{Deserialize, Serialize};
use std::{io::Error, path::Path, fs};

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterResponse {
    pub registered: bool,
    pub user: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthResponse {
    pub authenticated: bool,
    pub user: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AuthRequestBody {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserAccountKey {
    pub username: String,
    pub email: String,
    pub password: String,
    pub key: String,
}

pub struct UserAccountManager {}

impl UserAccountManager {
    pub fn save_user_account(user_account: UserAccountKey) -> Result<(), Error> {
        let content = serde_json::to_string(&user_account)?;
        ProtternFileSystem::write_base64_file(USER_ACCOUNT_AUTH_PATH, content)
    }

    pub fn get_user_account_data() -> Result<UserAccountKey, Error> {
        let user_account = ProtternFileSystem::read_base64_file(USER_ACCOUNT_AUTH_PATH)?;
        Ok(serde_json::from_str(&user_account).expect("Error when parsing user account object."))
    }

    pub async fn signup_user_account(
        user_account: &UserAccountData,
    ) -> Result<RegisterResponse, Error> {
        let response = {
            let body = serde_json::to_string(user_account)?;
            let requester = ProtternRequester::new();
            let request = requester.build_request("/user/signup", Method::POST, body);
            requester.request(request).await?
        };
        Ok(serde_json::from_str(&response).unwrap())
    }

    pub async fn authenticate_user_account(
        username: String,
        password: String,
    ) -> Result<AuthResponse, Error> {
        let response = {
            let body = serde_json::to_string(&AuthRequestBody { username, password }).unwrap();
            let requester = ProtternRequester::new();
            let request = requester.build_request("/user/login", Method::POST, body);
            requester.request(request).await?
        };

        Ok(serde_json::from_str(&response)?)
    }

    pub fn user_auth_exists() -> bool {
        Path::new(USER_ACCOUNT_AUTH_PATH).exists()
    }

    pub fn logout_current() -> Result<(), Error> {
        fs::remove_file(USER_ACCOUNT_AUTH_PATH)
    }
}
