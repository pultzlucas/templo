use crate::core::{
    file_system::{paths::TEMPLATES_PATH, read_base64_file, write_base64_file},
    template::Template,
    user_account::UserPermissions,
};
use crate::utils::errors::{not_found_error, permission_denied_error};
use crate::utils::errors::std_error;
use std::{
    fs,
    io::Error,
    path::{Path, PathBuf},
};

pub fn exists() -> bool {
    Path::new(TEMPLATES_PATH).exists()
}

pub fn create() -> Result<(), Error> {
    if !exists() {
        fs::create_dir_all(TEMPLATES_PATH)?;
        println!("Repository was created.");
    }

    Ok(())
}

pub fn save_template(template: Template) -> Result<(), Error> {
    let template_path = get_template_path(&template.name);
    let template_string = std_error(serde_json::to_string(&template))?;
    write_base64_file(template_path, template_string)
}

pub fn get_templates() -> Result<Vec<Template>, Error> {
    fs::read_dir(TEMPLATES_PATH)
        .unwrap()
        .map(|template| template.map(|e| e.path()))
        .map(|file| read_base64_file(file?))
        .map(|temp_string| std_error(serde_json::from_str(&temp_string?)))
        .collect()
}

pub fn get_template(template_name: &str) -> Result<Template, Error> {
    let template = {
        if is_empty() {
            return Err(not_found_error("Repository is empty."));
        }

        let matched_template = get_templates()?
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

pub fn delete_template(template_name: &str) -> Result<(), Error> {
    if !template_exists(template_name) {
        return Err(not_found_error(&format!(
            "Not is possible find \"{}\" on repository",
            template_name
        )));
    }

    let has_permission_to = UserPermissions::new();

    if !has_permission_to.delete_template(template_name) {
        return Err(permission_denied_error(&format!(
            "You do not has permission to delete \"{}\".",
            template_name
        )));
    }

    let template_path = get_template_path(template_name);
    fs::remove_file(template_path)?;
    Ok(())
}

pub fn total_templates() -> usize {
    let temps = [fs::read_dir(TEMPLATES_PATH).unwrap()];
    temps.len()
}

pub fn template_exists(template_name: &str) -> bool {
    get_template_path(template_name).exists()
}

/* pub fn get_remote_templates() -> Vec<Template> {
    get_templates_type(TemplateType::Remote)
}

pub fn get_local_templates() -> Vec<Template> {
    get_templates_type(TemplateType::Local)
} */

pub fn is_empty() -> bool {
    total_templates() == 0
}

/* fn get_templates_type(temp_type: TemplateType) -> Vec<Template> {
    get_templates()
        .clone()
        .into_iter()
        .filter(|temp| temp.template_type == temp_type)
        .collect()
} */

fn get_template_path(template_name: &str) -> PathBuf {
    Path::new(TEMPLATES_PATH)
        .join(template_name)
        .with_extension("ptmp")
}
