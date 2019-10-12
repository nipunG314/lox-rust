use std::env;
use std::fs;
use std::io::ErrorKind;

mod core;
mod error;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => core::run_prompt(),
        2 => match fs::read_to_string(&args[1]) {
            Ok(source) => core::run_file(source),
            Err(err) => match err.kind() {
                ErrorKind::NotFound => println!("{}", String::from("File not found.")),
                ErrorKind::PermissionDenied => println!("{}", String::from("Permission denied.")),
                _ => println!("There was a problem processing the file: {}", err),
            },
        },
        _ => {
            println!("Usage: lox [script]");
        }
    };
}
