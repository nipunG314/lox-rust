mod core;
pub mod error;
pub mod parser;
pub mod scanner;

use crate::parser::Parser;
use crate::scanner::Scanner;
use std::env;
use std::fs;
use std::io::ErrorKind;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => run_prompt(),
        2 => match fs::read_to_string(&args[1]) {
            Ok(source) => run_file(source),
            Err(err) => match err.kind() {
                ErrorKind::NotFound => println!("Couldn't find: {}", args[1]),
                ErrorKind::PermissionDenied => println!("Permission denied."),
                _ => println!("There was a problem processing the file: {}", err),
            },
        },
        _ => {
            println!("Usage: lox [script]");
        }
    };
}

fn run_prompt() {
    println!("Implement REPL");
}

fn run_file(source: String) {
    let mut scanner = Scanner::new(&source);
    scanner.scan_tokens();

    let mut parser = Parser::new(scanner.get_tokens());
    match parser.parse() {
        Ok(expr) => println!("{}", expr),
        Err(_) => println!("ParseError found"),
    };
}
