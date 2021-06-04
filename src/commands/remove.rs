use crate::utils::paths::TEMPLATES_PATH;
use std::{path::Path, fs};

pub fn remove(args: &[String]) -> Result<&str, String> {
    if args.len() < 1  {
        return Err("Template name must be specified.".to_string())
    }

    let template_name = &args[0];
    let template_dir_path = Path::new(TEMPLATES_PATH).join(template_name);

    if !template_dir_path.exists() {
        return Err(format!("Not is possible find \"{}\" on repository", template_name))
    }

    if let Err(e) = fs::remove_dir_all(template_dir_path) {
        return Err(e.to_string())
    }

    Ok("Template was removed.")
}