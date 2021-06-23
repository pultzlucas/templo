use crate::core::{
    file_system::{paths::TEMPLATES_PATH, ProtternFileSystem},
    requester::{HeaderValue, Method, ProtternRequester},
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
        let template_string = serde_json::to_string(&template).unwrap();
        ProtternFileSystem::write_base64_file(template_path, template_string)
    }

    pub fn get_template(template_name: &String) -> Result<Template, Error> {
        let template = {
            let all_templates = match TemplateManager::get_all_templates() {
                Some(t) => t,
                None => return Err(Error::new(ErrorKind::NotFound, "Repository is empty.")),
            };
            let matched_template = all_templates
                .into_iter()
                .find(|temp| temp.name == *template_name);
            match matched_template {
                Some(t) => t,
                None => {
                    return Err(Error::new(
                        ErrorKind::NotFound,
                        format!("Not is possible find \"{}\" on repository", template_name),
                    ))
                }
            }
        };

        Ok(template)
    }

    pub fn get_all_templates<'a>() -> Option<Vec<Template>> {
        let templates: Vec<Template> = fs::read_dir(TEMPLATES_PATH)
            .unwrap()
            .map(|template| template.map(|e| e.path()))
            .map(|template_file| {
                let template_file_string =
                    ProtternFileSystem::read_base64_file(template_file.unwrap()).unwrap();
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
                format!(
                    "You do not has permission to publish \"{}\".",
                    template.name
                ),
            );
            return Err(err);
        }

        let current_user = UserAccountManager::get_user_account_data()?;

        let request = {
            let body = match serde_json::to_string(&template) {
                Err(e) => return Err(Error::new(ErrorKind::Other, e.to_string())),
                Ok(t) => t,
            };
            let mut req = ProtternRequester::build_request("/templates/pub", Method::POST, body);
            let headers = req.headers_mut();
            headers.insert(
                "authorization",
                HeaderValue::from_str(current_user.key.as_str()).expect("Error when set headers."),
            );

            req
        };


        match ProtternRequester::request(request).await {
            Err(e) => Err(e),
            Ok(res) => {
                let res_json: PublishResponse =
                    serde_json::from_str(&res).expect("Error when parsing JSON.");

                if !res_json.published {
                    return Err(Error::new(ErrorKind::PermissionDenied, res_json.message));
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
        fs::remove_file(template_path)?;
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
