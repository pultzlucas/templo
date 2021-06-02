use crate::core::path::{get_template_dir_path, get_template_paths, save_head};
use crate::utils::structs::HEAD;
use std::path::Path;

pub fn save(args: &[String]) -> Result<&str, String> {
    if args.len() < 1 {
        return Err("Folder name must be specified.".to_string());
    }

    if args.len() < 2 {
        return Err("Template name must be specified.".to_string());
    }

    let directory = args[0].clone();
    let template_name = args[1].clone();

    if get_template_dir_path(&template_name).exists() {
        return Err(format!("Template \"{}\" already exists.", &template_name));
    }

    if !Path::new(&directory).exists() {
        return Err(format!("Error: Directory {} not exists.", directory));
    }

    let template_paths = match get_template_paths(directory) {
        Ok(o) => o,
        Err(e) => return Err(e.to_string())
    };

    let head = HEAD {
        name: template_name,
        paths: template_paths.join(";"),
    };

    let head_string = serde_json::to_string_pretty(&head).unwrap();

    if let Err(e) = save_head(head_string, head.name) {
        return Err(e.to_string())
    }

    Ok("Template was saved successfully.")
}
