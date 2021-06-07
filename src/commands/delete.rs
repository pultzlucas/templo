use crate::core::repository::delete_template;
use std::io::{Error, ErrorKind};

pub fn delete(args: &[String]) -> Result<&str, Error> {
    if args.len() < 1  {
        let err = Error::new(ErrorKind::InvalidInput, "Template name must be specified.");
        return Err(err);
    }

    if let Err(e) = delete_template(&args[0]) {
        return Err(e)
    } 

    Ok("Template was deleted.")
}