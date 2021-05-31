use std::env;

fn main() {
    let env: Vec<String> = env::args().collect();
    let args = &env[1..];

    println!("args: {:?}", args)
}
