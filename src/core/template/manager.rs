use super::Template;
use crate::core::template::{TempContent, TempPath, TempPathType};
use crate::{
    core::utils::path::{format_path_namespace, pathbuf_to_string},
    paint_string, paintln,
};
use std::{
    fs,
    io::Error,
    path::{Path, PathBuf},
};

pub fn gen_template(template: Template, directory: &Path) -> Result<(), Error> {
    paintln!("{gray}", "[creating files and folders...]");
    for path in template.paths.into_iter() {
        create_path(path, directory)?;
    }

    if template.contents.len() > 0 {
        print!("\n");
        paintln!("{gray}", "[writing contents...]");
        write_contents(template.contents, directory)?;
    }
    Ok(())
}

fn create_path(path: TempPath, directory: &Path) -> Result<(), Error> {
    let real_path = get_real_path(directory, path);

    if real_path.path_type == TempPathType::File {
        create_file(real_path.buf.clone())?;
    }
    if real_path.path_type == TempPathType::Dir {
        create_dir(real_path.buf)?;
    }

    Ok(())
}

fn write_contents(contents: Vec<TempContent>, directory: &Path) -> Result<(), Error> {
    for content in contents.into_iter() {
        let file_path = Path::new(directory).join(content.filename);
        if file_path.exists() {
            fs::write(&file_path, content.text)?;
            paintln!("{0}...{green}", pathbuf_to_string(file_path), "ok".to_string());
        }
    }

    print!("\n");
    Ok(())
}

fn get_real_path(directory: &Path, path: TempPath) -> TempPath {
    let buf = Path::new(directory)
        .join(pathbuf_to_string(path.buf))
        .to_path_buf();

    TempPath {
        buf: format_path_namespace(buf),
        path_type: path.path_type,
    }
}

fn create_file(path: PathBuf) -> Result<(), Error> {
    fs::write(&path, "")?;
    println!(
        "{} {}",
        paint_string!("{gray}", "file:"),
        pathbuf_to_string(path)
    );
    Ok(())
}

fn create_dir(path: PathBuf) -> Result<(), Error> {
    fs::create_dir(&path)?;
    println!(
        " {} {}",
        paint_string!("{gray}", "dir:"),
        pathbuf_to_string(path)
    );
    Ok(())
}
