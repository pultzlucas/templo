#[cfg(test)]
use crate::tests;

extern crate fs_tree;
extern crate regex;
use fs_tree::FsTreeBuilder;
use regex::Regex;

pub fn get_template_paths(directory: String) -> Result<Vec<String>, String> {
    let fs_tree = FsTreeBuilder::new(&directory).build();

    let files: Vec<String> = fs_tree
        .iter()
        .map(|file| file.unwrap().into_os_string().into_string().unwrap())
        .collect();

    let real_files = clear_files_name(directory, files);

    Ok(real_files)
}

fn clear_files_name(dir: String, files: Vec<String>) -> Vec<String> {
    let regex = Regex::new(dir.as_str()).unwrap();
    let mut clean_files: Vec<String> = vec![];

    for file in files.iter() {
        clean_files.push(regex.replace(file, "").to_string())
    }

    clean_files
}
