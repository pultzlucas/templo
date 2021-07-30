use crate::{
    core::{
        file_system::DirPath,
        /* io::ProtternOutput, */
        requester::{HeaderValue, Method, ProtternRequester},
        user_account::UserAccountManager,
    },
    /* paint ,*/ paint_string, /* paintln */
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
pub struct TemplateManager {
    templates: Vec<Template>,
}

impl TemplateManager {
    pub fn new(templates: Vec<Template>) -> Self {
        Self { templates }
    }

    pub fn gen_templates(&self, directory: &Path) -> Result<(), Error> {
        for template in self.templates.iter() {
            let template_paths =
                TemplateManager::deserialize_template_paths(template.paths.clone());
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
            if template.has_content() {
                let template_content =
                    TemplateManager::deserialize_template_content(template.content.clone());
                // writing the files content
                for (file_name, content_buf) in template_content.into_iter() {
                    let real_file_path = Path::new(directory).join(file_name);
                    if real_file_path.exists() {
                        let mut file = fs::OpenOptions::new().write(true).open(real_file_path)?;
                        file.write(&content_buf[..])?;
                    }
                }
            };
        }

        Ok(())
    }

    pub async fn publish_templates(&self) -> Result<String, Error> {
        let current_user = UserAccountManager::get_user_account_data()?;
        let requester = ProtternRequester::new();

        let request = {
            let body = serde_json::to_string(&self.templates).expect("Error when parsing template.");
            let mut req = requester.build_request("/templates/pub", Method::POST, body);
            let headers = req.headers_mut();
            headers.insert(
                "authorization",
                HeaderValue::from_str(current_user.key.as_str()).expect("Error when set headers."),
            );
            req
        };
        let response: PublishResponse = {
            let raw_response = requester.request(request).await?;
            serde_json::from_str(&raw_response).expect("Error when parsing JSON.")
        };
        if !response.published {
            return Err(Error::new(ErrorKind::PermissionDenied, response.message));
        }
        Ok(response.message)
    }

    pub fn describe_templates(&self) {
        for template in self.templates.iter() {
            let template_paths: Vec<&str> = template.paths.split(";").collect();
            for path in template_paths.into_iter() {
                let (path_name, _) = DirPath::deserialize(path.to_string());
                println!("{}", path_name);
            }
        }
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
