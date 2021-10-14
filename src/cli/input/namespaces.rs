use crate::core::utils::string::split_by;
use serde_derive::{Deserialize, Serialize};
use tabled::Tabled;

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
