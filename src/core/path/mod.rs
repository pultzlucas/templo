extern crate fs_tree;
extern crate regex;
mod dir_path;
mod file_content;

use crate::utils::paths::TEMPLATES_PATH;
pub use dir_path::DirPath;
pub use file_content::FileContent;
use fs_tree::FsTreeBuilder;
use std::{
    fs,
    path::{Path, PathBuf},
};

use super::repository::TemplateFormatter;

pub struct ProtternFileSystem {}

impl ProtternFileSystem {
    pub fn get_dir_address(template_name: &String) -> PathBuf {
        Path::new(TEMPLATES_PATH).join(template_name)
    }

    pub fn extract_template_from<'a>(
        directory: String,
    ) -> Result<(String, String), String> {
        let paths = ProtternFileSystem::dismount_dir(&directory)?;
        let files = paths.iter().filter(|path| path.path_type == "file");
        let content: Vec<FileContent> = files
            .map(|path| {
                FileContent::new(path.name.clone(), fs::read_to_string(&path.name).unwrap())
            })
            .collect();
            
        let formated_paths = TemplateFormatter::bundle_paths(directory, paths);
        let formated_content = TemplateFormatter::bundle_content(content);
        Ok((formated_paths, formated_content))
    }

    pub fn dismount_dir<'a>(directory: &String) -> Result<Vec<DirPath<'a>>, String> {
        if directory.contains(r"\") || directory.ends_with("/") {
            return Err("Invalid directory path".to_string());
        }
        let fs_tree = FsTreeBuilder::new(directory).build();
        let paths: Vec<DirPath> = fs_tree
            .into_iter()
            .map(|path| {
                let path = path.unwrap();
                if path.is_file() {
                    return DirPath::new(path.into_os_string().into_string().unwrap(), "file");
                }
                if path.is_dir() {
                    return DirPath::new(path.into_os_string().into_string().unwrap(), "dir");
                }
                panic!("Path Error.");
            })
            .collect();

        
        Ok(paths)
    }
}
