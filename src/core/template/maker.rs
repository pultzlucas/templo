use crate::core::template::{miner, TempMetadata, Template, TemplateType};
use crate::core::user_account::get_user_account_data;
use crate::core::utils::date::get_date_now_string;
use std::io::Error;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
struct TempData {
    name: String,
    paths: Vec<PathBuf>,
    contents: Vec<miner::File>,
}

pub fn create_template(temp_name: String, dir_path: String) -> Result<Template, Error> {
    let metadata = get_template_metadata()?;
    let data = get_template_data(temp_name, dir_path)?;
    Ok(Template {
        metadata,
        name: data.name,
        paths: data.paths,
        contents: data.contents,
    })
}

fn get_template_data(temp_name: String, dir_path: String) -> Result<TempData, Error> {
    let paths = miner::extract_paths_from(dir_path.as_str())?;
    let files = miner::extract_files_from_paths(paths.clone())
        .into_iter()
        .filter(|file| file.content != "")
        .collect();
    Ok(TempData {
        name: temp_name,
        paths,
        contents: files,
    })
}

fn get_template_metadata() -> Result<TempMetadata, Error> {
    let owner = get_user_account_data()?.username;
    let date_now = get_date_now_string();
    Ok(TempMetadata {
        owner,
        created_at: date_now,
        template_type: TemplateType::Local,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn it_should_return_a_valid_template_data() {
        let template = get_template_data(
            "temp-name".to_string(),
            "./src/core/tests/tree_files_only".to_string(),
        )
        .unwrap();

        assert_eq!(
            template,
            TempData {
                name: "temp-name".to_string(),
                paths: vec![
                    Path::new("./src/core/tests/tree_files_only").to_path_buf(),
                    Path::new("./src/core/tests/tree_files_only/.file4").to_path_buf(),
                    Path::new("./src/core/tests/tree_files_only/file-2").to_path_buf(),
                    Path::new("./src/core/tests/tree_files_only/file1").to_path_buf(),
                    Path::new("./src/core/tests/tree_files_only/file_3").to_path_buf(),
                    Path::new("./src/core/tests/tree_files_only/file_text.txt").to_path_buf()
                ],
                contents: vec![crate::core::template::miner::File {
                    filename: "./src/core/tests/tree_files_only/file_text.txt".to_string(),
                    content: "Lorem ipsum dolor\r\n\r\n123123123123\r\n\r\n{(-@#$%¨¨&*)}"
                        .to_string()
                }]
            }
        )
    }
}
