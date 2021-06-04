extern crate serde_json;

use crate::utils::paths::TEMPLATES_PATH;
use crate::utils::structs::HEAD;
use std::{fs, io};

pub fn templates<'a>() -> Result<&'a str, String> {
    let dir_names = fs::read_dir(TEMPLATES_PATH)
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();

    let template_names: Vec<String> = dir_names
        .iter()
        .map(|dir| {
            let head_path = dir.join("HEAD.json");
            let head_string = fs::read_to_string(head_path).unwrap();
            serde_json::from_str(head_string.as_str()).unwrap()
        })
        .map(|head: HEAD| head.name)
        .collect();

    for name in template_names.iter() {
        println!("- {}", name);
    }

    Ok("")
}
