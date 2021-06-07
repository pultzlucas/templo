use super::verify::template_exists;
use crate::utils::paths::TEMPLATES_PATH;
use std::{
    fs,
    io::{Error, ErrorKind},
    path::Path,
};

pub fn delete_template(template_name: &String) -> Result<(), Error> {
    if !template_exists(template_name) {
        let err = Error::new(
            ErrorKind::NotFound,
            format!("Not is possible find \"{}\" on repository", template_name),
        );
        return Err(err);
    }

    let template_path = Path::new(TEMPLATES_PATH).join(template_name);

    if let Err(e) = fs::remove_dir_all(template_path) {
        return Err(e);
    }

    Ok(())
}
