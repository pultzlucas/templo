use hyper::{Body, Method, Request, Client, body::to_bytes};
use serde_derive::{Deserialize, Serialize};
use std::io::{stdin, stdout, Error, ErrorKind, Write};
use crate::core::user_account::{UserAccountManager, UserAccountData};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct UserAuth {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct AuthResponse {
    authenticated: bool,
    key: String,
    message: String,
}

impl UserAuth {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }
}

pub async fn login() -> Result<(), Error> {
    let (username, password) = (
        ask_field("Username: ").unwrap(),
        ask_field("Password: ").unwrap(),
    );

    let user_auth = UserAuth::new(username.clone(), password.clone());

    let req = Request::builder()
        .method(Method::POST)
        .uri("http://localhost:8081/user/login")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&user_auth).unwrap()))
        .unwrap();

    let client = Client::new();

    let res = client.request(req).await.unwrap();
    let bytes = to_bytes(res.into_body()).await.unwrap();
    let data = String::from_utf8(bytes.into_iter().collect()).unwrap();
    let response_json: AuthResponse = serde_json::from_str(&data).unwrap();

    if !response_json.authenticated {
        let err = Error::new(ErrorKind::AlreadyExists, response_json.message);
        return Err(err);
    }

    let user_account = UserAccountData::new(username, "".to_string(), password);

    UserAccountManager::log_user_account(user_account, response_json.key)?;

    println!("\nAccount was authenticated.");

    Ok(())
}

fn ask_field(text: &str) -> Result<String, Error> {
    print!("{}", text);
    stdout().flush().unwrap();

    let mut info = String::new();
    if let Err(e) = stdin().read_line(&mut info) {
        return Err(e);
    }

    Ok(info.trim().to_string())
}
