use super::Template;
use crate::core::template::TempContent;
use crate::{core::utils::path::pathbuf_to_string, paint_string};
use std::{
    fs,
    io::Error,
    path::{Path, PathBuf},
};

pub fn gen_template(template: Template, directory: &Path) -> Result<(), Error> {
    for path in template.paths.into_iter() {
        create_path(path, directory)?;
    }

    if template.contents.len() > 0 {
        write_contents(template.contents, directory)?;
    }
    Ok(())
}

fn create_path(path: PathBuf, directory: &Path) -> Result<(), Error> {
    let real_path = get_real_path(directory, path);

    if real_path.is_file() {
        create_file(&real_path)?;
    }
    if real_path.is_dir() {
        create_dir(&real_path)?;
    }

    Ok(())
}

fn write_contents(contents: Vec<TempContent>, directory: &Path) -> Result<(), Error> {
    for content in contents.into_iter() {
        let file_path = Path::new(directory).join(content.filename);
        if file_path.exists() {
            fs::write(file_path, content.text)?;
        }
    }

    Ok(())
}

fn get_real_path(directory: &Path, path: PathBuf) -> PathBuf {
    Path::new(directory)
        .join(pathbuf_to_string(path))
        .to_path_buf()
}

fn create_file(path: &PathBuf) -> Result<(), Error> {
    fs::write(path, "")?;
    println!("{} {:?}", paint_string!("{gray}", "file:"), path);
    Ok(())
}

fn create_dir(path: &PathBuf) -> Result<(), Error> {
    fs::create_dir(path)?;
    println!(" {} {:?}", paint_string!("{gray}", "dir:"), path);
    Ok(())
}
