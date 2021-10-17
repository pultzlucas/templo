use super::config::get_config_args;
use super::{miner, TempPath, Template};
use crate::core::utils::date::get_date_now_string;
use crate::core::utils::errors::invalid_input_error;
use crate::core::utils::path::{format_path_namespace, pathbuf_to_string, remove_dir_prefix};
use std::io::Error;

pub fn make_template(
    temp_name: String,
    ref_path: &str,
    description: Option<String>,
) -> Result<Template, Error> {
    if temp_name.contains(" ") {
        return Err(invalid_input_error(
            "The template name cannot have whitespaces.",
        ));
    }

    let created_at = get_date_now_string();
    let paths = make_template_paths(ref_path)?;
    let args = get_config_args(ref_path)?;

    Ok(Template {
        name: temp_name,
        description,
        created_at,
        updated_at: None,
        paths,
        args,
    })
}

pub fn make_template_paths(dir_path: &str) -> Result<Vec<TempPath>, Error> {
    let raw_paths = miner::mine_paths_from(dir_path)?;
    // let files = miner::mine_files_from_paths(raw_paths.clone(), dir_path)?
    //     .into_iter()
    //     .filter(|file| file.bytes != "")
    //     .collect();

    let formatted_paths: Vec<TempPath> = raw_paths
        .into_iter()
        .map(|path| TempPath {
            path: format_path_namespace(path.path),
            is_file: path.is_file,
            content: path.content,
        })
        .map(|path| {
            let path_clean =
                remove_dir_prefix(path.path, dir_path).expect("Error when removing dir prefix");
            TempPath {
                path: path_clean,
                is_file: path.is_file,
                content: path.content,
            }
        })
        .filter(|path| pathbuf_to_string(path.path.clone()) != "")
        .collect();

    Ok(formatted_paths)
}
