use crate::cli::input::command::Command;
use crate::paintln;
use open;
use std::io::Error;
use std::time::Instant;

const DOCS_URL: &'static str = "https://github.com/Lucas-Lopes-Pultz/prottern-docs";

pub struct Docs;

impl Docs {
    pub fn run(_: Command) -> Result<(), Error> {
        let start = Instant::now(); // start timing process
        paintln!("{gray}", "[opening browser]");
    
        open::that(DOCS_URL)?;
    
        let end = Instant::now(); // stop timing process
        println!("Done in {:.2?}", end.duration_since(start));
        println!("Look at your browser!");
    
        Ok(())
    }
}

