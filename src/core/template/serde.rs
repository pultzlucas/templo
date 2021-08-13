use crate::core::template::TempContent;
use crate::core::template::{TempPath, TempPathType, Template, TemplateType};
use crate::core::utils::errors::std_error;
use crate::core::utils::path::{pathbuf_to_string, str_to_pathbuf};
use base64;
use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::io::Error;

#[derive(Serialize, Deserialize)]
struct TempPreSerde {
    name: String,
    owner: String,
    created_at: String,
    template_type: TemplateType,
    paths: String,
    contents: String,
}

pub fn serialize_template(template: Template) -> Result<String, Error> {
    let temp_pre_serde = temp_to_pre_serde(template);
    std_error(serde_json::to_string(&temp_pre_serde))
}

pub fn deserialize_template(temp_str: &str) -> Result<Template, Error> {
    let temp_pre_serde: TempPreSerde = std_error(serde_json::from_str(temp_str))?;
    let template = {
        let paths = deserialize_paths(temp_pre_serde.paths);
        let contents = deserialize_contents(temp_pre_serde.contents);
        Template {
            name: temp_pre_serde.name,
            owner: temp_pre_serde.owner,
            created_at: temp_pre_serde.created_at,
            template_type: temp_pre_serde.template_type,
            paths,
            contents,
        }
    };

    Ok(template)
}

pub fn serialize_template_vec(temp_vec: Vec<Template>) -> Result<String, Error> {
    let temps_pre_serde: Vec<TempPreSerde> =
        temp_vec.into_iter().map(temp_to_pre_serde).collect();

    std_error(serde_json::to_string(&temps_pre_serde))
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
                buf: str_to_pathbuf(&path_info[1]),
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

fn temp_to_pre_serde(template: Template) -> TempPreSerde {
    let paths_as_string = serialize_paths(template.paths);
    let contents_as_string = serialize_contents(template.contents);
    TempPreSerde {
        name: template.name,
        owner: template.owner,
        created_at: template.created_at,
        template_type: template.template_type,
        paths: paths_as_string,
        contents: contents_as_string,
    }
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
