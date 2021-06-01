#[cfg(test)]
use crate::tests;

extern crate fs_tree;
use fs_tree::FsTreeBuilder;

pub fn get_template_paths(directory: String) -> Vec<String> {
    let fs_tree = FsTreeBuilder::new(directory).build();

    let mut files: Vec<String> = fs_tree
        .iter()
        .map(|file| file.unwrap().into_os_string().into_string().unwrap())
        .collect();

    files
}
