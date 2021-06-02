#[cfg(test)]
mod tests;

extern crate fs_tree;
extern crate regex;
use fs_tree::FsTreeBuilder;
use regex::Regex;
use std::path::MAIN_SEPARATOR;

pub fn get_template_paths(directory: String) -> Result<Vec<String>, String> {
    if directory.contains(r"\") || directory.ends_with("/") {
        return Err("Invalid directory path".to_string())
    }

    let fs_tree = FsTreeBuilder::new(&directory).build();

    let files: Vec<String> = fs_tree
        .iter()
        .map(|file| file.unwrap().into_os_string().into_string().unwrap())
        .collect();

    let real_files = clear_files_name(directory, files);

    Ok(real_files)
}

fn clear_files_name(dir: String, files: Vec<String>) -> Vec<String> {
    let files = files
        .iter()
        .map(|file| {
            let regex = Regex::new(dir.as_str()).unwrap();
            let path_splitted: Vec<&str> = file.split(MAIN_SEPARATOR).collect();

            let clean_path: Vec<&str> = path_splitted
                .iter()
                .filter(|path| !regex.is_match(path) && **path != ".")
                .map(|path| *path)
                .collect();

            clean_path.join(&String::from(MAIN_SEPARATOR))
        })
        .filter(|file| file != "")
        .collect();

    files
}
