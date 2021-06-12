extern crate regex;
use crate::core::file_system::{DirPath, FileContent};
use regex::Regex;
use std::path::MAIN_SEPARATOR;

pub struct TemplateFormatter {}

impl TemplateFormatter {
    pub fn format_path<'a>(dir: &'a String, path: DirPath<'a>) -> DirPath<'a> {
        let mut regex = Regex::new(&dir).unwrap();

        if dir == "." {
            regex = Regex::new(r"\.$").unwrap();
        }

        let path_splitted: Vec<&str> = path.name.split(MAIN_SEPARATOR).collect();
        let mut formatted_path: Vec<&str> = vec![];

        for path in path_splitted.into_iter() {
            if !regex.is_match(path) && path != "." {
                formatted_path.push(path)
            }
        }

        let formatted_path = formatted_path.join(MAIN_SEPARATOR.to_string().as_str());

        DirPath::new(formatted_path, path.path_type)
    }

    pub fn bundle_paths(paths: Vec<DirPath>) -> String {
        let paths: Vec<String> = paths
            .into_iter()
            .map(|path| {
                if path.path_type == "file" {
                    return format!("file|{}", &path.name);
                }
                if path.path_type == "dir" {
                    return format!("dir|{}", &path.name);
                }

                panic!("Error when saving.")
            })
            .filter(|path| path != "dir|")
            .collect();

        paths.join(";").to_string()
    }

    pub fn bundle_content(file_contents: Vec<FileContent>) -> String {
        let content_vec: Vec<String> = file_contents
            .into_iter()
            .map(|fc| format!("{}|{}", fc.file, base64::encode(fc.content)))
            .collect();
        content_vec.join(";").to_string()
    }
}
