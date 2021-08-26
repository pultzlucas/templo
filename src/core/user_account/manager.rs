use super::UserAccountData;
use crate::core::{
    fs::{paths::get_user_auth_path, read_base64_file, write_base64_file},
    requester::{build_request, request, Method},
};
use crate::utils::errors::{other_error, std_error};
use serde_derive::{Deserialize, Serialize};
use std::{fs, io::Error, path::Path};

#[derive(Serialize, Deserialize, Debug)]
pub struct SignupResponse {
    pub registered: bool,
    pub user: Option<String>,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignupCheckResponse {
    pub message: String,
    pub ok: bool,
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
struct ConfirmAccountRequestBody {
    username: String,
    email: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfirmAccountResponse {
    token: Option<String>,
    message: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserAccountKey {
    pub username: String,
    pub key: String,
}

const AUTHENTICATOR_URL: &'static str = "https://prottern-authenticator.herokuapp.com";

pub fn save_user_account(user_account: UserAccountKey) -> Result<(), Error> {
    let content = serde_json::to_string(&user_account)?;
    write_base64_file(get_user_auth_path().unwrap(), content)
}

pub fn get_user_account_data() -> Result<UserAccountKey, Error> {
    let user_account = read_base64_file(get_user_auth_path().unwrap())?;
    Ok(std_error(serde_json::from_str(&user_account))?)
}

pub async fn request_account_confirmation(username: String, email: String) -> Result<String, Error> {
    let url = "https://prottern-authenticator.herokuapp.com/user/confirmAccount";
    let body = ConfirmAccountRequestBody { username, email };

    let req = build_request(url, Method::POST, std_error(serde_json::to_string(&body))?);
    let res: ConfirmAccountResponse = std_error(serde_json::from_str(&request(req).await?))?;

    if let Some(token) = res.token {
        return Ok(token);
    }

    if let Some(msg) = res.message {
        return Err(other_error(&msg));
    }

    panic!("Some bug has accurred during account confirmation. Please try again later.")
}

pub async fn signup_user_account(account: UserAccountData) -> Result<SignupResponse, Error> {
    let response = {
        let body = serde_json::to_string(&account)?;
        let url = format!("{}/{}", AUTHENTICATOR_URL, "user/signup");
        let req = build_request(&url, Method::POST, body);
        request(req).await?
    };

    Ok(std_error(serde_json::from_str(&response))?)
}

pub async fn check_user_signup(account: &UserAccountData) -> Result<SignupCheckResponse, Error> {
    let response = {
        let body = serde_json::to_string(account)?;
        let url = format!("{}/{}", AUTHENTICATOR_URL, "user/checkUserSignup");
        let req = build_request(&url, Method::POST, body);
        request(req).await?
    };

    Ok(std_error(serde_json::from_str(&response))?)
}

pub async fn log_user_account(username: String, password: String) -> Result<AuthResponse, Error> {
    /* let real_token = request_account_confirmation(username.clone(), email.clone()).await?;

    get_valid_input("Token: ", InputType::Text, "Invalid token.", |input| {
        input == real_token
    })?; */

    let response = {
        let body = std_error(serde_json::to_string(&AuthRequestBody {
            username,
            password,
        }))?;
        let url = format!("{}/{}", AUTHENTICATOR_URL, "user/login");
        let req = build_request(&url, Method::POST, body);
        request(req).await?
    };

    Ok(serde_json::from_str(&response)?)
}

pub fn user_auth_exists() -> bool {
    Path::new(&get_user_auth_path().unwrap()).exists()
}

pub fn logout_current() -> Result<(), Error> {
    fs::remove_file(get_user_auth_path().unwrap())
}
