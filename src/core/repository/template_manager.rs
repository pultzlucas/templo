use crate::core::path::ProtternFileSystem;
use crate::utils::{paths::TEMPLATES_PATH, structs::Template};
use std::{fs, path::Path, io::{Error, ErrorKind}};

pub struct TemplateManager {}

impl TemplateManager {
    pub fn save_template(head: String, template_name: String) -> Result<(), Error> {
        let template_path = ProtternFileSystem::get_dir_path(&template_name);
        match fs::create_dir(&template_path) {
            Ok(o) => o,
            Err(e) => return Err(e),
        }
    
        let template_path = template_path
            .join("HEAD.json")
            .into_os_string()
            .into_string()
            .unwrap();
            
        fs::write(template_path, head)
    }

    pub fn get_template(template_name: &String) -> Result<Template, String> {
        let templates_struct = match TemplateManager::get_all_templates() {
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

    pub fn get_all_templates() -> Option<Vec<Template>> {
        let dir_names = fs::read_dir(TEMPLATES_PATH)
            .unwrap()
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, Error>>()
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