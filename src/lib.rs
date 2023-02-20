use std::{fs, io, process};

pub mod token;
pub mod scanner;
pub mod expr;
pub mod parser;
pub mod ast;
pub mod interpreter;

use ast::ASTPrinter;
use parser::Parser;
use scanner::Scanner;
use token::{Token, Type};

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

pub fn scan_error(line: usize, message: &str) {
    report(line, None, message);
}

fn report(line: usize, location: Option<usize>, message: &str) {
    if let Some(location) = location {
        let location = location + 1;
        println!("[line {line}:{location}] Error: {message}");
    } else {
        println!("[line {line}] Error: {message}");
    }

    unsafe {
        HAD_ERROR = true;
    }
}

pub fn parse_error(token: &Token, message: &str) {
    if token.r#type == Type::EOF {
        println!("[line {}] Error at end: {message}", token.line);
    } else {
        println!("[line {}] Error at '{}': {message}", token.line, token.lexeme);
    }

    unsafe {
        HAD_ERROR = true;
    }
}

