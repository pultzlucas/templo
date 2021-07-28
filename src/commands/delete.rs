use crate::core::{
    io::messages::error::{INVALID_TEMPLATE_NAME, NOT_FOUND_USER_AUTH},
    repository::{RepositoryConnection, create_repository_if_not_exists},
    user_account::UserAccountManager,
};

use std::io::{Error, ErrorKind};

pub fn delete(args: &[String]) -> Result<(), Error> {
    create_repository_if_not_exists()?;

    if !UserAccountManager::user_auth_exists() {
        return Err(Error::new(ErrorKind::NotFound, NOT_FOUND_USER_AUTH));
    }

    if args.len() < 1 {
        return Err(Error::new(ErrorKind::InvalidInput, INVALID_TEMPLATE_NAME));
    }

    let templates_name = &args[0..];
    for name in templates_name.iter() {
        RepositoryConnection::new().delete_template(name)?;
        println!("Template \"{}\" was deleted.", name);
    }

    Ok(())
}
