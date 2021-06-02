extern crate serde;
extern crate serde_json;

use crate::core::{get_template_dir_path, get_template_paths};
use serde_derive::Serialize;
use std::{fs, io, path::Path};

#[derive(Debug, Serialize)]
struct HEAD {
    name: String,
    paths: String,
}

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

fn save_head(head: String, template_name: String) -> Result<(), io::Error> {
    let template_path = get_template_dir_path(&template_name);
    match fs::create_dir(&template_path) {
        Ok(o) => o,
        Err(e) => return Err(e),
    }

    let template_path = template_path
        .join("HEAD.json")
        .into_os_string()
        .into_string()
        .unwrap();
    fs::write(template_path, head)
}
