use crate::{HAD_ERROR, HAD_RUNTIME_ERROR};
use crate::token::{Token, Type};

#[allow(non_camel_case_types)]
pub trait rloxError {
    fn throw(&self);
}

#[derive(Debug)]
pub struct ScanError {
    pub line: usize,
    pub location: usize,
    pub message: String,
}

impl rloxError for ScanError {
    fn throw(&self) {
        println!(
            "[line {line}:{location}] Error: {message}",
            line = self.line,
            location = self.location + 1,
            message = self.message
        );

        unsafe {
            HAD_ERROR = true;
        }
    }
}

#[derive(Debug)]
pub struct ParseError {
    pub token: Token,
    pub message: String,
}

impl rloxError for ParseError {
    fn throw(&self) {
        if self.token.r#type == Type::EOF {
            println!(
                "[line {line}] Error at end: {message}",
                line = self.token.line,
                message = self.message
            );
        } else {
            println!(
                "[line {line}] Error at '{lexeme}': {message}",
                line = self.token.line,
                lexeme = self.token.lexeme,
                message = self.message
            );
        }

        unsafe {
            HAD_ERROR = true;
        }
    }
}

#[derive(Debug)]
pub struct RuntimeError {
    pub token: Token,
    pub message: String,
}

impl rloxError for RuntimeError {
    fn throw(&self) {
        println!(
            "[line {line}] Error at '{lexeme}': {message}",
            line = self.token.line,
            lexeme = self.token.lexeme,
            message = self.message
        );

        unsafe {
            HAD_RUNTIME_ERROR = true;
        }
    }
}

