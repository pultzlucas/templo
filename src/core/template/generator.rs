use super::Template;
use crate::core::template::TempPath;
use crate::{
    core::utils::path::{format_path_namespace, pathbuf_to_string},
    paint, paintln,
};
use std::fs::File;
use std::{
    fs,
    io::{Error, Write},
    path::{Path, PathBuf},
};
use templo_engine::{EngineArg, Engine};

pub fn gen_template(
    template: Template,
    directory: &Path,
    temp_args: Vec<EngineArg>,
) -> Result<(), Error> {
    if !directory.exists() {
        fs::create_dir_all(directory)?;
    }

    paintln!("{gray}", "[creating files and folders...]");
    for path in template.paths.into_iter() {
        create_path(path, directory, &temp_args)?;
    }

    print!("\n");
    Ok(())
}

fn create_path(
    path: TempPath,
    directory: &Path,
    temp_args: &Vec<EngineArg>,
) -> Result<(), Error> {
    let real_path = TempPath {
        path: get_real_path(directory, path.path),
        is_file: path.is_file,
        content: path.content,
    };

    if real_path.is_file {
        let mut file = File::create(&real_path.path)?;

        if let Some(content) = real_path.content {
            let bytes_decoded =
                base64::decode(content.bytes).expect("Error when decoding base64 file bytes.");

            // If content is text, it will be parsed by template engine
            if content.is_text {
                let text_decoded = String::from_utf8(bytes_decoded.clone())
                    .expect("Error when parsing template content bytes to utf8.");
                let engine = Engine::new(temp_args.clone());
                let text_content_parsed = engine.compile(text_decoded)?.as_bytes().to_vec();
                file.write_all(&text_content_parsed)?;
            } else {
                file.write_all(&bytes_decoded)?;
            }
        }

        paint!("{gray}", "file: ");
    } else {
        fs::create_dir(&real_path.path)?;
        paint!("{gray}", "dir:  ");
    }

    let path_string = pathbuf_to_string(format_path_namespace(real_path.path));
    println!("{}", path_string);

    Ok(())
}

fn get_real_path(directory: &Path, path: PathBuf) -> PathBuf {
    Path::new(directory)
        .join(pathbuf_to_string(path))
        .to_path_buf()
}
