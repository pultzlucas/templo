use crate::{core::repository::TemplateManager, init};
use std::{
    fs,
    io::{Error, ErrorKind},
    path::Path,
};

pub fn create(args: &[String]) -> Result<(), Error> {
    init()?;

    if args.len() < 1 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Template name must be specified.",
        ));
    }

    if args.len() < 2 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Directory path must be specified.",
        ));
    }
    let template_name = &args[0];
    let directory = Path::new(&args[1]);
    if directory.extension() != None {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "The path should be a directory.",
        ));
    }
    if !directory.exists() {
        fs::create_dir_all(directory)?;
    }
   
    TemplateManager::create_template(template_name, directory)?;

    println!("Project was created.");
    Ok(())
}
