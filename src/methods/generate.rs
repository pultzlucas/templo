use crate::cli::input::command::{Command, CommandOption};
use crate::cli::input::namespaces::get_repo_namespace_obj;
use crate::cli::output::messages::error::{INVALID_DIRECTORY_PATH_TYPE, INVALID_TEMPLATE_NAME};
use crate::core::repos::Repository;
use crate::core::template::engine::{get_engine_args_input, set_arg_default_value, TempEngineArg};
use crate::core::template::{generator, Template};
use crate::core::utils::errors::{invalid_input_error, std_error};
use crate::write_help;
use serde_json::from_str;
use std::{fs, io::Error, path::Path, time::Instant};

pub struct Generate;

impl Generate {
    pub fn help() {
        write_help!("../../help_files/generate.json");
    }

    pub async fn run(command: Command) -> Result<(), Error> {
        if command.has_help_flag() {
            Self::help();
            return Ok(());
        }

        if command.has_option("file") {
            return gen_from_template_file(command.clone());
        }

        if command.args.len() < 1 {
            return Err(invalid_input_error(INVALID_TEMPLATE_NAME));
        }

        let start = Instant::now(); // start timing process

        let directory = if command.args.len() < 2 {
            Path::new(".")
        } else {
            Path::new(&command.args[1])
        };

        if directory.extension() != None {
            return Err(invalid_input_error(INVALID_DIRECTORY_PATH_TYPE));
        }

        let template_namespace = get_repo_namespace_obj(&command.args[0]);
        let repo = Repository::connect(template_namespace.repo_name)?;
        let template = repo.get_template(&template_namespace.template_name)?;

        let temp_args = if !command.options.is_empty() {
            get_template_args_by_options(command.options, &template)?
        } else {
            get_template_args_by_temp(&template)?
        };

        generator::gen_template(template, directory, temp_args)?;
        println!(
            "Template \"{}\" was generated.",
            template_namespace.template_name
        );

        let end = Instant::now(); // stop timing process
        println!("Done in {:.2?}", end.duration_since(start));
        Ok(())
    }
}

fn gen_from_template_file(command: Command) -> Result<(), Error> {
    let start = Instant::now();

    let tpo_filename = &command.get_opt_by_name("file").unwrap().value;

    let directory = if command.args.len() < 2 {
        Path::new(".")
    } else {
        Path::new(&command.args[1])
    };

    if directory.extension().is_some() {
        return Err(invalid_input_error(INVALID_DIRECTORY_PATH_TYPE));
    }

    if !tpo_filename.ends_with(".tpo") {
        return Err(invalid_input_error("The file must have TPO extension."));
    }

    let template_string = fs::read_to_string(tpo_filename)?;
    let template: Template = std_error(from_str(&template_string))?;

    let temp_args = if !command.options.is_empty() {
        get_template_args_by_options(command.options, &template)?
    } else {
        get_template_args_by_temp(&template)?
    };

    generator::gen_template(template.clone(), directory, temp_args)?;
    println!("Template \"{}\" was generated.", template.name);

    let end = Instant::now(); // stop timing process
    println!("Done in {:.2?}", end.duration_since(start));
    Ok(())
}

fn get_template_args_by_options(
    options: Vec<CommandOption>,
    template: &Template,
) -> Result<Vec<TempEngineArg>, Error> {
    options
        .into_iter()
        .map(|option| TempEngineArg {
            key: option.name,
            value: option.value,
        })
        .map(|engine_arg| {
            if let Some(config_args) = &template.args {
                set_arg_default_value(engine_arg, config_args)
            } else {
                Ok(engine_arg)
            }
        })
        .collect()
}

fn get_template_args_by_temp(template: &Template) -> Result<Vec<TempEngineArg>, Error> {
    if let Some(config_args) = &template.args {
        let temp_args = get_engine_args_input(config_args)?
            .into_iter()
            .map(|arg| set_arg_default_value(arg, config_args));

        return temp_args.collect();
    }

    Ok(vec![])
}
