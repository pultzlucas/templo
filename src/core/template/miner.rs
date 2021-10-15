use super::{TempContent, TempPath, TempPathType};
use crate::core::{
    fs as core_fs,
    utils::errors::std_error,
    utils::path::{
        format_path_namespace, pathbuf_to_string, remove_dir_prefix, valid_directory_path,
    },
};
use base64;
use fs_tree::FsTreeBuilder;
use serde_json::from_str;
use std::{fs, io::Error, path::Path};

pub fn mine_paths_from(directory_path: &str) -> Result<Vec<TempPath>, Error> {
    valid_directory_path(directory_path)?;

    let paths_to_ignore = get_paths_to_ignore(directory_path)?;

    let fs_tree = (FsTreeBuilder::new(directory_path))
        .ignore_paths(&paths_to_ignore[..])
        .build();

    let vec_fs_tree: Vec<TempPath> = fs_tree
        .into_iter()
        .map(|path| TempPath::new(path.expect("Not is possible find the folder.")))
        .collect();

    Ok(vec_fs_tree)
}

pub fn mine_files_from_paths(
    paths: Vec<TempPath>,
    directory: &str,
) -> Result<Vec<TempContent>, Error> {
    let files = paths
        .into_iter()
        .filter(|path| path.path_type == TempPathType::File);

    files
        .map(|file| {
            let file_path_str = &pathbuf_to_string(file.path.clone());
            let bytes = core_fs::read_bytes(file_path_str)?;
            let is_text = core_fs::file_content_is_text(file_path_str);

            let file_path_clean =
                remove_dir_prefix(file.path, directory).expect("Error when removing dir prefix.");
            let filename = pathbuf_to_string(format_path_namespace(file_path_clean));
            Ok(TempContent::new(filename, base64::encode(bytes), is_text))
        })
        .collect()
}

fn get_paths_to_ignore(directory_path: &str) -> Result<Vec<String>, Error> {
    let template_config = Path::new(directory_path).join("TemplateConfig");

    if !template_config.exists() {
        return Ok(vec!["".to_string()]);
    }

    let ignore_filename = template_config.join("ignore.json");

    let mut paths_to_ignore = if ignore_filename.exists() {
        std_error(from_str(&fs::read_to_string(ignore_filename)?))?
    } else {
        vec![]
    };
    paths_to_ignore.push("./TemplateConfig/".to_string());

    let real_paths_to_ignore: Vec<String> = paths_to_ignore
        .into_iter()
        .map(|path| Path::new(directory_path).join(path))
        .map(pathbuf_to_string)
        .collect();

    Ok(real_paths_to_ignore)
}
