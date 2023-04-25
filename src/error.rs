use crate::{HAD_ERROR, HAD_RUNTIME_ERROR};
use crate::object::Object;
use crate::token::{Token, Type, Location};

pub fn did_error() -> bool {
    unsafe { HAD_ERROR || HAD_RUNTIME_ERROR }
}

#[allow(non_camel_case_types)]
pub trait Error {
    fn throw(&self);
}

#[derive(Debug)]
pub struct ScanError {
    pub location: Location,
    pub message: String,
}

impl Error for ScanError {
    fn throw(&self) {
        println!(
            "[line {line}:{column}] Error: {message}",
            line = self.location.line + 1,
            column = self.location.column + 1,
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

impl Error for ParseError {
    fn throw(&self) {
        if self.token.r#type == Type::EOF {
            println!(
                "[line {line}:{column}] Error at end: {message}",
                line = self.token.location.line,
                column = self.token.location.column,
                message = self.message
            );
        } else {
            println!(
                "[line {line}:{column}] Error at '{lexeme}': {message}",
                line = self.token.location.line,
                column = self.token.location.column,
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

impl Error for RuntimeError {
    fn throw(&self) {
        println!(
            "[line {line}:{column}] Error at '{lexeme}': {message}",
            line = self.token.location.line,
            column = self.token.location.column,
            lexeme = self.token.lexeme,
            message = self.message
        );

        unsafe {
            HAD_RUNTIME_ERROR = true;
        }
    }
}

/// Used to return from functions
#[derive(Debug)]
pub struct ReturnError {
    pub value: Object,
}
