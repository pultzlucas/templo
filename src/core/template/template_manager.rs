use crate::{
    core::{
        file_system::{DirPath, FileContent},
        io::ProtternOutput,
        requester::{HeaderValue, Method, ProtternRequester},
        user_account::{UserAccountManager, UserPermissions},
    },
    paint, paint_string, paintln,
};
use regex::Regex;
use serde_derive::{Deserialize, Serialize};
use std::{
    fs,
    io::{Error, ErrorKind, Write},
    path::{Path, MAIN_SEPARATOR},
};

#[derive(Deserialize, Serialize)]
pub struct PublishResponse {
    pub published: bool,
    pub message: String,
}

use super::Template;
pub struct TemplateManager {
    template: Template,
}

impl TemplateManager {
    pub fn new(template: Template) -> Self {
        Self { template }
    }

    pub fn create_template(&self, directory: &Path) -> Result<(), Error> {
        let template_paths = TemplateManager::deserialize_template_paths(self.template.paths.clone());
        let template_content = TemplateManager::deserialize_template_content(self.template.content.clone());

        // creating files and directories
        for (path_type, path_name) in template_paths.into_iter() {
            let real_path = Path::new(directory).join(path_name);
            if path_type == "file" {
                fs::write(&real_path, "")?;
                println!("{} {:?}", paint_string!("{gray}", "file:"), real_path);
            }
            if path_type == "dir" {
                fs::create_dir(&real_path)?;
                println!(" {} {:?}", paint_string!("{gray}", "dir:"), real_path);
            }
        }

        // writing the files content
        for (file_name, content_buf) in template_content.into_iter() {
            let real_file_path = Path::new(directory).join(file_name);
            if real_file_path.exists() {
                let mut file = fs::OpenOptions::new().write(true).open(real_file_path)?;
                file.write(&content_buf[..])?;
            }
        }

        Ok(())
    }

    pub async fn publish_template(&self) -> Result<String, Error> {
        let has_permission_to = UserPermissions::new();
        if !has_permission_to.publish_template(&self.template.name) {
            let err = Error::new(
                ErrorKind::PermissionDenied,
                format!(
                    "You do not has permission to publish \"{}\".",
                    self.template.name
                ),
            );
            return Err(err);
        }

        let current_user = UserAccountManager::get_user_account_data()?;

        let request = {
            let body = serde_json::to_string(&self.template).expect("Error when parsing template.");
            let mut req = ProtternRequester::build_request("/templates/pub", Method::POST, body);
            let headers = req.headers_mut();
            headers.insert(
                "authorization",
                HeaderValue::from_str(current_user.key.as_str()).expect("Error when set headers."),
            );

            req
        };

        let response: PublishResponse = {
            let raw_response = ProtternRequester::request(request).await?;
            serde_json::from_str(&raw_response).expect("Error when parsing JSON.")
        };

        if !response.published {
            return Err(Error::new(ErrorKind::PermissionDenied, response.message));
        }

        Ok(response.message)
    }

    pub fn split_template_paths(template_paths: Vec<&str>) -> Vec<(String, String)> {
        template_paths
            .into_iter()
            .map(|path| {
                let path_splitted: Vec<&str> = path.split("|").collect();
                (path_splitted[0].to_string(), path_splitted[1].to_string())
            })
            .collect()
    }

    pub fn describe_template(&self) {
        paintln!("{yellow} name", ">>");
        paint!("   {gray} ", "|");
        println!("{}\n", self.template.name);
        paintln!("{yellow} type", ">>");
        paint!("   {gray} ", "|");
        println!("{:?}\n", self.template.template_type);
        paintln!("{yellow} owner", ">>");
        paint!("   {gray} ", "|");
        println!("{}\n", self.template.owner);
        paintln!("{yellow} created at", ">>");
        paint!("   {gray} ", "|");
        println!("{}\n", self.template.created_at);
        paintln!("{yellow} paths", ">>");
        let template_paths: Vec<&str> = self.template.paths.split(";").collect();
        ProtternOutput::print_template_paths(template_paths);
    }

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
            .map(|fc| format!("{}|{}", fc.file, base64::encode(fc.content)))
            .collect();
        content_vec.join(";").to_string()
    }

    fn deserialize_template_paths(paths: String) -> Vec<(String, String)> {
        let template_paths_splitted: Vec<&str> = paths.split(";").collect();
        template_paths_splitted
            .into_iter()
            .map(|path| {
                let path_splitted: Vec<String> =
                    path.split("|").map(|piece| piece.to_string()).collect();
                (path_splitted[0].clone(), path_splitted[1].clone())
            })
            .collect()
    }

    fn deserialize_template_content(content: String) -> Vec<(String, Vec<u8>)> {
        let template_content_splitted: Vec<&str> = content.split(";").collect();
        template_content_splitted
            .into_iter()
            .map(|content| {
                let content_splitted: Vec<String> =
                    content.split("|").map(|piece| piece.to_string()).collect();
                (
                    content_splitted[0].clone(),
                    base64::decode(content_splitted[1].clone()).unwrap(),
                )
            })
            .collect()
    }
}
