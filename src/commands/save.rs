use crate::{
    cli::output::messages::error::{
        INVALID_DIRECTORY_PATH_NAME, INVALID_TEMPLATE_NAME, NOT_FOUND_USER_AUTH,
    },
    core::{
        file_system::ProtternFileSystem,
        repository::local,
        template::maker::create_template,
        user_account::UserAccountManager,
        utils::errors::{invalid_input_error, not_found_error},
    },
};
use std::{
    io::{Error, ErrorKind},
    path::Path,
    time::Instant,
};

pub fn save(args: &[String]) -> Result<(), Error> {
    local::create()?;

    if !UserAccountManager::user_auth_exists() {
        return Err(not_found_error(NOT_FOUND_USER_AUTH));
    }
    if args.len() < 1 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            INVALID_DIRECTORY_PATH_NAME,
        ));
    }
    if args.len() < 2 {
        return Err(invalid_input_error(INVALID_TEMPLATE_NAME));
    }

    let directory = args[0].clone();
    let template_name = args[1].clone();
    let template_path = ProtternFileSystem::get_template_path(&template_name);

    if template_path.exists() {
        let err = Error::new(
            ErrorKind::AlreadyExists,
            format!(r#"Template "{}" already exists."#, &template_name),
        );
        return Err(err);
    }
    if !Path::new(&directory).exists() {
        let err = Error::new(
            ErrorKind::InvalidInput,
            format!(r#"Directory "{}" not exists."#, directory),
        );
        return Err(err);
    }

    // Mine template from directory
    let start = Instant::now(); // start timing process
    let template = create_template(template_name, directory)?;

    //save template in repository
    local::save_template(template)?;
    println!("Template was saved successfully.");
    let end = Instant::now(); // stop timing process
    println!("Done in {:.2?}", end.duration_since(start));

    Ok(())
}
