use super::user_account_data::UserAccountData;
use crate::core::file_system::paths::USER_ACCOUNT_PATH;
use serde_derive::{Deserialize, Serialize};
use std::{fs, io::Error};

#[derive(Serialize, Deserialize, Debug)]
struct UserAccountKey<'a> {
    username: &'a str,
    email: &'a str,
    password: &'a str,
    key: String,
}

impl<'a> UserAccountKey<'a> {
    pub fn new(username: &'a String, email: &'a String, password: &'a String, key: String) -> Self {
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
        let user_account = UserAccountKey::new(&fields.username, &fields.email, &fields.password, key);
        let content = serde_json::to_string(&user_account)?;
        fs::write(USER_ACCOUNT_PATH, content)
    }

    pub fn get_user_account_data() -> Result<UserAccountData, serde_json::Error> {
        let user_account = fs::read_to_string(USER_ACCOUNT_PATH).unwrap();
        serde_json::from_str(&user_account)
    }
}
