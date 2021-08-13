use super::serde::*;
use crate::core::template::{TempPath, TempPathType, Template, TemplateType};
use std::path::Path;

#[test]
fn it_should_return_contents_serialized() {
    let paths = vec![
        TempPath {
            buf: Path::new(".file4").to_path_buf(),
            path_type: TempPathType::File,
        },
        TempPath {
            buf: Path::new("file-2").to_path_buf(),
            path_type: TempPathType::File,
        },
        TempPath {
            buf: Path::new("file1").to_path_buf(),
            path_type: TempPathType::File,
        },
        TempPath {
            buf: Path::new("file_3").to_path_buf(),
            path_type: TempPathType::File,
        },
        TempPath {
            buf: Path::new("file_text.txt").to_path_buf(),
            path_type: TempPathType::File,
        },
    ];
    let paths_ser = serialize_paths(paths);
    assert_eq!(
        paths_ser,
        "File|.file4;File|file-2;File|file1;File|file_3;File|file_text.txt".to_string()
    );
}

#[test]
fn it_should_return_paths_serialized() {
    let contents = vec![crate::core::template::TempContent {
        filename: "./src/core/tests/tree_files_only/file_text.txt".to_string(),
        text: "Lorem ipsum dolor\r\n\r\n123123123123\r\n\r\n{(-@#$%¨¨&*)}".to_string(),
    }];

    let contents_ser = serialize_contents(contents);
    assert_eq!(contents_ser, "Li9zcmMvY29yZS90ZXN0cy90cmVlX2ZpbGVzX29ubHkvZmlsZV90ZXh0LnR4dHxMb3JlbSBpcHN1bSBkb2xvcg0KDQoxMjMxMjMxMjMxMjMNCg0KeygtQCMkJcKowqgmKil9".to_string())
}

#[test]
fn it_should_return_contents_deserialized() {
    let paths_des = deserialize_paths(
        "File|.file4;File|file-2;File|file1;File|file_3;File|file_text.txt".to_string(),
    );
    assert_eq!(
        paths_des,
        vec![
            TempPath {
                buf: Path::new(".file4").to_path_buf(),
                path_type: TempPathType::File
            },
            TempPath {
                buf: Path::new("file-2").to_path_buf(),
                path_type: TempPathType::File
            },
            TempPath {
                buf: Path::new("file1").to_path_buf(),
                path_type: TempPathType::File
            },
            TempPath {
                buf: Path::new("file_3").to_path_buf(),
                path_type: TempPathType::File
            },
            TempPath {
                buf: Path::new("file_text.txt").to_path_buf(),
                path_type: TempPathType::File
            },
        ]
    )
}

#[test]
fn it_should_return_paths_deserialized() {
    let contents_des = deserialize_contents("Li9zcmMvY29yZS90ZXN0cy90cmVlX2ZpbGVzX29ubHkvZmlsZV90ZXh0LnR4dHxMb3JlbSBpcHN1bSBkb2xvcg0KDQoxMjMxMjMxMjMxMjMNCg0KeygtQCMkJcKowqgmKil9".to_string());
    assert_eq!(
        contents_des,
        vec![crate::core::template::TempContent {
            filename: "./src/core/tests/tree_files_only/file_text.txt".to_string(),
            text: "Lorem ipsum dolor\r\n\r\n123123123123\r\n\r\n{(-@#$%¨¨&*)}".to_string(),
        }]
    )
}

#[test]
fn it_should_serialize_template() {
    let template = Template {
        name: "temp-name".to_string(),
        owner: "Username".to_string(),
        created_at: "123123123123".to_string(),
        template_type: TemplateType::Local,
        paths: vec![
            TempPath {
                buf: Path::new(".file4").to_path_buf(),
                path_type: TempPathType::File,
            },
            TempPath {
                buf: Path::new("file-2").to_path_buf(),
                path_type: TempPathType::File,
            },
            TempPath {
                buf: Path::new("file1").to_path_buf(),
                path_type: TempPathType::File,
            },
            TempPath {
                buf: Path::new("file_3").to_path_buf(),
                path_type: TempPathType::File,
            },
            TempPath {
                buf: Path::new("file_text.txt").to_path_buf(),
                path_type: TempPathType::File,
            },
        ],
        contents: vec![crate::core::template::TempContent {
            filename: "./src/core/tests/tree_files_only/file_text.txt".to_string(),
            text: "Lorem ipsum dolor\r\n\r\n123123123123\r\n\r\n{(-@#$%¨¨&*)}".to_string(),
        }],
    };

    let temp_as_string = serialize_template(template).unwrap();

    assert_eq!(
        temp_as_string,
        r#"{"metadata":"eyJuYW1lIjoidGVtcC1uYW1lIiwib3duZXIiOiJVc2VybmFtZSIsImNyZWF0ZWRfYXQiOiIxMjMxMjMxMjMxMjMiLCJ0ZW1wbGF0ZV90eXBlIjoiTG9jYWwifQ==","name":"temp-name","paths":"File|.file4;File|file-2;File|file1;File|file_3;File|file_text.txt","contents":"Li9zcmMvY29yZS90ZXN0cy90cmVlX2ZpbGVzX29ubHkvZmlsZV90ZXh0LnR4dHxMb3JlbSBpcHN1bSBkb2xvcg0KDQoxMjMxMjMxMjMxMjMNCg0KeygtQCMkJcKowqgmKil9"}"#
    )
}

#[test]
fn it_should_deserialize_template() {
    let temp_as_str = r#"{"metadata":"eyJuYW1lIjoidGVtcC1uYW1lIiwib3duZXIiOiJVc2VybmFtZSIsImNyZWF0ZWRfYXQiOiIxMjMxMjMxMjMxMjMiLCJ0ZW1wbGF0ZV90eXBlIjoiTG9jYWwifQ==","name":"temp-name","paths":"File|.file4;File|file-2;File|file1;File|file_3;File|file_text.txt","contents":"Li9zcmMvY29yZS90ZXN0cy90cmVlX2ZpbGVzX29ubHkvZmlsZV90ZXh0LnR4dHxMb3JlbSBpcHN1bSBkb2xvcg0KDQoxMjMxMjMxMjMxMjMNCg0KeygtQCMkJcKowqgmKil9"}"#;
    let template = deserialize_template(temp_as_str).unwrap();

    assert_eq!(
        template,
        Template {
            name: "temp-name".to_string(),
            owner: "Username".to_string(),
            created_at: "123123123123".to_string(),
            template_type: TemplateType::Local,
            paths: vec![
                TempPath {
                    buf: Path::new(".file4").to_path_buf(),
                    path_type: TempPathType::File
                },
                TempPath {
                    buf: Path::new("file-2").to_path_buf(),
                    path_type: TempPathType::File
                },
                TempPath {
                    buf: Path::new("file1").to_path_buf(),
                    path_type: TempPathType::File
                },
                TempPath {
                    buf: Path::new("file_3").to_path_buf(),
                    path_type: TempPathType::File
                },
                TempPath {
                    buf: Path::new("file_text.txt").to_path_buf(),
                    path_type: TempPathType::File
                },
            ],
            contents: vec![crate::core::template::TempContent {
                filename: "./src/core/tests/tree_files_only/file_text.txt".to_string(),
                text: "Lorem ipsum dolor\r\n\r\n123123123123\r\n\r\n{(-@#$%¨¨&*)}".to_string(),
            }],
        }
    )
}
