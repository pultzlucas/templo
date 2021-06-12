use crate::core::user_account::{UserAccountData, UserAccountManager};
use hyper::{body::to_bytes, Body, Client, Method, Request};
use serde_derive::{Deserialize, Serialize};
use std::io::{stdin, stdout, Error, ErrorKind, Write};

type RegisterFields = (String, String, String, String);

#[derive(Serialize, Deserialize, Debug)]
struct RegisterResponse {
    registered: bool,
    key: String,
    message: String,
}

pub async fn register() -> Result<(), Error> {
    let fields = (
        ask_field("Username: ").unwrap(),
        ask_field("Email: ").unwrap(),
        ask_field("Password: ").unwrap(),
        ask_field("Confirm your passoword: ").unwrap(),
    );

    if let Err(e) = validate_register_fields(&fields) {
        return Err(e);
    }

    let (username, email, password, _) = fields;
    let user_account = UserAccountData::new(username, email, password);

    // save user account on database
    
    let req = Request::builder()
    .method(Method::POST)
    .uri("http://localhost:8081/user/register")
    .header("content-type", "application/json")
    .body(Body::from(serde_json::to_string(&user_account).unwrap()))
    .unwrap();
    
    let client = Client::new();
    
    let res = client.request(req).await.unwrap();
    let bytes = to_bytes(res.into_body()).await.unwrap();
    let data = String::from_utf8(bytes.into_iter().collect()).unwrap();
    let response_json: RegisterResponse = serde_json::from_str(&data).unwrap();
    
    // verify if user account is valid
    if !response_json.registered {
        let err = Error::new(ErrorKind::AlreadyExists, response_json.message);
        return Err(err);
    }

    UserAccountManager::log_user_account(&user_account)?;

    println!("\nAccount was registered.");

    Ok(())
}

fn validate_register_fields(
    (username, email, password, password2): &RegisterFields,
) -> Result<(), Error> {
    let err = |msg: &str| Err(Error::new(ErrorKind::InvalidInput, msg));

    if username.len() > 30 {
        return err("The username should not have more than 30 characters.");
    }

    if email.len() > 30 {
        return err("The email should not have more than 30 characters.");
    }
    if password.len() > 30 {
        return err("The password should not have more than 30 characters.");
    }

    if password != password2 {
        return err("The confirm password is incorrect.");
    }

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
