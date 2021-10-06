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
        }

        #[derive(Serialize, Deserialize, Debug)]
        struct HelpArgs {
            inputs: Option<Vec<HelpArgsInput>>,
            flags: Option<Vec<HelpArgsFlag>>,
            options: Option<Vec<HelpArgsOption>>,
        }

        #[derive(Serialize, Deserialize, Debug)]
        struct Help {
            name: String,
            about: String,
            usage: String,
            parents: Option<Vec<String>>,
            args: HelpArgs,
        }

        let help_content = include_str!($src);
        let help: Help = serde_json::from_str(help_content)
            .expect(&format!("Error when parsing \"{}\" help file.", $src));

        use crate::paint_string;

        // NAME
        let mut full_name = String::new();
        if let Some(parents) = help.parents {
            for parent in parents {
                if !parent.is_empty() {
                    full_name.push_str(&format!("{} ", parent));
                }
            }
        }
        full_name.push_str(&paint_string!("{yellow}", help.name));
        println!("{}", full_name);
        
        // ABOUT
        println!("{}", help.about);
        
        // USAGE
        print!("\n");
        println!("USAGE:");
        println!("{}", full_name);
        print!("    ");
        println!("{}", help.usage);

        // FLAGS
        if let Some(flags) = help.args.flags {
            print!("\n");
            println!("FLAGS:");

            for flag in flags {
                print!("    ");
                if let Some(short) = flag.short {
                    print!("-{}, ", short);
                }

                print!("--{}", flag.long);
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
                if let Some(short) = opt.short {
                    print!("-{}, ", short);
                }

                print!("--{}", opt.long);
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
                print!("{} ", input.name);

                if let Some(required) = input.required {
                    if required && input.default_value.is_some() {
                        panic!("An input can not be required and have a default value.");
                    }

                    if required {
                        print!("[required]");
                    }
                }

                if let Some(default_value) = input.default_value {
                    print!("[default: {}]", default_value);
                }

                print!("\t");
                println!("{}", input.help);
            }
        }

        //println!("{:?}", help);
    };
}
