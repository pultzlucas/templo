use crate::core::get_template_paths;
use crate::utils::structs::HEAD;
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

    let template_head = HEAD {
        name: template_name,
        paths: template_paths.unwrap().join(";")
    };

    println!("HEAD: \n{:?}", template_head);

    Ok("saved!")
}
