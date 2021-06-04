use std::{fs, path::Path};

use crate::utils::paths::TEMPLATES_PATH;

pub fn delete_template(template_name: &String) -> Result<(), String> {
    let template_dir_path = Path::new(TEMPLATES_PATH).join(template_name);

    if !template_dir_path.exists() {
        return Err(format!("Not is possible find \"{}\" on repository", template_name))
    }

    if let Err(e) = fs::remove_dir_all(template_dir_path) {
        return Err(e.to_string())
    }

    Ok(())
}