use super::*;
use std::path::Path;
//use repository;
//use template;
//use file_system;
//use requester;
//use user_account;

const TREE_FILES_ONLY: &'static str = "./src/core/tests/tree_files_only";

// MINER

#[test]
fn it_should_return_the_tree_files_only_flatted() {
    let flat = template::miner::extract_paths_from(TREE_FILES_ONLY).unwrap();
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
    let paths = template::miner::extract_paths_from(TREE_FILES_ONLY).unwrap();
    let contents: Vec<template::miner::File> = template::miner::extract_files_from_paths(paths)
        .into_iter()
        .filter(|file_content| file_content.content != "")
        .collect();

    assert_eq!(
        contents,
        vec![template::miner::File {
            path: Path::new("./src/core/tests/tree_files_only/file_text.txt").to_path_buf(),
            filename: "./src/core/tests/tree_files_only/file_text.txt".to_string(),
            content: "Lorem ipsum dolor\r\n\r\n123123123123\r\n\r\n{(-@#$%¨¨&*)}".to_string()
        }]
    )
}

#[test]
fn it_should_return_a_valid_template() {
    let template =
        template::maker::create_template("Juliano".to_string(), TREE_FILES_ONLY.to_string())
            .unwrap();

    assert_eq!(
        template,
        template::maker::Template {
            name: "Juliano".to_string(),
            paths: vec![
                Path::new("./src/core/tests/tree_files_only").to_path_buf(),
                Path::new("./src/core/tests/tree_files_only/.file4").to_path_buf(),
                Path::new("./src/core/tests/tree_files_only/file-2").to_path_buf(),
                Path::new("./src/core/tests/tree_files_only/file1").to_path_buf(),
                Path::new("./src/core/tests/tree_files_only/file_3").to_path_buf(),
                Path::new("./src/core/tests/tree_files_only/file_text.txt").to_path_buf()
            ],
            contents: vec![template::miner::File {
                path: Path::new("./src/core/tests/tree_files_only/file_text.txt").to_path_buf(),
                filename: "./src/core/tests/tree_files_only/file_text.txt".to_string(),
                content: "Lorem ipsum dolor\r\n\r\n123123123123\r\n\r\n{(-@#$%¨¨&*)}".to_string()
            }]
        }
    )
}
