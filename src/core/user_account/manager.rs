use super::UserAccountData;
use crate::core::utils::errors::std_error;
use crate::core::{
    file_system::{paths::USER_ACCOUNT_AUTH_PATH, write_base64_file, read_base64_file},
    requester::{build_request, request, Method},
};
use serde_derive::{Deserialize, Serialize};
use std::{fs, io::Error, path::Path};

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

pub fn save_user_account(user_account: UserAccountKey) -> Result<(), Error> {
    let content = serde_json::to_string(&user_account)?;
    write_base64_file(USER_ACCOUNT_AUTH_PATH, content)
}

pub fn get_user_account_data() -> Result<UserAccountKey, Error> {
    let user_account = read_base64_file(USER_ACCOUNT_AUTH_PATH)?;
    Ok(std_error(serde_json::from_str(&user_account))?)
}

pub async fn signup_user_account(
    user_account: &UserAccountData,
) -> Result<RegisterResponse, Error> {
    let response = {
        let body = serde_json::to_string(user_account)?;
        let req = build_request("/user/signup", Method::POST, body);
        request(req).await?
    };
    Ok(std_error(serde_json::from_str(&response))?)
}

pub async fn authenticate_user_account(
    username: String,
    password: String,
) -> Result<AuthResponse, Error> {
    let response = {
        let body = std_error(serde_json::to_string(&AuthRequestBody {
            username,
            password,
        }))?;
        let req = build_request("/user/login", Method::POST, body);
        request(req).await?
    };

    Ok(serde_json::from_str(&response)?)
}

pub fn user_auth_exists() -> bool {
    Path::new(USER_ACCOUNT_AUTH_PATH).exists()
}

pub fn logout_current() -> Result<(), Error> {
    fs::remove_file(USER_ACCOUNT_AUTH_PATH)
}