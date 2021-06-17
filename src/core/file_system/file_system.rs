use crate::core::repository::TemplateFormatter;
use super::{DirPath, FileContent, paths::TEMPLATES_PATH};
use fs_tree::FsTreeBuilder;
use std::{fs, path::{Path, PathBuf}};


pub struct ProtternFileSystem {}

impl ProtternFileSystem {
    pub fn get_template_path(template_name: &String) -> PathBuf {
        let template_filename = format!("{}.json", template_name);
        Path::new(TEMPLATES_PATH).join(template_filename)
    }

    pub fn extract_template_from<'a>(directory: String) -> Result<(String, String), String> {
        let paths = ProtternFileSystem::dismount_dir(&directory)?;
        let mut formatted_paths: Vec<DirPath> = vec![];
        let mut content: Vec<FileContent> = vec![];

        for path in paths.into_iter() {
            let is_file = &path.path_type == &"file";
            let fp = TemplateFormatter::format_path(&directory, path.clone());
            if is_file {
                let file_content = fs::read_to_string(path.name.to_string()).unwrap();

                if !file_content.is_empty() {
                    content.push(FileContent::new(fp.clone().name, file_content));
                }
            }

            formatted_paths.push(fp);
        }

        let paths_bundle = TemplateFormatter::bundle_paths(formatted_paths);
        let content_bundle = TemplateFormatter::bundle_content(content);
        Ok((paths_bundle, content_bundle))
    }

    pub fn dismount_dir<'a>(directory: &String) -> Result<Vec<DirPath>, String> {
        if directory.contains(r"\") || directory.ends_with("/") {
            return Err("Invalid directory path".to_string());
        }
        let fs_tree = FsTreeBuilder::new(directory).build();
        let paths: Vec<DirPath> = fs_tree
            .into_iter()
            .map(|path| {
                let path = path.unwrap();
                if path.is_file() {
                    let path_name = path.into_os_string().into_string().unwrap();
                    return DirPath::new(path_name, "file");
                }
                if path.is_dir() {
                    let path_name = path.into_os_string().into_string().unwrap();
                    return DirPath::new(path_name, "dir");
                }
                panic!("Path Error.");
            })
            .collect();

        Ok(paths)
    }
}
