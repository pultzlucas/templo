// {
//     "name": "method-name",
//     "about": "Method description",
//     "args": {
//         "flags": {},
//         "options": null,
//         "inputs": [
//             {
//                 "name": "template-namespace",
//                 "help": "Sets the template namespace",
//                 "required": true
//             }
//         ]
//     }
// }

#[macro_export]
macro_rules! write_help {
    ($src:expr) => {
        use serde_derive::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize, Debug)]
        struct HelpArgsSubmethod {
            name: String,
            help: String,
        }

        #[derive(Serialize, Deserialize, Debug)]
        struct HelpArgsInput {
            name: String,
            help: String,
            required: Option<bool>,
            default_value: Option<String>,
        }

        #[derive(Serialize, Deserialize, Debug)]
        struct HelpArgsFlag {
            short: Option<String>,
            long: String,
            help: String,
        }

        #[derive(Serialize, Deserialize, Debug)]
        struct HelpArgsOption {
            short: Option<String>,
            long: String,
            help: String,
            default_value: Option<String>,
            value_name: Option<String>,
        }

        #[derive(Serialize, Deserialize, Debug)]
        struct HelpArgs {
            inputs: Option<Vec<HelpArgsInput>>,
            flags: Option<Vec<HelpArgsFlag>>,
            options: Option<Vec<HelpArgsOption>>,
            submethods: Option<Vec<HelpArgsSubmethod>>,
        }

        #[derive(Serialize, Deserialize, Debug)]
        struct Help {
            name: String,
            about: String,
            parents: Option<Vec<String>>,
            args: HelpArgs,
        }

        let help_content = include_str!($src);
        let help: Help = serde_json::from_str(help_content)
            .expect(&format!("Error when parsing \"{}\" help file.", $src));

        use crate::paint_string;

        // NAME
        let parents_str = if let Some(parents) = help.parents {
            parents.join(" ")
        } else {
            "".to_string()
        };
        println!(
            "> {} {}",
            parents_str,
            paint_string!("{yellow}", &help.name)
        );

        // ABOUT
        println!("{}", help.about);

        // USAGE
        print!("\n");
        println!("USAGE:");
        print!("    ");
        print!("{} {}", parents_str, help.name);

        if help.args.flags.is_some() {
            print!(" [OPTIONS]");
        }

        if help.args.flags.is_some() {
            print!(" [FLAGS]");
        }

        if let Some(inputs) = &help.args.inputs {
            for input in inputs.iter() {
                print!(" <{}>", input.name);
            }
        }

        print!("\n");

        // FLAGS
        if let Some(flags) = help.args.flags {
            print!("\n");
            println!("FLAGS:");

            for flag in flags {
                print!("    ");
                if let Some(short) = flag.short {
                    print!("{:<20}", format!("-{}, --{}", short, flag.long));
                } else {
                    print!("    {:<20}", format!("--{}", flag.long));
                }

                print!("\t");
                println!("{}", flag.help);
            }
        }

        // OPTIONS
        if let Some(options) = help.args.options {
            print!("\n");
            println!("OPTIONS:");
            for opt in options {
                print!("    ");

                let value_name = if let Some(value_name) = opt.value_name {
                    value_name
                } else {
                    "value".to_string()
                };

                if let Some(short) = opt.short {
                    print!(
                        "{:<22}",
                        format!("-{}, --{}=<{}>", short, opt.long, value_name)
                    );
                } else {
                    print!("    {:<22}", format!("--{}=<{}>", opt.long, value_name));
                }

                print!("\t");
                println!("{}", opt.help);
            }
        }

        // INPUTS
        if let Some(inputs) = help.args.inputs {
            print!("\n");
            println!("INPUTS:");
            for input in inputs {
                print!("    ");

                if let Some(required) = input.required {
                    if required && input.default_value.is_some() {
                        panic!("An input can not be required and have a default value.");
                    }

                    if required {
                        print!("{:<20}", format!("{} [required]", input.name));
                    }
                }

                if let Some(default_value) = &input.default_value {
                    print!(
                        "{:<20}",
                        format!("{} [default: {}]", input.name, default_value)
                    );
                }

                if input.required.is_none() && input.default_value.is_none() {
                    print!("{:<20}", input.name)
                }

                print!("\t");
                println!("{}", input.help);
            }
        }

        // SUBMETHODS
        if let Some(submethods) = help.args.submethods {
            print!("\n");
            println!("SUBMETHODS:");

            for submethod in submethods.iter() {
                print!("    ");
                print!("{:<20}", submethod.name);
                print!("\t");
                println!("{}", submethod.help)
            }
        }
    };
}
