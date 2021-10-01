use crate::core::path::get_root_repos_path;
use crate::core::template::Template;
use crate::utils::errors::not_found_error;
use crate::utils::errors::repo_connection_error;
use crate::utils::errors::std_error;
use serde_json::{from_str, to_string_pretty};
use std::{
    fs,
    io::Error,
    path::{Path, PathBuf},
};

use super::path::get_repo_path;

pub struct Repository {
    name: String,
}

impl Repository {
    pub fn connect(name: String) -> Result<Self, Error> {
        if Self::exists(&name) {
            let err_msg = format!("Error when connect to repo. Repo \"{}\" not exists.", name);
            return Err(repo_connection_error(&err_msg));
        }

        Ok(Self { name })
    }

    pub fn create(repo_name: &str) -> Result<(), Error> {
        if !Self::exists(repo_name) {
            let repo_path = get_root_repos_path()?.join(repo_name);
            fs::create_dir_all(&repo_name)?;
            println!("Repository was created.");
        }

        Ok(())
    }

    pub fn exists(repo_name: &str) -> bool {
        Path::new(&get_repo_path(repo_name).unwrap()).exists()
    }

    pub fn template_exists(&self, template_name: &str) -> bool {
        self.get_template_path(template_name).exists()
    }

    pub fn is_empty(&self) -> bool {
        self.total_templates() == 0
    }

    pub fn total_templates(&self) -> usize {
        let templates_path = get_repo_path(&self.name).expect("Error when get templates path.");
        let temps = fs::read_dir(&templates_path).expect("Error when reading templates dir.");
        temps.count()
    }

    pub fn save_template(&self, template: Template) -> Result<(), Error> {
        let template_path = self.get_template_path(&template.name);
        let template_string = std_error(to_string_pretty(&template))?;
        fs::write(template_path, template_string)
    }

    pub fn get_templates(&self) -> Result<Vec<Template>, Error> {
        fs::read_dir(&get_repo_path(&self.name)?)?
            .map(|template| template.map(|e| e.path()))
            .map(|file| fs::read_to_string(file?))
            .map(|temp_string| std_error(from_str(&temp_string?)))
            .collect()
    }

    pub fn get_template(&self, template_name: &str) -> Result<Template, Error> {
        let template = {
            if self.is_empty() {
                return Err(not_found_error("Repository is empty."));
            }

            let matched_template = self
                .get_templates()?
                .clone()
                .into_iter()
                .find(|temp| temp.name == *template_name);

            match matched_template {
                Some(t) => t,
                None => {
                    return Err(not_found_error(&format!(
                        "Not is possible find \"{}\" on repository",
                        template_name
                    )))
                }
            }
        };

        Ok(template)
    }

    pub fn delete_template(&self, template_name: String) -> Result<(), Error> {
        if !self.template_exists(&template_name) {
            return Err(not_found_error(&format!(
                "Not is possible find \"{}\" on repository",
                template_name
            )));
        }

        let template_path = self.get_template_path(&template_name);
        fs::remove_file(template_path)?;

        Ok(())
    }
    pub fn update_template_name(
        &self,
        old_template_name: &str,
        new_template_name: String,
    ) -> Result<(), Error> {
        let old_template = self.get_template(old_template_name)?;
        self.delete_template(old_template_name.to_string())?;

        let new_template = Template {
            name: new_template_name,
            description: old_template.description,
            contents: old_template.contents,
            created_at: old_template.created_at,
            paths: old_template.paths,
            args: old_template.args,
        };

        self.save_template(new_template)?;

        Ok(())
    }

    pub fn update_template_description(
        &self,
        template_name: &str,
        new_template_description: Option<String>,
    ) -> Result<(), Error> {
        let old_template = self.get_template(template_name)?;
        self.delete_template(template_name.to_string())?;

        let new_template = Template {
            name: old_template.name,
            description: new_template_description,
            contents: old_template.contents,
            created_at: old_template.created_at,
            paths: old_template.paths,
            args: old_template.args,
        };

        self.save_template(new_template)?;

        Ok(())
    }

    pub fn update_template_content(
        &self,
        old_template_name: String,
        new_template: Template,
    ) -> Result<(), Error> {
        self.delete_template(old_template_name)?;
        self.save_template(new_template)?;
        Ok(())
    }

    pub fn get_template_path(&self, template_name: &str) -> PathBuf {
        Path::new(&get_repo_path(&self.name).unwrap())
            .join(template_name)
            .with_extension("tpo")
    }
}
