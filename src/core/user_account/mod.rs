extern crate serde_json;
use crate::utils::{paths::USER_ACCOUNT_PATH};
use std::{fs, io::Error};
mod user_account_data;
pub use user_account_data::UserAccountData;

pub struct UserAccountManager {}

impl UserAccountManager {
    pub fn create_user_account(fields: &UserAccountData) -> Result<(), Error> {
        let content = serde_json::to_string(fields)?;
        fs::write(USER_ACCOUNT_PATH, content)
    }
}
