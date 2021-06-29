use crate::{
    core::{
        io::messages::error::{
            INVALID_DIRECTORY_PATH_NAME, INVALID_DIRECTORY_PATH_TYPE, INVALID_TEMPLATE_NAME,
        },
        repository::TemplateManager,
    },
    init,
};
use std::{
    fs,
    io::{Error, ErrorKind},
    path::Path,
};

pub fn create(args: &[String]) -> Result<(), Error> {
    init()?;

    if args.len() < 1 {
        return Err(Error::new(ErrorKind::InvalidInput, INVALID_TEMPLATE_NAME));
    }

    if args.len() < 2 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            INVALID_DIRECTORY_PATH_NAME,
        ));
    }
    let template_name = &args[0];
    let directory = Path::new(&args[1]);
    if directory.extension() != None {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            INVALID_DIRECTORY_PATH_TYPE,
        ));
    }
    if !directory.exists() {
        fs::create_dir_all(directory)?;
    }
    TemplateManager::create_template(template_name, directory)?;

    println!("Project was created.");
    Ok(())
}
