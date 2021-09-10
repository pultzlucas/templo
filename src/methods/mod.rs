use tabled::Tabled;

#[derive(Tabled)]
pub struct MethodInfo {
    method: &'static str,
    description: &'static str,
}

pub const METHODS: [MethodInfo; 9] = [
    MethodInfo {
        method: "save",
        description: "Saves a template from directory."
    },
    MethodInfo {
        method: "gen",
        description: "Generates a project from template."
    },
    MethodInfo {
        method: "del",
        description: "Deletes a template."
    },
    MethodInfo {
        method: "repo",
        description: "Returns all templates existents on repository."
    },
    MethodInfo {
        method: "update",
        description: "Updates a template."
    },
    MethodInfo {
        method: "desc",
        description: "Describes a template."
    },
    MethodInfo {
        method: "get",
        description: "Installes a public template."
    },
    MethodInfo {
        method: "docs",
        description: "Open the Prottern documentation."
    },
    MethodInfo {
        method: "version/v",
        description: "Shows the prottern version installed."
    }
];

pub mod generate;
pub mod delete;
pub mod describe;
pub mod get;
pub mod help;
pub mod welcome;
pub mod save;
pub mod repo;
pub mod version;
pub mod docs;
pub mod registry;
pub mod update;
