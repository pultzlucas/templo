use crate::utils::errors::{invalid_input_error, std_error};
use crate::utils::string::split_by;
use regex::Regex;
use serde_derive::{Deserialize, Serialize};
use std::io::Error;
use tabled::Tabled;

use super::repos::remote_repos_reg::get_reg;

#[derive(Serialize, Deserialize, Tabled, Debug, Clone)]
pub struct RemoteRepoNamespace {
    pub name: String,
    pub base_url: String,
    pub requires_authorization: bool,
}

#[derive(Debug)]
pub struct NamespaceObject {
    pub repo_name: String,
    pub template_name: String,
}

pub fn parse_namespace_to_raw_url(route: String) -> Result<String, Error> {
    let regex = std_error(Regex::new(r"^[\w-]+"))?;
    let namespace_name = regex.find(&route);

    if let Some(namespace_name) = namespace_name {
        let namespace = get_reg(namespace_name.as_str())?;
        let raw_url = regex.replace(&route, namespace.base_url).to_string();
        return Ok(raw_url);
    }

    Err(invalid_input_error("Invalid namespace syntax."))
}

pub fn get_repo_namespace_obj(namespace: &str) -> NamespaceObject {
    let ns_splitted = split_namespace_str(namespace);
    if ns_splitted.len() == 1 {
        return NamespaceObject {
            repo_name: "main".to_string(),
            template_name: namespace.to_string(),
        };
    }

    NamespaceObject {
        repo_name: ns_splitted[0].clone(),
        template_name: ns_splitted[1].clone(),
    }
}

pub fn split_namespace_str(ns_string: &str) -> Vec<String> {
    split_by(ns_string, "/")
}

