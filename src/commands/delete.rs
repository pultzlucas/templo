use crate::{
    cli::output::messages::error::{INVALID_TEMPLATE_NAME, NOT_FOUND_USER_AUTH},
    core::{repository::local, user_account::UserAccountManager},
};
use std::{
    io::{Error, ErrorKind},
    time::Instant,
};

pub fn delete(args: &[String]) -> Result<(), Error> {
    local::create()?;

    if !UserAccountManager::user_auth_exists() {
        return Err(Error::new(ErrorKind::NotFound, NOT_FOUND_USER_AUTH));
    }

    if args.len() < 1 {
        return Err(Error::new(ErrorKind::InvalidInput, INVALID_TEMPLATE_NAME));
    }

    let templates_name = &args[0..];

    // Deleting templates
    let start = Instant::now(); // start timing process
    for name in templates_name.iter() {
        local::delete_template(name)?;
        println!("Template \"{}\" was deleted.", name);
    }
    let end = Instant::now(); // stop timing process
    println!("Done in {:.2?}", end.duration_since(start));
    Ok(())
}
