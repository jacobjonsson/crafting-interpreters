use std::io::{stdin, Read};
use std::{fs::File, process::exit};

use scanner::Scanner;
use token_kind::TokenKind;

mod scanner;
mod token;
mod token_kind;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 2 {
        println!("Usage: jlox [script]");
        exit(64);
    } else if args.len() == 2 {
        run_file(args.get(1).unwrap())
    } else {
        run_prompt();
    }
}

fn run_file(path: &str) {
    let mut file = File::open(path).expect("Failed to read file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read to string");
    run(&contents);
}

fn run_prompt() {
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        if &input == "" {
            break;
        }
        run(&input);
    }
}

fn run(source: &str) {
    #![allow(unused)]
    let mut has_error = false;
    let mut scanner = Scanner::new(source);
    let mut tokens = Vec::new();

    loop {
        match scanner.scan_token() {
            Ok(token) => {
                println!("{:?}", token);

                if token.kind == TokenKind::EOF {
                    tokens.push(token);
                    break;
                } else {
                    tokens.push(token);
                }
            }
            Err(err) => {
                has_error = true;
                println!("Error: {:?} at line: {:?}", err.kind, err.line);
            }
        }
    }
}
