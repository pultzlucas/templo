use crate::core::utils::errors::{permission_denied_error, std_error};
use crate::{
    core::{
        requester::{build_request, request, HeaderValue, Method},
        user_account::UserAccountManager,
        utils::path::pathbuf_to_string,
    },
    paint_string,
};
use serde_derive::{Deserialize, Serialize};
use std::{fs, io::Error, path::Path};

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
        for template in self.templates.clone().into_iter() {
            // creating files and directories
            for path in template.paths.into_iter() {
                let real_path = Path::new(directory)
                    .join(pathbuf_to_string(path))
                    .to_path_buf();
                if real_path.is_file() {
                    fs::write(&real_path, "")?;
                    println!("{} {:?}", paint_string!("{gray}", "file:"), real_path);
                }

                if real_path.is_dir() {
                    fs::create_dir(&real_path)?;
                    println!(" {} {:?}", paint_string!("{gray}", "dir:"), real_path);
                }
            }

            if template.contents.len() > 0 {
                // writing the files content
                for data in template.contents.into_iter() {
                    let file_path = Path::new(directory).join(data.filename);
                    if file_path.exists() {
                        fs::write(file_path, data.content)?;
                    }
                }
            };
        }

        Ok(())
    }

    pub async fn publish_templates(&self) -> Result<String, Error> {
        let current_user = UserAccountManager::get_user_account_data()?;

        let req = {
            let body = std_error(serde_json::to_string(&self.templates))?;
            let mut req = build_request("/templates/pub", Method::POST, body);
            let headers = req.headers_mut();
            headers.insert(
                "authorization",
                std_error(HeaderValue::from_str(current_user.key.as_str()))?,
            );
            req
        };

        let response: PublishResponse = {
            let res = request(req).await?;
            std_error(serde_json::from_str(&res))?
        };
        if !response.published {
            return Err(permission_denied_error(&response.message));
        }
        Ok(response.message)
    }
}
