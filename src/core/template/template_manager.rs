use crate::{
    core::{
        repository::RepositoryConnection,
        requester::{HeaderValue, Method, ProtternRequester},
        user_account::{UserAccountManager, UserPermissions},
    },
    paint_string,
};
use serde_derive::{Deserialize, Serialize};
use std::{
    fs,
    io::{Error, ErrorKind, Write},
    path::Path,
};

#[derive(Deserialize, Serialize)]
pub struct PublishResponse {
    pub published: bool,
    pub message: String,
}

use super::Template;
pub struct TemplateManager;

impl TemplateManager {
    pub fn create_template(template_name: &String, directory: &Path) -> Result<(), Error> {
        let repository = RepositoryConnection::new();
        let template = repository.get_template(&template_name)?;
        let template_paths = TemplateManager::deserialize_template_paths(template.paths);
        let template_content = TemplateManager::deserialize_template_content(template.content);

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

    pub async fn publish_template(template: Template) -> Result<String, Error> {
        let has_permission_to = UserPermissions::new();
        if !has_permission_to.publish_template(&template.name) {
            let err = Error::new(
                ErrorKind::PermissionDenied,
                format!(
                    "You do not has permission to publish \"{}\".",
                    template.name
                ),
            );
            return Err(err);
        }

        let current_user = UserAccountManager::get_user_account_data()?;

        let request = {
            let body = serde_json::to_string(&template).expect("Error when parsing template.");
            let mut req = ProtternRequester::build_request("/templates/pub", Method::POST, body);
            let headers = req.headers_mut();
            headers.insert(
                "authorization",
                HeaderValue::from_str(current_user.key.as_str()).expect("Error when set headers."),
            );

            req
        };

        let response = ProtternRequester::request(request).await?;

        let res_json: PublishResponse =
            serde_json::from_str(&response).expect("Error when parsing JSON.");

        if !res_json.published {
            return Err(Error::new(ErrorKind::PermissionDenied, res_json.message));
        }

        Ok(res_json.message)
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
