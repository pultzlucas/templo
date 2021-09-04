use tabled::Tabled;

#[derive(Tabled)]
pub struct CommandInfo {
    command: &'static str,
    description: &'static str,
}

pub const COMMANDS: [CommandInfo; 8] = [
    CommandInfo {
        command: "save",
        description: "Saves a template from directory."
    },
    CommandInfo {
        command: "gen",
        description: "Generates a project from template."
    },
    CommandInfo {
        command: "del",
        description: "Deletes a template."
    },
    CommandInfo {
        command: "repo",
        description: "Returns all templates existents on repository."
    },
    CommandInfo {
        command: "desc",
        description: "Describes a template."
    },
    CommandInfo {
        command: "get",
        description: "Installes a public template."
    },
    CommandInfo {
        command: "docs",
        description: "Open the Prottern documentation."
    },
    CommandInfo {
        command: "version/v",
        description: "Shows the prottern version installed."
    }
];

/* use std::io::Error;
pub trait Command {
    fn info() -> CommandInfo;
    fn exec(args: &[String]) -> Result<(), Error>;
} */

pub mod generate;
pub mod delete;
pub mod describe;
pub mod get;
pub mod help;
pub mod prottern;
pub mod save;
pub mod repository;
pub mod version;
pub mod documentation;
