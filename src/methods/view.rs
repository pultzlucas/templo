use crate::cli::input::check_flags;
use crate::cli::input::command::Command;
use crate::cli::output::messages::error::INVALID_TEMPLATE_NAME;
use crate::core::namespaces::{get_repo_namespace_obj, NamespaceObject};
use crate::core::repos::Repository;
use crate::core::template::{TempPath, TempPathType, Template};
use crate::utils::errors::invalid_input_error;
use crate::utils::path::pathbuf_to_string;
use crate::utils::string::decode_base64;
use crate::{paint_string, paintln, write_help};
use std::io::Error;

pub struct View;

impl View {
    pub fn help() {
        write_help!("../../help_files/view.json");
    }

    pub fn run(command: Command) -> Result<(), Error> {
        if command.has_help_flag() {
            Self::help();
            return Ok(());
        }

        let expected_flags = vec!["--paths", "--created-at", "--updated-at", "--desc"];
        check_flags(&command.flags, expected_flags)?;

        if command.args.is_empty() {
            return Err(invalid_input_error(INVALID_TEMPLATE_NAME));
        }

        // Get template from repository
        let NamespaceObject {
            repo_name,
            template_name,
        } = get_repo_namespace_obj(&command.args[0]);
        let repo = Repository::connect(repo_name)?;
        let template = repo.get_template(&template_name)?;

        if command.has_flag("--paths") {
            display_template_paths(template.paths);
            return Ok(());
        }

        if command.has_flag("--created-at") {
            println!("{}", template.created_at);
            return Ok(());
        }

        if command.has_flag("--updated-at") {
            if let Some(updated_at) = template.updated_at {
                println!("{}", updated_at);
            } else {
                println!("None");
            }
            return Ok(());
        }

        if command.has_flag("--desc") {
            if let Some(description) = &template.description {
                println!("{}", description);
            }
            return Ok(());
        }

        paintln!("> {yellow}", &template.name);

        // Template description
        if let Some(description) = &template.description {
            println!("{}", description);
        }

        // Template file content
        if command.args.len() > 1 {
            return display_file_content(&command.args[1..], template.clone());
        }

        print!("\n");

        // Template creation date
        paintln!("{gray}", "[CREATED AT]");
        print!("    ");
        println!("{}", template.created_at);
        print!("\n");

        // Template update date
        if let Some(updated_at) = template.updated_at {
            paintln!("{gray}", "[UPDATED AT]");
            print!("    ");
            println!("{}", updated_at);
            print!("\n");
        }

        // Template paths
        paintln!("{gray}", "[PATHS]");
        display_template_paths(template.paths);
        print!("\n");

        Ok(())
    }
}

fn display_template_paths(paths: Vec<TempPath>) {
    paths
        .iter()
        .for_each(|path| {
            print!("    ");
            println!("{}", pathbuf_to_string(path.path.clone()))
        });
}

fn display_file_content(file_paths: &[String], template: Template) -> Result<(), Error> {
    for file_path in file_paths {
        let file_path_temp = template
            .paths
            .iter()
            .find(|path| file_path == path.path.to_str().unwrap());

        if let Some(file_path) = file_path_temp {
            let path_name = pathbuf_to_string(file_path.path.clone());

            if file_path.path_type == TempPathType::Dir {
                return Err(invalid_input_error(&format!(
                    "Path \"{}\" is not a file.",
                    path_name
                )));
            }

            let file_content = template
                .contents
                .iter()
                .find(|content| content.file_path == path_name);

            println!("{}", paint_string!("{gray}", format!("[{}]", path_name)));

            if let Some(content) = file_content {
                let text = decode_base64(content.text.clone())?;
                println!("{}", text);
            } else {
                println!("No content.");
            }
        } else {
            return Err(invalid_input_error(&format!(
                "Path \"{}\" not exists in \"{}\" template.",
                file_path, &template.name
            )));
        }
    }

    Ok(())
}
