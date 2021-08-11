use crate::core::utils::errors::std_error;
use crate::core::utils::path::pathbuf_to_string;
use crate::core::template::{TempMetadata, Template};
use crate::core::template::File;
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

pub fn deserialize_contents(content_string: String) -> Vec<File> {
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

pub fn deserialize_paths(paths_string: String) -> Vec<PathBuf> {
    split_by(paths_string, ";")
        .into_iter()
        .map(|path| split_by(path, "|")[1].clone())
        .map(|path| Path::new(&path).to_path_buf())
        .collect()
}

pub fn serialize_contents(contents: Vec<File>) -> String {
    let contents_strings: Vec<String> = contents
        .into_iter()
        .map(|content: File| [content.filename, content.content].join("|"))
        .map(base64::encode)
        .collect();
    contents_strings.join(";")
}

pub fn serialize_paths(paths: Vec<PathBuf>) -> String {
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