use crate::utils::{paths::USER_ACCOUNT_PATH, structs::UserAccountData};
use serde_json;
use std::{fs, io::Error};

pub struct UserAccountManager {}

impl UserAccountManager {
    pub fn create_user_account(fields: &UserAccountData) -> Result<(), Error> {
        let content = serde_json::to_string(fields)?;
        fs::write(USER_ACCOUNT_PATH, content)
    }
}
