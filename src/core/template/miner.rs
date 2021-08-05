use super::TemplateBundler;
use crate::core::file_system::{DirPath, FileContent};
use fs_tree::FsTreeBuilder;
use std::{
    fs,
    io::{Error, ErrorKind},
};

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
                    content.push(FileContent::new(fp.clone().name, file_content));
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
