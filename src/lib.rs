use std::{fs, io, process};

pub mod error;
pub mod token;
pub mod scanner;
pub mod expr;
pub mod parser;
pub mod ast;
pub mod interpreter;

use ast::ASTPrinter;
use parser::Parser;
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
    let tokens = scanner.scan_tokens();

    unsafe {
        if HAD_ERROR {
            return;
        }
    }

    let mut parser = Parser::new(tokens);
    let expression = parser.parse();

    unsafe {
        if HAD_ERROR {
            return;
        }
    }

    let mut ast = ASTPrinter {};
    println!("{}", ast.print(expression.clone().unwrap()));

    let mut interpreter = interpreter::Interpreter {};
    println!("{}", interpreter.evaluate(&expression.unwrap()));
}

