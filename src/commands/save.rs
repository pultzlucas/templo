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

    let template_paths = get_template_paths(directory);

    if template_paths.is_err() {
        return Err(format!("Error: {}", template_paths.unwrap_err()));
    }

    let head = HEAD {
        name: template_name,
        paths: template_paths.unwrap().join(";"),
    };

    let head_string = serde_json::to_string_pretty(&head).unwrap();
    let err = save_head(head_string, head.name);

    if err.is_err() {
        return Err(err.unwrap_err().to_string());
    }

    Ok("Template was saved successfully.")
}
