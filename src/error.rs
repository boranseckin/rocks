use crate::object::Object;
use crate::token::{Token, Type, Location};

static mut HAD_ERROR: bool = false;
static mut HAD_RUNTIME_ERROR: bool = false;

/// Checks if an error occurred during scanning, parsing, or interpreting.
pub fn did_error() -> bool {
    unsafe { HAD_ERROR || HAD_RUNTIME_ERROR }
}

/// Checks if an error occurred during runtime.
pub fn did_runtime_error() -> bool {
    unsafe { HAD_RUNTIME_ERROR }
}

/// Resets the error flag.
/// This is used to reset the interpreter after an error occurs when running prompts.
pub fn reset_error() {
    unsafe {
        HAD_ERROR = false;
        HAD_RUNTIME_ERROR = false;
    }
}

/// Every error type must implement this trait.
pub trait Error {
    /// Prints the error message and sets the error flag.
    fn throw(&self);
}

/// Represents an error that occurs during scanning.
#[derive(Debug)]
pub struct ScanError {
    pub location: Location,
    pub message: String,
}

impl Error for ScanError {
    fn throw(&self) {
        eprintln!(
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

/// Represents an error that occurs during parsing.
#[derive(Debug)]
pub struct ParseError {
    pub token: Token,
    pub message: String,
}

impl Error for ParseError {
    fn throw(&self) {
        if self.token.r#type == Type::EOF {
            eprintln!(
                "[line {line}:{column}] Error at end: {message}",
                line = self.token.location.line,
                column = self.token.location.column,
                message = self.message
            );
        } else {
            eprintln!(
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

/// Represents an error that occurs during resolution.
#[derive(Debug)]
pub struct ResolveError {
    pub token: Token,
    pub message: String,
}

impl Error for ResolveError {
    fn throw(&self) {
        eprintln!(
            "[line {line}:{column}] Error at '{lexeme}': {message}",
            line = self.token.location.line,
            column = self.token.location.column,
            lexeme = self.token.lexeme,
            message = self.message
        );

        unsafe {
            HAD_ERROR = true;
        }
    }
}

/// Represents an error that occurs during runtime.
#[derive(Debug)]
pub struct RuntimeError {
    pub token: Token,
    pub message: String,
}

impl Error for RuntimeError {
    fn throw(&self) {
        eprintln!(
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

/// Represents a special error that is caught by the interpreter.
/// This error is thrown when a return statement is executed. Since errors are propagated up the
/// execution stack, the interpreter can catch it and return the value of the return statement.
#[derive(Debug)]
pub struct ReturnError {
    pub value: Object,
}
