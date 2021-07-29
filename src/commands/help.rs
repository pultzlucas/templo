use super::COMMANDS;
use crate::core::info::{REPOSITORY, VERSION};
use std::io::Error;
use tabled::{Alignment, Column, Modify, Row, Style, Table, Tabled};

#[derive(Tabled)]
struct Repository {
    repository: &'static str,
}

#[derive(Tabled)]
struct Version {
    version: &'static str,
}

pub fn help() -> Result<(), Error> {
    //println!("Version:\n{}v\n", VERSION);
    //println!("Respository:\n{}\n", REPOSITORY);
    print!(
        "{}",
        Table::new(&[Version { version: VERSION }]).with(Style::pseudo())
    );

    print!(
        "{}\n",
        Table::new(&[Repository {
            repository: REPOSITORY
        }])
        .with(Style::pseudo())
    );
    print_commands();

    Ok(())
}

fn print_commands() {
    let commands_tb = Table::new(&COMMANDS)
        .with(Modify::new(Column(1..)).with(Alignment::left()))
        .with(Modify::new(Column(..1)).with(Alignment::right()))
        .with(Modify::new(Row(..1)).with(Alignment::center_horizontal()))
        .with(Style::psql());

    print!("{}", commands_tb);
    /* for command_info in COMMANDS.iter() {
        println!("{:>20}\t{}", command_info.name, command_info.description);
    } */
}
