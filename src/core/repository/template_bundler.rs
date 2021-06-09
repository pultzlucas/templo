extern crate regex;
use crate::core::path::{DirPath, FileContent};
use regex::Regex;
use std::path::MAIN_SEPARATOR;

pub struct TemplateFormatter {}

impl TemplateFormatter {
    pub fn bundle_paths(dir: String, paths: Vec<DirPath>) -> String {
        let paths: Vec<String> = paths
            .into_iter()
            .map(|path| {
                let regex = Regex::new(dir.as_str()).unwrap();
                let path_splitted: Vec<&str> = path.name.split(MAIN_SEPARATOR).collect();
                let clean_splitted_path: Vec<&str> = path_splitted
                    .iter()
                    .filter(|path| !regex.is_match(path) && **path != ".")
                    .map(|path| *path)
                    .collect();
                let clean_path = clean_splitted_path.join(&String::from(MAIN_SEPARATOR));
                if path.path_type == "file" {
                    return format!("file|{}", clean_path);
                }
                if path.path_type == "dir" {
                    return format!("dir|{}", clean_path);
                }
                panic!("Error when saving.")
            })
            .filter(|path| !(path == "dir|") && !(path == "file|"))
            .collect();

        paths.join(";").to_string()
    }

    pub fn bundle_content(file_contents: Vec<FileContent>) -> String {
        let content_vec: Vec<String> = file_contents
            .into_iter()
            .map(|fc| format!("{}|{}", fc.file, fc.content))
            .collect();
        
        content_vec.join(";").to_string()
    }
}
