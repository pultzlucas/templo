use super::serde::*;
use crate::core::template::TemplateType;
use crate::core::template::{TempMetadata, Template};
use std::path::Path;

#[test]
fn it_should_return_contents_serialized() {
    let paths = vec![
        Path::new("./src/core/tests/tree_files_only").to_path_buf(),
        Path::new("./src/core/tests/tree_files_only/.file4").to_path_buf(),
        Path::new("./src/core/tests/tree_files_only/file-2").to_path_buf(),
        Path::new("./src/core/tests/tree_files_only/file1").to_path_buf(),
        Path::new("./src/core/tests/tree_files_only/file_3").to_path_buf(),
        Path::new("./src/core/tests/tree_files_only/file_text.txt").to_path_buf(),
    ];
    let paths_ser = serialize_paths(paths);
    assert_eq!(paths_ser, "dir|./src/core/tests/tree_files_only;file|./src/core/tests/tree_files_only/.file4;file|./src/core/tests/tree_files_only/file-2;file|./src/core/tests/tree_files_only/file1;file|./src/core/tests/tree_files_only/file_3;file|./src/core/tests/tree_files_only/file_text.txt".to_string());
}

#[test]
fn it_should_return_paths_serialized() {
    let contents = vec![crate::core::template::miner::File {
        filename: "./src/core/tests/tree_files_only/file_text.txt".to_string(),
        content: "Lorem ipsum dolor\r\n\r\n123123123123\r\n\r\n{(-@#$%¨¨&*)}".to_string(),
    }];

    let contents_ser = serialize_contents(contents);
    assert_eq!(contents_ser, "Li9zcmMvY29yZS90ZXN0cy90cmVlX2ZpbGVzX29ubHkvZmlsZV90ZXh0LnR4dHxMb3JlbSBpcHN1bSBkb2xvcg0KDQoxMjMxMjMxMjMxMjMNCg0KeygtQCMkJcKowqgmKil9".to_string())
}

#[test]
fn it_should_return_contents_deserialized() {
    let paths_des = deserialize_paths("dir|./src/core/tests/tree_files_only;file|./src/core/tests/tree_files_only/.file4;file|./src/core/tests/tree_files_only/file-2;file|./src/core/tests/tree_files_only/file1;file|./src/core/tests/tree_files_only/file_3;file|./src/core/tests/tree_files_only/file_text.txt".to_string());
    assert_eq!(
        paths_des,
        vec![
            Path::new("./src/core/tests/tree_files_only").to_path_buf(),
            Path::new("./src/core/tests/tree_files_only/.file4").to_path_buf(),
            Path::new("./src/core/tests/tree_files_only/file-2").to_path_buf(),
            Path::new("./src/core/tests/tree_files_only/file1").to_path_buf(),
            Path::new("./src/core/tests/tree_files_only/file_3").to_path_buf(),
            Path::new("./src/core/tests/tree_files_only/file_text.txt").to_path_buf(),
        ]
    )
}

#[test]
fn it_should_return_paths_deserialized() {
    let contents_des = deserialize_contents("Li9zcmMvY29yZS90ZXN0cy90cmVlX2ZpbGVzX29ubHkvZmlsZV90ZXh0LnR4dHxMb3JlbSBpcHN1bSBkb2xvcg0KDQoxMjMxMjMxMjMxMjMNCg0KeygtQCMkJcKowqgmKil9".to_string());
    assert_eq!(
        contents_des,
        vec![crate::core::template::miner::File {
            filename: "./src/core/tests/tree_files_only/file_text.txt".to_string(),
            content: "Lorem ipsum dolor\r\n\r\n123123123123\r\n\r\n{(-@#$%¨¨&*)}".to_string(),
        }]
    )
}

#[test]
fn it_should_serialize_template() {
    let template = Template {
        metadata: TempMetadata {
            name: "temp-name".to_string(),
            owner: "Username".to_string(),
            created_at: "123123123123".to_string(),
            template_type: TemplateType::Local,
        },
        paths: vec![
            Path::new("./src/core/tests/tree_files_only").to_path_buf(),
            Path::new("./src/core/tests/tree_files_only/.file4").to_path_buf(),
            Path::new("./src/core/tests/tree_files_only/file-2").to_path_buf(),
            Path::new("./src/core/tests/tree_files_only/file1").to_path_buf(),
            Path::new("./src/core/tests/tree_files_only/file_3").to_path_buf(),
            Path::new("./src/core/tests/tree_files_only/file_text.txt").to_path_buf(),
        ],
        contents: vec![crate::core::template::miner::File {
            filename: "./src/core/tests/tree_files_only/file_text.txt".to_string(),
            content: "Lorem ipsum dolor\r\n\r\n123123123123\r\n\r\n{(-@#$%¨¨&*)}".to_string(),
        }],
    };

    let temp_as_string = serialize_template(template).unwrap();

    assert_eq!(
        temp_as_string,
        r#"{"metadata":"eyJuYW1lIjoidGVtcC1uYW1lIiwib3duZXIiOiJVc2VybmFtZSIsImNyZWF0ZWRfYXQiOiIxMjMxMjMxMjMxMjMiLCJ0ZW1wbGF0ZV90eXBlIjoiTG9jYWwifQ==","name":"temp-name","paths":"dir|./src/core/tests/tree_files_only;file|./src/core/tests/tree_files_only/.file4;file|./src/core/tests/tree_files_only/file-2;file|./src/core/tests/tree_files_only/file1;file|./src/core/tests/tree_files_only/file_3;file|./src/core/tests/tree_files_only/file_text.txt","contents":"Li9zcmMvY29yZS90ZXN0cy90cmVlX2ZpbGVzX29ubHkvZmlsZV90ZXh0LnR4dHxMb3JlbSBpcHN1bSBkb2xvcg0KDQoxMjMxMjMxMjMxMjMNCg0KeygtQCMkJcKowqgmKil9"}"#
    )
}

#[test]
fn it_should_deserialize_template() {
    let temp_as_str = r#"{"metadata":"eyJuYW1lIjoidGVtcC1uYW1lIiwib3duZXIiOiJVc2VybmFtZSIsImNyZWF0ZWRfYXQiOiIxMjMxMjMxMjMxMjMiLCJ0ZW1wbGF0ZV90eXBlIjoiTG9jYWwifQ==","name":"temp-name","paths":"dir|./src/core/tests/tree_files_only;file|./src/core/tests/tree_files_only/.file4;file|./src/core/tests/tree_files_only/file-2;file|./src/core/tests/tree_files_only/file1;file|./src/core/tests/tree_files_only/file_3;file|./src/core/tests/tree_files_only/file_text.txt","contents":"Li9zcmMvY29yZS90ZXN0cy90cmVlX2ZpbGVzX29ubHkvZmlsZV90ZXh0LnR4dHxMb3JlbSBpcHN1bSBkb2xvcg0KDQoxMjMxMjMxMjMxMjMNCg0KeygtQCMkJcKowqgmKil9"}"#;
    let template = deserialize_template(temp_as_str).unwrap();

    assert_eq!(
        template,
        Template {
            metadata: TempMetadata {
                name: "temp-name".to_string(),
                owner: "Username".to_string(),
                created_at: "123123123123".to_string(),
                template_type: TemplateType::Local,
            },
            paths: vec![
                Path::new("./src/core/tests/tree_files_only").to_path_buf(),
                Path::new("./src/core/tests/tree_files_only/.file4").to_path_buf(),
                Path::new("./src/core/tests/tree_files_only/file-2").to_path_buf(),
                Path::new("./src/core/tests/tree_files_only/file1").to_path_buf(),
                Path::new("./src/core/tests/tree_files_only/file_3").to_path_buf(),
                Path::new("./src/core/tests/tree_files_only/file_text.txt").to_path_buf(),
            ],
            contents: vec![crate::core::template::miner::File {
                filename: "./src/core/tests/tree_files_only/file_text.txt".to_string(),
                content: "Lorem ipsum dolor\r\n\r\n123123123123\r\n\r\n{(-@#$%¨¨&*)}".to_string(),
            }],
        }
    )
}
