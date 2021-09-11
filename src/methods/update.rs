use crate::cli::input::args::Args;
use crate::cli::output::messages::error::{
    INVALID_DIRECTORY_PATH_NAME, INVALID_DIRECTORY_PATH_TYPE, INVALID_TEMPLATE_NAME,
};
use crate::core::repo;
use crate::core::template::maker::make_template;
use crate::methods::check_flags;
use crate::utils::errors::invalid_input_error;
use crate::utils::path::str_to_pathbuf;
use std::fs;
use std::io::Error;
use std::time::Instant;

pub fn run(args: Args) -> Result<(), Error> {
    repo::create()?;

    let flags = vec!["--name"];
    check_flags(&args.flags, flags)?;

    if args.has_flag("--name") {
        if args.inputs.len() < 1 {
            return Err(invalid_input_error(
                "Current template name must be specified.",
            ));
        }

        if args.inputs.len() < 2 {
            return Err(invalid_input_error("New template name must be specified."));
        }

        let old_template_name = &args.inputs[0];
        let new_template_name = &args.inputs[1];
        repo::update_template_name(old_template_name, new_template_name)?;

        println!(
            "Template \"{}\" name was changed to \"{}\".",
            old_template_name, new_template_name
        );

        return Ok(());
    }

    if args.inputs.len() < 1 {
        return Err(invalid_input_error(INVALID_TEMPLATE_NAME));
    }

    if args.inputs.len() < 2 {
        return Err(invalid_input_error(INVALID_DIRECTORY_PATH_NAME));
    }

    let template_name = args.inputs[0].clone();
    let directory = &args.inputs[1];
    let directory_pathbuf = str_to_pathbuf(directory);

    if directory_pathbuf.extension() != None {
        return Err(invalid_input_error(INVALID_DIRECTORY_PATH_TYPE));
    }

    if !directory_pathbuf.exists() {
        fs::create_dir_all(directory_pathbuf)?;
    }

    let start = Instant::now(); // start timing process

    let new_template = make_template(template_name.clone(), directory)?;
    repo::update_template_content(template_name.clone(), new_template)?;

    println!("Template \"{}\" was updated.", template_name);

    let end = Instant::now(); // stop timing process
    println!("Done in {:.2?}", end.duration_since(start));

    Ok(())
}
