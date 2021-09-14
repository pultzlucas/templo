use serde_json::from_str;

use crate::cli::input::command::Command;
use crate::core::template::{Template, generator};
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
            return Err(invalid_input_error("The file must have TPO extension."))
        }

        let template_string = fs::read_to_string(tpo_filename)?;
        let template: Template = std_error(from_str(&template_string))?;

        generator::gen_template(template, directory)?;

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

    generator::gen_template(template, directory)?;
    println!("Template \"{}\" was generated.", template_name);

    let end = Instant::now(); // stop timing process
    println!("Done in {:.2?}", end.duration_since(start));

    Ok(())
}