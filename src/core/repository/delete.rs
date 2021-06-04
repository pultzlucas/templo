use std::{fs, path::Path};
use crate::utils::paths::TEMPLATES_PATH;
use super::verify::template_exists;

pub fn delete_template(template_name: &String) -> Result<(), String> {
    if !template_exists(template_name) {
        return Err(format!("Not is possible find \"{}\" on repository", template_name))
    }

    let template_path = Path::new(TEMPLATES_PATH).join(template_name);

    if let Err(e) = fs::remove_dir_all(template_path) {
        return Err(e.to_string())
    }

    Ok(())
}