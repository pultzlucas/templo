#[macro_export]
macro_rules! write_help {
    ($src:expr) => {
        use clap::{SubCommand, load_yaml};

        let yml = load_yaml!($src);
        let method = SubCommand::from_yaml(yml);
        method.write_help(&mut std::io::stdout()).unwrap();
        print!("\n");
    };
}