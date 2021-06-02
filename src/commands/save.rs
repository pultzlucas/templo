use crate::core::get_template_paths;
extern crate serde;
extern crate serde_json;
use serde_derive::{Serialize};

#[derive(Debug, Serialize)]
struct HEAD {
    name: String,
    paths: String,
}

use std::path::Path;

pub fn save(args: &[String]) -> Result<&str, String> {
    if args.len() < 1 {
        return Err("Folder name must be specified.".to_string())
    }

    if args.len() < 2 {
        return Err("Template name must be specified.".to_string())
    }
    
    let directory = args[0].clone();
    let template_name = args[1].clone();

    if !Path::new(&directory).exists() {
       return Err(format!("Error: Directory {} not exists.", directory))
    }

    let template_paths = get_template_paths(directory);

    if template_paths.is_err() {
        return Err(format!("Error: {}", template_paths.unwrap_err()))
    }

    let head = HEAD {
        name: template_name,
        paths: template_paths.unwrap().join(";")
    };

    let head_string = serde_json::to_string(&head).unwrap();

    println!("{:?}", head_string);

    Ok("saved!")
}
