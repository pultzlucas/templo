use crate::core::repository::TemplateManager;
use std::{fs, path::Path, io::{Error, ErrorKind}};

pub fn create(args: &[String]) -> Result<&str, Error> {
    if args.len() < 1 {
        let err = Error::new(ErrorKind::InvalidInput, "Template name must be specified.");
        return Err(err);
    }

    if args.len() < 2 {
        let err = Error::new(ErrorKind::InvalidInput, "Directory path must be specified.");
        return Err(err);
    }
    
    let template_name = &args[0];
    let directory = Path::new(&args[1]);
    
    if directory.extension() != None {
        let err = Error::new(ErrorKind::InvalidInput, "The path should be a directory.");
        return Err(err);
    }
    
    if !directory.exists() {
        fs::create_dir_all(directory).unwrap();
    }
    
    let template = match TemplateManager::get_template(template_name) {
        Ok(t) => t,
        Err(e) => {
            let err = Error::new(ErrorKind::NotFound, e);
            return Err(err);
        }
    };

    let paths_splitted: Vec<&str> = template.paths.split(";").collect();
    let paths_with_type: Vec<(&str, &Path)> = paths_splitted
        .iter()
        .map(|path| {
            let path_splitted: Vec<&str> = path.split("|").collect();
            (path_splitted[0], Path::new(path_splitted[1]))
        })
        .collect();

    for (path_type, path_name) in paths_with_type.iter() {

        let real_path = Path::new(directory).join(path_name);
    
        if *path_type == "file" {
            fs::write(&real_path, "").unwrap();
        }
    
        if *path_type == "dir" {
            fs::create_dir(&real_path).unwrap();
        }
    }

    Ok("Project was created.")
}
