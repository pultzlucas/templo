use super::TemplateBundler;
use crate::core::errors::invalid_input_error;
use crate::core::file_system::{DirPath, FileContent};
use fs_tree::FsTreeBuilder;
use serde_derive::{Deserialize, Serialize};
use std::{
    fs,
    io::{Error, ErrorKind},
    path::{Path, PathBuf},
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct File {
    pub filename: String,
    pub content: String,
}

pub fn extract_paths_from(directory: &str) -> Result<Vec<PathBuf>, Error> {
    valid_directory_path(directory)?;
    let fs_tree = FsTreeBuilder::new(directory).build();
    let vec_fs_tree: Vec<PathBuf> = fs_tree
        .into_iter()
        .map(|path| format_pathbuf(path.unwrap()))
        .collect();
    Ok(vec_fs_tree)
}

pub fn extract_files_from_paths(paths: Vec<PathBuf>) -> Vec<File> {
    paths
        .into_iter()
        .filter(|path| path.is_file())
        .map(|file| File {
            filename: pathbuf_to_string(file.clone()),
            content: fs::read_to_string(file).unwrap(),
        })
        .collect()
}

fn format_pathbuf(path: PathBuf) -> PathBuf {
    Path::new(&pathbuf_to_string(path).replace(r"\", "/")).to_path_buf()
}

fn pathbuf_to_string(path: PathBuf) -> String {
    path.as_os_str().to_str().unwrap().to_string()
}

fn valid_directory_path(directory: &str) -> Result<(), Error> {
    if directory.contains(r"\") || directory.ends_with("/") {
        return Err(invalid_input_error("Invalid directory path."));
    }
    Ok(())
}

pub struct TemplateMiner {
    directory: String,
}

impl TemplateMiner {
    pub fn new(directory: String) -> Self {
        Self { directory }
    }

    pub fn extract_template_content<'a>(&self) -> Result<(String, String), Error> {
        let paths = self.dismount_dir()?;
        let mut formatted_paths: Vec<DirPath> = vec![];
        let mut content: Vec<FileContent> = vec![];

        for path in paths.into_iter() {
            let is_file = &path.path_type == &"file";
            let fp = TemplateBundler::format_path(&self.directory, path.clone());
            if is_file {
                let file_content = fs::read_to_string(path.name.to_string()).unwrap();

                if !file_content.is_empty() {
                    content.push(FileContent {
                        filename: fp.clone().name,
                        content: file_content,
                    });
                }
            }

            formatted_paths.push(fp);
        }

        let paths_bundle = TemplateBundler::bundle_paths(formatted_paths);
        let content_bundle = TemplateBundler::bundle_content(content);
        Ok((paths_bundle, content_bundle))
    }

    fn dismount_dir<'a>(&self) -> Result<Vec<DirPath>, Error> {
        if self.directory.contains(r"\") || self.directory.ends_with("/") {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Invalid directory path.",
            ));
        }
        let paths: Vec<DirPath> = {
            let fs_tree = FsTreeBuilder::new(&self.directory).build();
            fs_tree
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
                .collect()
        };

        Ok(paths)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    const TREE_FILES_ONLY: &'static str = "./src/core/tests/tree_files_only";

    #[test]
    fn it_should_convert_pathbuf_to_string() {
        assert_eq!(
            pathbuf_to_string(Path::new(TREE_FILES_ONLY).to_path_buf()),
            TREE_FILES_ONLY
        )
    }

    #[test]
    fn it_should_return_the_tree_files_only_flatted() {
        let flat = extract_paths_from(TREE_FILES_ONLY).unwrap();
        assert_eq!(
            flat,
            vec![
                Path::new("./src/core/tests/tree_files_only"),
                Path::new("./src/core/tests/tree_files_only/.file4"),
                Path::new("./src/core/tests/tree_files_only/file-2"),
                Path::new("./src/core/tests/tree_files_only/file1"),
                Path::new("./src/core/tests/tree_files_only/file_3"),
                Path::new("./src/core/tests/tree_files_only/file_text.txt")
            ]
        );
    }

    #[test]
    fn it_should_return_all_tree_files_only_files() {
        let paths = extract_paths_from(TREE_FILES_ONLY).unwrap();
        let contents: Vec<File> = extract_files_from_paths(paths)
            .into_iter()
            .filter(|file_content| file_content.content != "")
            .collect();

        assert_eq!(
            contents,
            vec![File {
                filename: "./src/core/tests/tree_files_only/file_text.txt".to_string(),
                content: "Lorem ipsum dolor\r\n\r\n123123123123\r\n\r\n{(-@#$%¨¨&*)}".to_string()
            }]
        )
    }
}
