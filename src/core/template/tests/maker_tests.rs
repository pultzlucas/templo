use crate::{
    core::template::{maker::make_template, TempPath},
    core::utils::date::get_date_now_string,
};

use super::{config::ConfigArg, Template};
use crate::core::utils::path::str_to_pathbuf;

const FOLDER: &'static str = "./folder-for-tests";

fn get_correct_template_struct() -> Template {
    Template {
        name: "test".to_string(),
        description: None,
        author: None,
        created_at: get_date_now_string(),
        updated_at: None,
        paths: vec![
            TempPath {
                path: str_to_pathbuf("([ filename  ]).rs"),
                is_file: true,
                content: None,
            },
            TempPath {
                path: str_to_pathbuf("rust-file.rs"),
                is_file: true,
                content: None,
            },
            TempPath {
                path: str_to_pathbuf("tree"),
                is_file: true,
                content: None,
            },
            TempPath {
                path: str_to_pathbuf("tree/([folder1])"),
                is_file: true,
                content: None,
            },
            TempPath {
                path: str_to_pathbuf("tree/([folder1])/file3"),
                is_file: true,
                content: None,
            },
            TempPath {
                path: str_to_pathbuf("tree/([folder1])/folder2"),
                is_file: true,
                content: None,
            },
            TempPath {
                path: str_to_pathbuf("tree/([folder1])/folder2/file4"),
                is_file: true,
                content: None,
            },
            TempPath {
                path: str_to_pathbuf("tree/file1"),
                is_file: true,
                content: None,
            },
            TempPath {
                path: str_to_pathbuf("tree/file2"),
                is_file: true,
                content: None,
            },
        ],
        // contents: vec![
        //     TempContent {
        //         file_path: "([ filename  ]).rs".to_string(),
        //         content: "Zm4gez5maWxlbmFtZTx9KCkNCnsNCiAgICBwcmludCEoIns+bmFtZTx9IHs+ICBsYXN0TmFtZSAgPH0iKTsNCn0=".to_string(),
        //         is_text: true
        //     },
        //     TempContent {
        //         file_path: "rust-file.rs".to_string(),
        //         content: "Zm4gbWFpbigpIHsNCiAgICBwcmludCEoImhlbGxvIikNCn0=".to_string(),
        //         is_text: true
        //     }
        // ],
        args: Some(vec![
            ConfigArg {
                key: "name".to_string(),
                query: "Your name (pultzlucas): ".to_string(),
                about: None,
                default: Some("pultzlucas".to_string()),
            },
            ConfigArg {
                key: "lastName".to_string(),
                query: "Your last name: ".to_string(),
                about: None,
                default: None,
            },
            ConfigArg {
                key: "filename".to_string(),
                query: "Javascript filename (script): ".to_string(),
                about: Some("Sets javacript filename".to_string()),
                default: Some("script".to_string()),
            },
            ConfigArg {
                key: "folder1".to_string(),
                query: "Folder name: ".to_string(),
                about: Some("Sets folder1 name".to_string()),
                default: Some("folder1".to_string()),
            },
        ]),
    }
}

#[test]
#[ignore]
fn test_template_creation() {
    let template = make_template("test".to_string(), FOLDER, None).unwrap();

    assert_eq!(template, get_correct_template_struct());
}
