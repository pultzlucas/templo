use crate::core::file_system::{paths::TEMPLATES_PATH, ProtternFileSystem};
use std::{
    fs,
    io::{Error, ErrorKind},
    path::Path,
};

use super::Template;
pub struct TemplateManager {}

impl TemplateManager {
    pub fn save_template(template: Template) -> Result<(), Error> {
        let template_path = ProtternFileSystem::get_template_path(&template.name);
        let template_file_buf = serde_json::to_string_pretty(&template).unwrap();

        fs::write(template_path, template_file_buf)
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
                let template_file_buf = fs::read_to_string(template_file).unwrap();
                serde_json::from_str(template_file_buf.as_str()).unwrap()
            })
            .collect();
        if templates.is_empty() {
            return None;
        }
        Some(templates)
    }

    pub fn delete_template(template_name: &String) -> Result<(), Error> {
        if !TemplateManager::template_exists(template_name) {
            let err = Error::new(
                ErrorKind::NotFound,
                format!("Not is possible find \"{}\" on repository", template_name),
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
}
