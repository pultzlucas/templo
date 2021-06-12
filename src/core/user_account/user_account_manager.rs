use crate::core::file_system::paths::USER_ACCOUNT_PATH;
use std::{fs, io::Error};
use super::user_account_data::UserAccountData;

pub struct UserAccountManager {}

impl UserAccountManager {
    pub fn log_user_account(fields: &UserAccountData) -> Result<(), Error> {
        let content = serde_json::to_string(fields)?;
        fs::write(USER_ACCOUNT_PATH, content)
    }

    pub fn get_user_account_data() -> Result<UserAccountData, serde_json::Error> {
        let user_account = fs::read_to_string(USER_ACCOUNT_PATH).unwrap();
        serde_json::from_str(&user_account)
    }
}
