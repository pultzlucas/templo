use std::io::Error;
use crate::cli::input::args::Args;
use crate::core::config;
use crate::core::path::get_config_path;
use crate::utils::path::pathbuf_to_string;

pub fn run(args: Args) -> Result<(), Error> {
    config::create_files()?;

    if args.has_flag("--local") {
        println!("{}", pathbuf_to_string(get_config_path()?));
        return Ok(())
    }

    Ok(())
}