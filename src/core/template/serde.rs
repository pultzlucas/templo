use crate::core::utils::errors::std_error;
use crate::core::utils::path::pathbuf_to_string;
use crate::core::template::{TempMetadata, Template};
use crate::core::template::miner::File;
use base64;
use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::io::Error;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize)]
struct TempPreSerde {
    metadata: String,
    name: String,
    paths: String,
    contents: String,
}

pub fn serialize_template(template: &Template) -> Result<String, Error> {
    let temp_pre_serde = {
        let metadata_as_string = base64::encode(serde_json::to_string(&template.metadata).unwrap());
        let paths_as_string = serialize_paths(template.paths);
        let contents_as_string = serialize_contents(template.contents);
        TempPreSerde {
            metadata: metadata_as_string,
            name: template.name,
            paths: paths_as_string,
            contents: contents_as_string,
        }
    };

    std_error(serde_json::to_string(&temp_pre_serde))
}

pub fn deserialize_template(temp_str: &str) -> Result<Template, Error> {
    let temp_pre_serde: TempPreSerde = std_error(serde_json::from_str(temp_str))?;

    let template = {
        let metadata: TempMetadata = {
            let meta_as_string_utf8 = decode_base64(temp_pre_serde.metadata);
            serde_json::from_str(&meta_as_string_utf8).unwrap()
        };
        let paths = deserialize_paths(temp_pre_serde.paths);
        let contents = deserialize_contents(temp_pre_serde.contents);
        Template {
            metadata,
            name: temp_pre_serde.name,
            paths,
            contents,
        }
    };

    Ok(template)
}

// OPERATORS

fn deserialize_contents(content_string: String) -> Vec<File> {
    split_by(content_string, ";")
        .into_iter()
        .map(|content_b64| decode_base64(content_b64))
        .map(|content| {
            let content_info = split_by(content, "|");
            File {
                filename: content_info[0].clone(),
                content: content_info[1].clone(),
            }
        })
        .collect()
}

fn deserialize_paths(paths_string: String) -> Vec<PathBuf> {
    split_by(paths_string, ";")
        .into_iter()
        .map(|path| split_by(path, "|")[1].clone())
        .map(|path| Path::new(&path).to_path_buf())
        .collect()
}

fn serialize_contents(contents: Vec<File>) -> String {
    let contents_strings: Vec<String> = contents
        .into_iter()
        .map(|content: File| [content.filename, content.content].join("|"))
        .map(base64::encode)
        .collect();
    contents_strings.join(";")
}

fn serialize_paths(paths: Vec<PathBuf>) -> String {
    let paths_strings: Vec<String> = paths
        .into_iter()
        .map(|path: PathBuf| {
            if path.is_dir() {
                return format!("dir|{}", pathbuf_to_string(path));
            }
            if path.is_file() {
                return format!("file|{}", pathbuf_to_string(path));
            }
            panic!("Invalid path type!");
        })
        .collect();

    paths_strings.join(";")
}

fn split_by(string: String, sep: &str) -> Vec<String> {
    string
        .split(sep)
        .into_iter()
        .map(|piece: &str| piece.to_string())
        .collect()
}

fn decode_base64(b64: String) -> String {
    String::from_utf8(base64::decode(b64).unwrap()).unwrap()
}

// TESTS

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::template::maker::{TempMetadata, Template};
    use crate::core::template::TemplateType;
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
                owner: "Username".to_string(),
                created_at: "123123123123".to_string(),
                template_type: TemplateType::Local,
            },
            name: "temp-name".to_string(),
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

        let temp_as_string = serialize_template(&template).unwrap();
        assert_eq!(
            temp_as_string,
            r#"{"metadata":"eyJvd25lciI6IlVzZXJuYW1lIiwiY3JlYXRlZF9hdCI6IjEyMzEyMzEyMzEyMyIsInRlbXBsYXRlX3R5cGUiOiJMb2NhbCJ9","name":"temp-name","paths":"dir|./src/core/tests/tree_files_only;file|./src/core/tests/tree_files_only/.file4;file|./src/core/tests/tree_files_only/file-2;file|./src/core/tests/tree_files_only/file1;file|./src/core/tests/tree_files_only/file_3;file|./src/core/tests/tree_files_only/file_text.txt","contents":"Li9zcmMvY29yZS90ZXN0cy90cmVlX2ZpbGVzX29ubHkvZmlsZV90ZXh0LnR4dHxMb3JlbSBpcHN1bSBkb2xvcg0KDQoxMjMxMjMxMjMxMjMNCg0KeygtQCMkJcKowqgmKil9"}"#
        )
    }

    #[test]
    fn it_should_deserialize_template() {
        let temp_as_str = r#"{"metadata":"eyJvd25lciI6IlVzZXJuYW1lIiwiY3JlYXRlZF9hdCI6IjEyMzEyMzEyMzEyMyIsInRlbXBsYXRlX3R5cGUiOiJMb2NhbCJ9","name":"temp-name","paths":"dir|./src/core/tests/tree_files_only;file|./src/core/tests/tree_files_only/.file4;file|./src/core/tests/tree_files_only/file-2;file|./src/core/tests/tree_files_only/file1;file|./src/core/tests/tree_files_only/file_3;file|./src/core/tests/tree_files_only/file_text.txt","contents":"Li9zcmMvY29yZS90ZXN0cy90cmVlX2ZpbGVzX29ubHkvZmlsZV90ZXh0LnR4dHxMb3JlbSBpcHN1bSBkb2xvcg0KDQoxMjMxMjMxMjMxMjMNCg0KeygtQCMkJcKowqgmKil9"}"#;
        let template = deserialize_template(temp_as_str).unwrap();

        assert_eq!(
            template,
            Template {
                metadata: TempMetadata {
                    owner: "Username".to_string(),
                    created_at: "123123123123".to_string(),
                    template_type: TemplateType::Local,
                },
                name: "temp-name".to_string(),
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
                    content: "Lorem ipsum dolor\r\n\r\n123123123123\r\n\r\n{(-@#$%¨¨&*)}"
                        .to_string(),
                }],
            }
        )
    }
}
