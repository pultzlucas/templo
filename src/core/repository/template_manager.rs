use crate::core::{
    file_system::{paths::TEMPLATES_PATH, ProtternFileSystem},
    requester::{Method, ProtternRequester, HeaderValue},
    user_account::{UserAccountManager, UserPermissions},
};
use serde_derive::{Deserialize, Serialize};
use std::{
    fs,
    io::{Error, ErrorKind},
    path::Path,
};

#[derive(Deserialize, Serialize)]
pub struct PublishResponse {
    pub published: bool,
    pub message: String,
}

use super::Template;
pub struct TemplateManager {}

impl TemplateManager {
    pub fn save_template(template: Template) -> Result<(), Error> {
        let template_path = ProtternFileSystem::get_template_path(&template.name);
        let template_file_string = serde_json::to_string(&template).unwrap();
        ProtternFileSystem::write_base64_file(template_path, template_file_string)
    }

    pub fn get_template(template_name: &String) -> Result<Template, Error> {
        let templates_struct = match TemplateManager::get_all_templates() {
            Some(t) => t,
            None => {
                let err = Error::new(ErrorKind::NotFound, "Repository was empty.");
                return Err(err);
            }
        };
        let template_option = templates_struct
            .into_iter()
            .find(|temp| temp.name == *template_name);
        let template = match template_option {
            Some(t) => t,
            None => {
                let err = Error::new(
                    ErrorKind::NotFound,
                    format!("Not is possible find \"{}\" on repository", template_name),
                );

                return Err(err);
            }
        };
        Ok(template)
    }

    pub fn get_all_templates<'a>() -> Option<Vec<Template>> {
        let template_files = fs::read_dir(TEMPLATES_PATH)
            .unwrap()
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, Error>>()
            .unwrap();
        let templates: Vec<Template> = template_files
            .iter()
            .map(|template_file| {
                //let template_file_buf = fs::read_to_string(template_file).unwrap();
                //let template_file_bytes = base64::decode(template_file_buf).expect("Decode error");
                let template_file_string =
                    ProtternFileSystem::read_base64_file(template_file).unwrap();
                serde_json::from_str(template_file_string.as_str()).unwrap()
            })
            .collect();
        if templates.is_empty() {
            return None;
        }
        Some(templates)
    }

    pub async fn publish_template(template: Template) -> Result<String, Error> {

        let has_permission_to = UserPermissions::new();
        if !has_permission_to.publish_template(&template.name) {
            let err = Error::new(
                ErrorKind::PermissionDenied,
                format!("You do not has permission to publish \"{}\".", template.name),
            );
            return Err(err);
        }

        let current_user = UserAccountManager::get_user_account_data()?;

        let template_as_string = match serde_json::to_string(&template) {
            Err(e) => {
                let err = Error::new(ErrorKind::Other, e.to_string());
                return Err(err)
            },
            Ok(t) => t,
        };

        let mut req =
            ProtternRequester::build_request("/templates/pub", Method::POST, template_as_string);

        let headers = req.headers_mut();
        headers.insert(
            "authorization",
            HeaderValue::from_str(current_user.key.as_str()).expect("Error when set headers."),
        );

        match ProtternRequester::request(req).await {
            Err(e) => Err(e),
            Ok(res) => {
                let res_json: PublishResponse =
                    serde_json::from_str(&res).expect("Error when parsing JSON.");

                if !res_json.published {
                    let err = Error::new(ErrorKind::PermissionDenied, res_json.message);
                    return Err(err);
                }

                Ok(res_json.message)
            }
        }
    }

    pub fn delete_template(template_name: &String) -> Result<(), Error> {
        if !TemplateManager::template_exists(template_name) {
            let err = Error::new(
                ErrorKind::NotFound,
                format!("Not is possible find \"{}\" on repository", template_name),
            );
            return Err(err);
        }

        let has_permission_to = UserPermissions::new();

        if !has_permission_to.delete_template(&template_name) {
            let err = Error::new(
                ErrorKind::PermissionDenied,
                format!("You do not has permission to delete \"{}\".", template_name),
            );
            return Err(err);
        }

        let template_path = Path::new(TEMPLATES_PATH).join(template_name);
        if let Err(e) = fs::remove_dir_all(template_path) {
            return Err(e);
        }
        Ok(())
    }

    pub fn template_exists(template_name: &String) -> bool {
        Path::new(TEMPLATES_PATH).join(template_name).exists()
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
}
