use crate::{HAD_ERROR, HAD_RUNTIME_ERROR};
use crate::object::Object;
use crate::token::{Token, Type};

pub fn did_error() -> bool {
    unsafe { HAD_ERROR }
}

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

/// Used to return from functions
#[derive(Debug)]
pub struct ReturnError {
    pub value: Object,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::literal::Literal;

    #[test]
    fn scan_error() {
        let error = ScanError {
            line: 1,
            location: 2,
            message: String::from("test"),
        };

        error.throw();

        assert!(did_error());
    }

    #[test]
    fn parse_error() {
        let error = ParseError {
            token: Token::new(
                Type::And,
                String::from("lex"),
                Some(Literal::String(String::from("xel"))),
                12,
            ),
            message: String::from("test"),
        };

        error.throw();

        assert!(did_error());
    }

    #[test]
    fn parse_error_eof() {
        let error = ParseError {
            token: Token::new(Type::EOF, String::from(""), None, 12),
            message: String::from("test"),
        };

        error.throw();

        assert!(did_error());
    }

    #[test]
    fn runtime_error() {
        let error = RuntimeError {
            token: Token::new(
                Type::And,
                String::from("lex"),
                Some(Literal::String(String::from("xel"))),
                12,
            ),
            message: String::from("test"),
        };

        error.throw();

        assert!(did_error());
    }
}
