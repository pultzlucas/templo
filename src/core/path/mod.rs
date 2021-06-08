extern crate fs_tree;
extern crate regex;
use crate::utils::paths::TEMPLATES_PATH;
use fs_tree::FsTreeBuilder;
use regex::Regex;
use std::{
    path::MAIN_SEPARATOR,
    path::{Path, PathBuf},
};

struct DirPath<'a> {
    name: String,
    path_type: &'a str,
}

pub struct ProtternFileSystem {}

impl ProtternFileSystem {
    pub fn get_dir_path(template_name: &String) -> PathBuf {
        Path::new(TEMPLATES_PATH).join(template_name)
    }

    pub fn dismount_dir(directory: String) -> Result<Vec<String>, String> {
        if directory.contains(r"\") || directory.ends_with("/") {
            return Err("Invalid directory path".to_string());
        }
        let fs_tree = FsTreeBuilder::new(&directory).build();
        let paths: Vec<DirPath> = fs_tree
            .iter()
            .map(|path| {
                let path = path.unwrap();
                if path.is_file() {
                    return DirPath {
                        name: path.into_os_string().into_string().unwrap(),
                        path_type: "file",
                    };
                }
                if path.is_dir() {
                    return DirPath {
                        name: path.into_os_string().into_string().unwrap(),
                        path_type: "dir",
                    };
                }
                panic!("Path Error.");
            })
            .collect();
        let clean_paths = ProtternFileSystem::format_paths_name(directory, paths);
        Ok(clean_paths)
    }

    fn format_paths_name(dir: String, paths: Vec<DirPath>) -> Vec<String> {
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
        paths
    }
}