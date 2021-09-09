use crate::core::{path::get_templates_path, template::Template};
use crate::utils::errors::not_found_error;
use crate::utils::errors::std_error;
use std::{
    fs,
    io::Error,
    path::{Path, PathBuf},
};

pub fn exists() -> bool {
    Path::new(&get_templates_path().unwrap()).exists()
}

pub fn create() -> Result<(), Error> {
    if !exists() {
        fs::create_dir_all(&get_templates_path().unwrap())?;
        println!("Repository was created.");
    }

    Ok(())
}

pub fn save_template(template: Template) -> Result<(), Error> {
    let template_path = get_template_path(&template.name);
    let template_string = std_error(serde_json::to_string_pretty(&template))?;
    fs::write(template_path, template_string)
}

pub fn get_templates() -> Result<Vec<Template>, Error> {
    fs::read_dir(&get_templates_path().unwrap())
        .unwrap()
        .map(|template| template.map(|e| e.path()))
        .map(|file| fs::read(file?))
        .map(|bytes| std_error(String::from_utf8(bytes?)))
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

pub fn delete_template(template_name: String) -> Result<(), Error> {
    if !template_exists(&template_name) {
        return Err(not_found_error(&format!(
            "Not is possible find \"{}\" on repository",
            template_name
        )));
    }

    let template_path = get_template_path(&template_name);
    fs::remove_file(template_path)?;

    Ok(())
}

pub fn update_template_name(old_template_name: &str, new_template_name: &str) -> Result<(), Error> {
    let old_template = get_template(old_template_name)?;
    delete_template(old_template_name.to_string())?;

    let new_template = Template {
        name: new_template_name.to_string(),
        contents: old_template.contents,
        created_at: old_template.created_at,
        paths: old_template.paths,
    };

    save_template(new_template)?;

    Ok(())
}

pub fn update_template_content(
    old_template_name: String,
    new_template: Template,
) -> Result<(), Error> {
    delete_template(old_template_name)?;
    save_template(new_template)?;
    Ok(())
}

pub fn total_templates() -> usize {
    let temps = fs::read_dir(&get_templates_path().unwrap()).unwrap();
    temps.count()
}

pub fn template_exists(template_name: &str) -> bool {
    get_template_path(template_name).exists()
}

pub fn is_empty() -> bool {
    total_templates() == 0
}

fn get_template_path(template_name: &str) -> PathBuf {
    Path::new(&get_templates_path().unwrap())
        .join(template_name)
        .with_extension("tpo")
}
