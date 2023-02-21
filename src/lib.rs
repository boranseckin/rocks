use std::{fs, io::{self, Write}, process};

pub mod error;
pub mod token;
pub mod scanner;
pub mod expr;
pub mod stmt;
pub mod parser;
pub mod ast;
pub mod interpreter;

use parser::Parser;
use scanner::Scanner;

static mut HAD_ERROR: bool = false;
static mut HAD_RUNTIME_ERROR: bool = false;

#[allow(non_camel_case_types)]
pub struct rlox {
    interpreter: interpreter::Interpreter,
}

impl rlox {
    pub fn new() -> Self {
        rlox {
            interpreter: interpreter::Interpreter {},
        }
    }

    pub fn run_file(&mut self, path: String) {
        let contents = fs::read_to_string(path)
            .expect("Should have been able to read the file");

        self.run(contents);

        unsafe {
            if HAD_ERROR {
                process::exit(65);
            }
            if HAD_RUNTIME_ERROR {
                process::exit(70);
            }
        }
    }

    pub fn run_prompt(&mut self) {
        loop {
            let mut input = String::new();
            io::stdout().write_all(b"> ").unwrap();
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).expect("acceptable expression");

            self.run(input);

            unsafe {
                HAD_ERROR = false;
                HAD_RUNTIME_ERROR = false;
            }
        }
    }

    fn run(&mut self, source: String) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        unsafe {
            if HAD_ERROR {
                return;
            }
        }

        let mut parser = Parser::new(tokens);
        let statements = parser.parse();

        unsafe {
            if HAD_ERROR {
                return;
            }
        }

        self.interpreter.interpret(&statements);
    }
}

impl Default for rlox {
    fn default() -> Self {
        Self::new()
    }
}
