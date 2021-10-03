use crate::utils::errors::invalid_input_error;
use std::io::Error;
use tabled::Tabled;

pub fn check_flags(flags: &Vec<String>, expected_flags: Vec<&str>) -> Result<(), Error> {
    let invalid_flag = flags
        .into_iter()
        .find(|flag| !expected_flags.contains(&flag.as_str()));

    if let Some(invalid_flag) = invalid_flag {
        return Err(invalid_input_error(&format!(
            "Invalid flag \"{}\"",
            invalid_flag
        )));
    }

    Ok(())
}

#[derive(Tabled)]
pub struct MethodInfo {
    method: &'static str,
    description: &'static str,
}

pub const METHODS: [MethodInfo; 10] = [
    MethodInfo {
        method: "save",
        description: "Saves a template from directory.",
    },
    MethodInfo {
        method: "gen",
        description: "Generates a project from template.",
    },
    MethodInfo {
        method: "del",
        description: "Deletes a template.",
    },
    MethodInfo {
        method: "repo",
        description: "Returns all templates existents on repository.",
    },
    MethodInfo {
        method: "update",
        description: "Updates a template.",
    },
    MethodInfo {
        method: "desc",
        description: "Describes a template.",
    },
    MethodInfo {
        method: "get",
        description: "Installes a public template.",
    },
    MethodInfo {
        method: "docs",
        description: "Open the Prottern documentation.",
    },
    MethodInfo {
        method: "namespace",
        description: "Manage remote repos namespaces.",
    },
    MethodInfo {
        method: "version/v",
        description: "Shows the prottern version installed.",
    },
];

pub mod delete;
pub mod describe;
pub mod docs;
pub mod generate;
pub mod get;
pub mod help;
pub mod namespace;
pub mod repo;
pub mod repos;
pub mod save;
pub mod update;
pub mod version;
pub mod r#move;
pub mod welcome;
