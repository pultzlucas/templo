#[cfg(test)]
use crate::tests;

extern crate fs_tree;
use fs_tree::FsTreeBuilder;

pub fn get_template_paths(directory: String) -> Result<Vec<String>, String> {
    let fs_tree = FsTreeBuilder::new(&directory).build();

    let files: Vec<String> = fs_tree
        .iter()
        .map(|file| file.unwrap().into_os_string().into_string().unwrap())
        .collect();

    Ok(files)
}
