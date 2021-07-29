use crate::core::{
    file_system::{paths::TEMPLATES_PATH, ProtternFileSystem},
    template::{Template, TemplateType},
    user_account::UserPermissions,
};
use std::{
    fs,
    io::{Error, ErrorKind},
    path::Path,
};

#[derive(Clone)]
pub struct RepositoryConnection {
    templates: Vec<Template>,
}

impl RepositoryConnection {
    pub fn new() -> Self {
        let templates = RepositoryConnection::get_all_templates();
        Self { templates }
    }

    fn get_all_templates() -> Vec<Template> {
        let templates: Vec<Template> = fs::read_dir(TEMPLATES_PATH)
            .unwrap()
            .map(|template| template.map(|e| e.path()))
            .map(|template_file| {
                let template_file_string =
                    ProtternFileSystem::read_base64_file(template_file.unwrap()).unwrap();
                serde_json::from_str(template_file_string.as_str()).unwrap()
            })
            .collect();

        templates
    }

    pub fn total_templates(&self) -> usize {
        RepositoryConnection::get_all_templates().len()
    }

    pub fn save_template(&self, template: Template) -> Result<(), Error> {
        let template_path = ProtternFileSystem::get_template_path(&template.name);
        let template_string = serde_json::to_string(&template).unwrap();
        ProtternFileSystem::write_base64_file(template_path, template_string)
    }

    pub fn template_exists(&self, template_name: &String) -> bool {
        Path::new(TEMPLATES_PATH).join(template_name).exists()
    }

    pub fn get_template(&self, template_name: &String) -> Result<Template, Error> {
        let template = {
            if self.is_empty() {
                return Err(Error::new(ErrorKind::NotFound, "Repository is empty."));
            }

            let matched_template = self
                .templates
                .clone()
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

    pub fn get_remote_templates(&self) -> Vec<Template> {
        self.templates
            .clone()
            .into_iter()
            .filter(|temp| temp.template_type == TemplateType::Remote)
            .collect()
    }
    pub fn get_local_templates(&self) -> Vec<Template> {
        self.templates
            .clone()
            .into_iter()
            .filter(|temp| temp.template_type == TemplateType::Local)
            .collect()
    }

    pub fn delete_template(&self, template_name: &String) -> Result<(), Error> {
        if !self.template_exists(&template_name) {
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

    pub fn is_empty(&self) -> bool {
        self.templates.len() == 0
    }
}
