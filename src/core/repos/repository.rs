use super::repo_exists;
use crate::core::path::get_repo_path;
use crate::core::template::Template;
use crate::core::utils::errors::already_exists_error;
use crate::core::utils::errors::not_found_error;
use crate::core::utils::errors::repo_connection_error;
use crate::core::utils::errors::std_error;
use serde_json::{from_str, to_string_pretty};
use std::{
    fs,
    io::Error,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct Repository {
    pub name: String,
    pub path: PathBuf,
}

impl Repository {
    pub fn connect(name: String) -> Result<Self, Error> {
        if !repo_exists(&name) {
            let err_msg = format!("Error when connect to repo. Repo \"{}\" not exists.", name);
            return Err(repo_connection_error(&err_msg));
        }

        let path = get_repo_path(&name)?;

        Ok(Self { name, path })
    }

    pub fn create(repo_name: &str) -> Result<(), Error> {
        if !repo_exists(repo_name) {
            let repo_path = get_repo_path(repo_name)?;
            fs::create_dir_all(&repo_path)?;
            println!("Repository \"{}\" was created.", repo_name);
        }

        Ok(())
    }

    pub fn has_template(&self, template_name: &str) -> bool {
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
        fs::read_dir(&self.path)?
            .map(|template| template.map(|e| e.path()))
            .map(|file| fs::read_to_string(file?))
            .map(|temp_string| {
                Ok(from_str(&temp_string?)
                    .expect("Error when deserializing template string object."))
            })
            .collect()
    }

    pub fn get_template(&self, template_name: &str) -> Result<Template, Error> {
        if self.is_empty() {
            return Err(not_found_error(&format!(
                "Repo \"{}\" is empty.",
                self.name
            )));
        }

        let template_file = fs::read_dir(&self.path)?.into_iter().find(|temp_file| {
            temp_file.as_ref().unwrap().file_name().to_str().unwrap()
                == format!("{}.tpo", template_name)
        });

        if let Some(temp) = template_file {
            let template_file_string = fs::read_to_string(temp?.path())?;
            return Ok(from_str(&template_file_string)
                .expect("Error when deserializing template string object."));
        } else {
            return Err(not_found_error(&format!(
                "Not is possible find \"{}\" on \"{}\" repository",
                template_name, self.name
            )));
        }
    }

    pub fn delete_template(&self, template_name: &str) -> Result<(), Error> {
        if !self.has_template(template_name) {
            return Err(not_found_error(&format!(
                "Not is possible find \"{}\" on \"{}\" repository",
                template_name, self.name
            )));
        }

        let template_path = self.get_template_path(template_name);
        fs::remove_file(template_path)?;

        Ok(())
    }
    pub fn update_template_name(
        &self,
        old_template_name: &str,
        new_template_name: String,
    ) -> Result<(), Error> {
        let old_template = self.get_template(old_template_name)?;
        self.delete_template(old_template_name)?;

        let new_template = Template {
            name: new_template_name,
            description: old_template.description,
            created_at: old_template.created_at,
            updated_at: old_template.updated_at,
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
        self.delete_template(template_name)?;

        let new_template = Template {
            name: old_template.name,
            description: new_template_description,
            created_at: old_template.created_at,
            updated_at: old_template.updated_at,
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
        self.delete_template(&old_template_name)?;
        self.save_template(new_template)?;
        Ok(())
    }

    pub fn get_template_path(&self, template_name: &str) -> PathBuf {
        Path::new(&get_repo_path(&self.name).unwrap())
            .join(template_name)
            .with_extension("tpo")
    }

    pub fn move_template_to(&self, template_name: &str, repo: &Repository) -> Result<(), Error> {
        let template_path = self.get_template_path(template_name);
        let template = self.get_template(template_name)?;

        if repo.has_template(template_name) {
            return Err(already_exists_error(&format!(
                "Already exists a template named as \"{}\" in \"{}\" repo.",
                template_name, repo.name
            )));
        }

        fs::remove_file(template_path)?;
        repo.save_template(template)?;
        Ok(())
    }
}
