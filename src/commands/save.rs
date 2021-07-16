use crate::{
    core::{
        file_system::ProtternFileSystem,
        io::messages::error::{
            INVALID_DIRECTORY_PATH_NAME, INVALID_TEMPLATE_NAME, NOT_FOUND_USER_AUTH,
        },
        repository::RepositoryConnection,
        template::{Template, TemplateMiner},
        user_account::UserAccountManager,
    },
    init,
};
use std::{
    io::{Error, ErrorKind},
    path::Path,
};

pub fn save(args: &[String]) -> Result<(), Error> {
    init()?;

    if !UserAccountManager::user_auth_exists() {
        return Err(Error::new(ErrorKind::NotFound, NOT_FOUND_USER_AUTH));
    }
    if args.len() < 1 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            INVALID_DIRECTORY_PATH_NAME,
        ));
    }
    if args.len() < 2 {
        return Err(Error::new(ErrorKind::InvalidInput, INVALID_TEMPLATE_NAME));
    }

    let directory = args[0].clone();
    let template_name = args[1].clone();
    let template_path = ProtternFileSystem::get_template_path(&template_name);

    if template_path.exists() {
        let err = Error::new(
            ErrorKind::AlreadyExists,
            format!("Template \"{}\" already exists.", &template_name),
        );
        return Err(err);
    }
    if !Path::new(&directory).exists() {
        let err = Error::new(
            ErrorKind::InvalidInput,
            format!("Directory {} not exists.", directory),
        );
        return Err(err);
    }

    let miner = TemplateMiner::new(directory);
    let (template_paths, template_content) = miner.extract_template_content()?;

    let template = Template::new(template_name, template_paths, template_content);
    RepositoryConnection::new().save_template(template)?;

    println!("Template was saved successfully.");

    Ok(())
}
