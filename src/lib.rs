use std::{fs, io};

pub mod token;

mod scanner;
use scanner::Scanner;

pub fn run_file(path: String) {
    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");
    run(contents);
}

pub fn run_prompt() {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("acceptable expression");

        run(input);
    }
}

fn run(source: String) {
    let mut scanner = Scanner::new(source);
    scanner.scan_tokens();

    for token in scanner.tokens.iter() {
        println!("{:#?}", token);
    }
}

pub fn error(line: usize, message: &str) {
    // TODO: add had_error
    report(line, "", message);
}

fn report(line: usize, location: &str, message: &str) {
    println!("[line {line}] Error{location}: {message}");
}

