use std::{fs, io, process};

pub mod token;

mod scanner;
use scanner::Scanner;

static mut HAD_ERROR: bool = false;

pub fn run_file(path: String) {
    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");
    run(contents);

    unsafe {
        if HAD_ERROR {
            process::exit(65);
        }
    }
}

pub fn run_prompt() {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("acceptable expression");

        run(input);

        unsafe {
            HAD_ERROR = false;
        }
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
    report(line, None, message);
}

fn report(line: usize, location: Option<usize>, message: &str) {
    if let Some(location) = location {
        let location = location + 1;
        println!("[{line}:{location}] Error: {message}");
    } else {
        println!("[{line}] Error: {message}");
    }

    unsafe {
        HAD_ERROR = true;
    }
}

