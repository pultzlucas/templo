use crate::{
    core::{
        repository::TemplateManager,
        user_account::UserAccountManager,
    },
    init
};
use std::io::{Error, ErrorKind};

pub fn delete(args: &[String]) -> Result<(), Error> {
    init()?;

    if !UserAccountManager::user_auth_exists() {
        let err = Error::new(
            ErrorKind::NotFound, 
            r#"This process cannot be runned because You dont has an authenticated user account.
Please type "prottern register" to register one.
If you already have a user account created, type "prottern login" to authenticate it."#
        );
        return Err(err);
    }

    if args.len() < 1  {
        let err = Error::new(ErrorKind::InvalidInput, "Template name must be specified.");
        return Err(err);
    }

    if let Err(e) = TemplateManager::delete_template(&args[0]) {
        return Err(e)
    } 

    println!("Template was deleted.");

    Ok(())
}