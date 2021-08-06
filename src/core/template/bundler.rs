use crate::core::file_system::{DirPath, FileContent};
use regex::Regex;
use std::path::MAIN_SEPARATOR;

pub struct TemplateBundler;

impl TemplateBundler {
    /* pub fn split_template_paths(template_paths: Vec<&str>) -> Vec<(String, String)> {
        template_paths
            .into_iter()
            .map(|path| {
                let path_splitted: Vec<&str> = path.split("|").collect();
                (path_splitted[0].to_string(), path_splitted[1].to_string())
            })
            .collect()
    } */

    pub fn format_path<'a>(dir: &'a String, path: DirPath<'a>) -> DirPath<'a> {
        let mut regex = Regex::new(&dir).unwrap();

        if dir == "." {
            regex = Regex::new(r"\.$").unwrap();
        }

        let formatted_path = {
            let path_pieces: Vec<&str> = path.name.split(MAIN_SEPARATOR).collect();
            let right_path_pieces: Vec<&str> = path_pieces
                .into_iter()
                .filter(|path_piece| !regex.is_match(path_piece) && *path_piece != ".")
                .collect();
            right_path_pieces.join(MAIN_SEPARATOR.to_string().as_str())
        };

        DirPath::new(formatted_path, path.path_type)
    }

    pub fn bundle_paths(paths: Vec<DirPath>) -> String {
        let paths: Vec<String> = paths
            .into_iter()
            .map(|path| {
                if path.path_type == "file" {
                    return format!("file|{}", &path.name);
                }
                if path.path_type == "dir" {
                    return format!("dir|{}", &path.name);
                }

                panic!("Error when saving.")
            })
            .filter(|path| path != "dir|")
            .collect();

        paths.join(";").to_string()
    }

    pub fn bundle_content(file_contents: Vec<FileContent>) -> String {
        let content_vec: Vec<String> = file_contents
            .into_iter()
            .map(|fc| format!("{}|{}", fc.filename, base64::encode(fc.content)))
            .collect();
        content_vec.join(";").to_string()
    }
}
