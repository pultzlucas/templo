use serde_json::from_str;

use crate::cli::input::command::{Command, CommandOption};
use crate::core::template::engine::{get_engine_args_input, set_arg_default_value, TempEngineArg};
use crate::core::template::{generator, Template};
use crate::utils::errors::{invalid_input_error, std_error};
use crate::{
    cli::output::messages::error::{INVALID_DIRECTORY_PATH_TYPE, INVALID_TEMPLATE_NAME},
    core::repo,
};
use std::{fs, io::Error, path::Path, time::Instant};

pub fn run(command: Command) -> Result<(), Error> {
    repo::create()?;

    if command.has_flag("-f") || command.has_flag("--file") {
        let tpo_filename = &command.args[0];
        let directory = if command.args.len() < 2 {
            Path::new(".")
        } else {
            Path::new(&command.args[1])
        };

        if directory.extension() != None {
            return Err(invalid_input_error(INVALID_DIRECTORY_PATH_TYPE));
        }

        if !directory.exists() {
            fs::create_dir_all(directory)?;
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

        generator::gen_template(template, directory, temp_args)?;

        return Ok(());
    }

    if command.args.len() < 1 {
        return Err(invalid_input_error(INVALID_TEMPLATE_NAME));
    }

    let template_name = &command.args[0];
    let directory = if command.args.len() < 2 {
        Path::new(".")
    } else {
        Path::new(&command.args[1])
    };

    if directory.extension() != None {
        return Err(invalid_input_error(INVALID_DIRECTORY_PATH_TYPE));
    }

    if !directory.exists() {
        fs::create_dir_all(directory)?;
    }

    let start = Instant::now(); // start timing process
    let template = repo::get_template(&template_name)?;

    let temp_args = if !command.options.is_empty() {
        get_template_args_by_options(command.options, &template)?
    } else {
        get_template_args_by_temp(&template)?
    };

    generator::gen_template(template, directory, temp_args)?;
    println!("Template \"{}\" was generated.", template_name);

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
