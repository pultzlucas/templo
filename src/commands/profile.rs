use crate::utils::errors::not_found_error;
use crate::{
    cli::output::messages::error::NOT_FOUND_USER_AUTH,
    core::user_account::{get_user_account_data, user_auth_exists},
};
use std::io::Error;

pub fn profile() -> Result<(), Error> {
    if !user_auth_exists() {
        return Err(not_found_error(NOT_FOUND_USER_AUTH));
    }

    let current_user = get_user_account_data()?;
    println!("{}", current_user.username);

    Ok(())
}
