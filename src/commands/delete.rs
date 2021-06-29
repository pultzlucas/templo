use crate::{
    core::{
        io::messages::error::{INVALID_TEMPLATE_NAME, NOT_FOUND_USER_AUTH},
        repository::TemplateManager,
        user_account::UserAccountManager,
    },
    init,
};
use std::io::{Error, ErrorKind};

pub fn delete(args: &[String]) -> Result<(), Error> {
    init()?;

    if !UserAccountManager::user_auth_exists() {
        return Err(Error::new(ErrorKind::NotFound, NOT_FOUND_USER_AUTH));
    }

    if args.len() < 1 {
        return Err(Error::new(ErrorKind::InvalidInput, INVALID_TEMPLATE_NAME));
    }

    TemplateManager::delete_template(&args[0])?;

    println!("Template was deleted.");

    Ok(())
}
