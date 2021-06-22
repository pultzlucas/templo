use crate::{
    core::{
        file_system::ProtternFileSystem,
        repository::{Template, TemplateManager},
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
        let err = Error::new(
            ErrorKind::NotFound,
            r#"This process cannot be runned because You dont has an authenticated user account.
Please type "prottern register" to register one.
If you already have a user account created, type "prottern login" to authenticate it."#,
        );
        return Err(err);
    }

    if args.len() < 1 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Folder name must be specified.",
        ));
    }
    if args.len() < 2 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Template name must be specified.",
        ));
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

    let (template_paths, template_content) =
        match ProtternFileSystem::extract_template_from(directory) {
            Ok(o) => o,
            Err(e) => {
                let err = Error::new(ErrorKind::Other, e);
                return Err(err);
            }
        };

    let template = Template::new(template_name, template_paths, template_content);
    TemplateManager::save_template(template)?;

    println!("Template was saved successfully.");

    Ok(())
}
