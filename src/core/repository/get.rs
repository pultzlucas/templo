use std::{fs, io};

extern crate serde_json;
use crate::utils::paths::TEMPLATES_PATH;
use crate::utils::structs::Template;

pub fn get_templates_as_struct() -> Option<Vec<Template>> {
    let dir_names = fs::read_dir(TEMPLATES_PATH)
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();

    let heads: Vec<Template> = dir_names
        .iter()
        .map(|dir| {
            let head_path = dir.join("HEAD.json");
            let head_string = fs::read_to_string(head_path).unwrap();
            serde_json::from_str(head_string.as_str()).unwrap()
        })
        .collect();

    if heads.is_empty() {
        return None;
    }

    Some(heads)
}

pub fn get_template(template_name: &String) -> Result<Template, String> {
    let templates_struct = match get_templates_as_struct() {
        Some(t) => t,
        None => return Err("Repository was empty.".to_string())
    };

    let template_option = templates_struct
        .into_iter()
        .find(|temp| temp.name == *template_name);

    let template = match template_option {
        Some(t) => t,
        None => return Err(format!("Not is possible find \"{}\" on repository", template_name))
    };

    Ok(template)
}
