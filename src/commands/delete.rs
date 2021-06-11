use crate::core::repository::TemplateManager;
use std::io::{Error, ErrorKind};

pub fn delete(args: &[String]) -> Result<(), Error> {
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