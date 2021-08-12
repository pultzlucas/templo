use crate::core::template::TempContent;
use crate::core::template::{TempMetadata, TempPath, TempPathType, Template};
use crate::core::utils::errors::std_error;
use crate::core::utils::path::pathbuf_to_string;
use base64;
use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::io::Error;
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct TempPreSerde {
    metadata: String,
    name: String,
    paths: String,
    contents: String,
}

pub fn serialize_template(template: Template) -> Result<String, Error> {
    let temp_pre_serde = {
        let metadata_as_string = base64::encode(serde_json::to_string(&template.metadata).unwrap());
        let paths_as_string = serialize_paths(template.paths);
        let contents_as_string = serialize_contents(template.contents);
        TempPreSerde {
            metadata: metadata_as_string,
            name: template.metadata.name,
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
            paths,
            contents,
        }
    };

    Ok(template)
}

// OPERATORS

pub fn deserialize_contents(content_string: String) -> Vec<TempContent> {
    split_by(content_string, ";")
        .into_iter()
        .map(|content_b64| decode_base64(content_b64))
        .map(|content| {
            let content_info = split_by(content, "|");
            TempContent {
                filename: content_info[0].clone(),
                text: content_info[1].clone(),
            }
        })
        .collect()
}

pub fn deserialize_paths(paths_string: String) -> Vec<TempPath> {
    split_by(paths_string, ";")
        .into_iter()
        .map(|path| {
            let path_info = split_by(path, "|");
            TempPath {
                buf: Path::new(&path_info[1]).to_path_buf(),
                path_type: deserialize_temp_path_type(path_info[0].clone()),
            }
        })
        .collect()
}

pub fn serialize_contents(contents: Vec<TempContent>) -> String {
    let contents_strings: Vec<String> = contents
        .into_iter()
        .map(|content: TempContent| [content.filename, content.text].join("|"))
        .map(base64::encode)
        .collect();
    contents_strings.join(";")
}

pub fn serialize_paths(paths: Vec<TempPath>) -> String {
    let paths_strings: Vec<String> = paths
        .into_iter()
        .map(|path: TempPath| {
            if path.path_type == TempPathType::File {
                return format!("File|{}", pathbuf_to_string(path.buf));
            }
            if path.path_type == TempPathType::Dir {
                return format!("Dir|{}", pathbuf_to_string(path.buf));
            }
            panic!("Invalid path type!");
        })
        .collect();

    paths_strings.join(";")
}

fn deserialize_temp_path_type(type_str: String) -> TempPathType {
    if type_str == "File" {
        return TempPathType::File;
    }

    if type_str == "Dir" {
        return TempPathType::Dir;
    }

    panic!("Invalid path type!");
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
