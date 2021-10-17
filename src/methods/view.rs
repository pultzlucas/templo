use crate::cli::input::check_flags;
use crate::cli::input::command::Command;
use crate::cli::input::namespaces::{get_repo_namespace_obj, NamespaceObject};
use crate::cli::output::messages::error::INVALID_TEMPLATE_NAME;
use crate::core::repos::Repository;
use crate::core::template::config::ConfigArg;
use crate::core::template::{TempPath, Template};
use crate::core::utils::errors::invalid_input_error;
use crate::core::utils::path::pathbuf_to_string;
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

        let expected_flags = vec![
            "--paths",
            "--created-at",
            "--updated-at",
            "--desc",
            "--args",
        ];
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

        if command.has_flag("--args") {
            if let Some(args) = template.args {
                display_template_args(args, false);
            }
            return Ok(());
        }

        paintln!("> {yellow}", &template.name);

        // Template description
        if let Some(description) = &template.description {
            println!("{}", description);
        }

        // Template file content
        if let Some(file) = command.get_opt_by_name("file") {
            return display_file_content(&file.value, template.clone());
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

        // Template config args
        if let Some(args) = template.args {
            paintln!("{gray}", "[ARGS]");
            display_template_args(args, true);
            print!("\n");
        }

        Ok(())
    }
}

fn display_template_paths(paths: Vec<TempPath>) {
    paths.iter().for_each(|path| {
        print!("    ");
        println!("{}", pathbuf_to_string(path.path.clone()))
    });
}

fn display_template_args(args: Vec<ConfigArg>, tab: bool) {
    args.iter().for_each(|arg| {
        if tab {
            print!("    ");
        }
        println!("{}", arg.key.to_uppercase());

        if let Some(about) = &arg.about {
            if tab {
                print!("    ");
            }
            println!("{}", about);
        }

        if tab {
            print!("    ");
        }
        println!("Query: '{}'", arg.query);

        if let Some(default) = &arg.default {
            if tab {
                print!("    ");
            }
            println!("Default: {}", default);
        }

        print!("\n")
    })
}

fn display_file_content(file_path: &str, template: Template) -> Result<(), Error> {
    print!("\n");
    let file_path_temp = template
        .paths
        .iter()
        .find(|path| file_path == path.path.to_str().unwrap());

    if let Some(file_path) = file_path_temp {
        let path_name = pathbuf_to_string(file_path.path.clone());

        if !file_path.is_file {
            return Err(invalid_input_error(&format!(
                "Path \"{}\" is not a file.",
                path_name
            )));
        }

        let temp_files = template.files();
        let file = temp_files
            .into_iter()
            .find(|file| pathbuf_to_string((*file.path).to_path_buf()) == path_name);

        println!("{}", paint_string!("{gray}", format!("[{}]", path_name)));

        if let Some(file) = file {
            let content = file.content.unwrap();
            let bytes_decoded =
                base64::decode(content.bytes).expect("Error when decoding base64 file bytes.");

            if content.is_text {
                let text = String::from_utf8(bytes_decoded)
                    .expect("Error when parsing bytes to utf8 string.");
                println!("{}", text);
            } else {
                println!("{:?}", bytes_decoded);
            }
        }
    } else {
        return Err(invalid_input_error(&format!(
            "Path \"{}\" not exists in \"{}\" template.",
            file_path, &template.name
        )));
    }

    Ok(())
}
