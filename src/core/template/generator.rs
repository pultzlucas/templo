use super::Template;
use crate::core::template::{TempContent, TempPath, TempPathType};
use crate::{
    utils::path::{format_path_namespace, pathbuf_to_string, str_to_pathbuf},
    paint, paintln,
};
use crate::utils::string::decode_base64;
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
    let real_path = TempPath {
        buf: get_real_path(directory, path.buf),
        path_type: path.path_type,
    };

    if real_path.path_type == TempPathType::File {
        fs::write(&real_path.buf, "")?;
        paint!("{gray}", "file: ");
    }
    if real_path.path_type == TempPathType::Dir {
        fs::create_dir(&real_path.buf)?;
        paint!("{gray}", "dir: ");
    }

    println!(
        "{}",
        pathbuf_to_string(format_path_namespace(real_path.buf))
    );

    Ok(())
}

fn write_contents(contents: Vec<TempContent>, directory: &Path) -> Result<(), Error> {
    for content in contents.into_iter() {
        let file_path = get_real_path(directory, str_to_pathbuf(&content.file_path));
        if file_path.exists() {
            let text = decode_base64(content.text)?;
            fs::write(&file_path, text)?;

            print!("{}", pathbuf_to_string(format_path_namespace(file_path)));
            paintln!("...{green}", "ok");
        }
    }

    print!("\n");
    Ok(())
}

fn get_real_path(directory: &Path, path: PathBuf) -> PathBuf {
    Path::new(directory)
        .join(pathbuf_to_string(path))
        .to_path_buf()
}
