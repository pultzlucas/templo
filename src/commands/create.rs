use crate::core::repository::get_template;
use std::{fs, path::Path};

pub fn create(args: &[String]) -> Result<&str, String> {
    if args.len() < 1 {
        return Err("Template name must be specified.".to_string());
    }

    if args.len() < 2 {
        return Err("Directory path must be specified.".to_string());
    }

    let template_name = &args[0];
    let directory = Path::new(&args[1]);

    if let Err(e) = validate_input_path(Path::new(&directory)) {
        return Err(e);
    }

    let template = match get_template(template_name) {
        Ok(t) => t,
        Err(e) => return Err(e),
    };

    let paths_splitted: Vec<&str> = template.paths.split(";").collect();
    let paths_with_type: Vec<(&str, &Path)> = paths_splitted
        .iter()
        .map(|path| {
            let path_splitted: Vec<&str> = path.split("|").collect();
            (path_splitted[0], Path::new(path_splitted[1]))
        })
        .collect();

    paths_with_type.iter().for_each(|(path_type, path_name)| {
        let real_path = Path::new(directory).join(path_name);

        if *path_type == "file" {
            fs::write(&real_path, "").unwrap();
        }

        if *path_type == "dir" {
            fs::create_dir(&real_path).unwrap();
        }
    });

    Ok("Project was created.")
}

fn validate_input_path(path: &Path) -> Result<(), String> {
    if !path.exists() {
        return Err("The directory not exists.".to_string());
    }

    if !path.is_dir() {
        return Err("The path should be a directory.".to_string());
    }

    Ok(())
}
