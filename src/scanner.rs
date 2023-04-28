use std::str::Chars;

use peekmore::{PeekMore, PeekMoreIterator};

use crate::token::{Token, Type, Location};
use crate::literal::Literal;
use crate::error::{Error, ScanError};

pub struct Scanner<'a> {
    source: PeekMoreIterator<Chars<'a>>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    column_offset: usize,
}

impl<'a> Scanner<'a> {
    /// Creates a new scanner.
    pub fn new(source: &'a str) -> Scanner<'a> {
        Scanner {
            source: source.chars().peekmore(),
            tokens: vec!(),
            start: 0,
            current: 0,
            line: 0,
            column_offset: 0
        }
    }

    /// Scans the source code and returns a vector of tokens.
    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(
            Token::new(
                Type::EOF,
                String::from(""),
                None,
                Location::new(self.line, 0)
            )
        );

        self.tokens.clone()
    }

    /// Returns the next character.
    fn advance(&mut self) -> char {
        match self.source.next() {
            Some(char) => {
                self.current += 1;
                char
            },
            None => panic!("tried to advance past end of the file."),
        }
    }

    /// Returns the next character without consuming it.
    fn peek(&mut self) -> &char {
        match self.source.peek() {
            Some(char) => char,
            None => panic!("tried to peek past end of the file."),
        }
    }

    /// Returns the next next character without consuming it.
    fn peek_next(&mut self) -> &char {
        match self.source.peek_next() {
            Some(char) => char,
            None => panic!("tried to peek next past end of the file."),
        }
    }

    /// Returns if the next character is the expected character.
    fn match_next(&mut self, expected: char) -> bool {
        match self.source.peek_next() {
            Some(char) if *char == expected => true,
            Some(_) => false,
            None => false,
        }
    }

    /// Adds a new token to the list of tokens.
    fn add_token(&mut self, r#type: Type, lexeme: String, literal: Option<Literal>) {
        self.tokens.push(
            Token::new(
                r#type,
                lexeme,
                literal,
                Location::new(self.line, self.start - self.column_offset)
            )
        );
    }

    /// Adds a new single char token to the list of tokens.
    fn add_single_char_token(&mut self, r#type: Type) {
        let c = self.advance();
        self.add_token(r#type, c.to_string(), None);
    }

    /// Adds a new double char token to the list of tokens.
    fn add_double_char_token(&mut self, r#type: Type) {
        let first = self.advance();
        let second = self.advance();

        self.add_token(r#type, format!("{first}{second}"), None);
    }

    /// Returns if the scanner has reached the end of the file.
    fn is_at_end(&mut self) -> bool {
        self.source.peek().is_none()
    }

    /// Handles a string literal.
    fn string(&mut self) {
        self.advance(); // Move past the starting double quotes.
        let start = (self.line, self.start);

        let mut value = Vec::new();
        while !self.is_at_end() {
            match self.source.next_if(|&x| x != '"') {
                Some(c) => {
                    self.current += 1;
                    value.push(c);
                },
                None => { break; },
            }
        }

        if self.is_at_end() {
            ScanError {
                location: Location::new(start.0, start.1),
                message: String::from("Unterminated string"),
            }.throw();
            return;
        }

        self.advance();  // Move to the closing double quotes.

        let value: String = value.into_iter().collect();

        // Literal does not include the double quotes unlike the lexeme.
        self.add_token(Type::String, value.clone(), Some(Literal::String(value)));
    }

    /// Handles a number literal.
    fn number(&mut self) {
        let mut value = Vec::new();

        while self.peek().is_ascii_digit() {
            value.push(self.advance());
        }

        if *self.peek() == '.' {
            if self.peek_next().is_ascii_digit() {
                value.push(self.advance());  // Consume the dot.

                while self.peek().is_ascii_digit() {
                    value.push(self.advance());
                }
            } else {
                ScanError {
                    location: Location::new(self.line, self.start),
                    message: String::from("Unterminated number"),
                }.throw();
                return;
            }
        }

        let value: String = value.into_iter().collect();
        let value_num: f32 = value.parse().unwrap();

        self.add_token(Type::Number, value, Some(Literal::Number(value_num)));
    }

    /// Handles an identifier or a keyword.
    fn identifier(&mut self) {
        let mut value = Vec::new();

        // is_alphanumeric does not include underscores.
        while matches!(self.peek(), c if c.is_alphanumeric() || *c == '_') {
            value.push(self.advance());
        }

        let value = String::from_iter(value);
        let token_type = match value.as_str() {
            "and"      => Type::And,
            "class"    => Type::Class,
            "else"     => Type::Else,
            "false"    => Type::False,
            "for"      => Type::For,
            "fun"      => Type::Fun,
            "if"       => Type::If,
            "null"     => Type::Null,
            "or"       => Type::Or,
            "print"    => Type::Print,
            "return"   => Type::Return,
            "break"    => Type::Break,
            "super"    => Type::Super,
            "this"     => Type::This,
            "true"     => Type::True,
            "var"      => Type::Var,
            "while"    => Type::While,
            _          => Type::Identifier,
        };

        self.add_token(token_type, value, None);
    }

    /// Scans the next token.
    fn scan_token(&mut self) {
        let c = *self.peek();
        match c {
            // One character tokens
            '(' => self.add_single_char_token(Type::LeftParen),
            ')' => self.add_single_char_token(Type::RightParen),
            '{' => self.add_single_char_token(Type::LeftBrace),
            '}' => self.add_single_char_token(Type::RightBrace),
            ',' => self.add_single_char_token(Type::Comma),
            '.' => self.add_single_char_token(Type::Dot),
            '-' => self.add_single_char_token(Type::Minus),
            '+' => self.add_single_char_token(Type::Plus),
            ';' => self.add_single_char_token(Type::Semicolon),
            '*' => self.add_single_char_token(Type::Star),

            // Two character tokens
            '!' => {
                if self.match_next('=') {
                    self.add_double_char_token(Type::BangEqual);
                } else {
                    self.add_single_char_token(Type::Bang)
                };
            },
            '=' => {
                if self.match_next('=') {
                    self.add_double_char_token(Type::EqualEqual);
                } else {
                    self.add_single_char_token(Type::Equal)
                };
            },
            '<' => {
                if self.match_next('=') {
                    self.add_double_char_token(Type::LessEqual);
                } else {
                    self.add_single_char_token(Type::Less)
                };
            },
            '>' => {
                if self.match_next('=') {
                    self.add_double_char_token(Type::GreaterEqual);
                } else {
                    self.add_single_char_token(Type::Greater)
                };
            },
            '/' => {
                if self.match_next('/') {
                    while *self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_single_char_token(Type::Slash);
                }
            },

            // Ignore whitespace
            ' ' | '\r' | '\t' => {
                self.advance();
            },

            // Update line counter
            '\n' => {
                self.advance();

                self.line += 1;
                self.column_offset = self.current;
            },

            // String
            '"' => self.string(),

            // Numbers
            c if c.is_ascii_digit() => self.number(),

            // Identifiers
            c if c.is_alphabetic() || c == '_' => self.identifier(),

            _ => {
                ScanError {
                    location: Location::new(self.line, self.start - self.column_offset),
                    message: format!("Unexpected character '{c}'"),
                }.throw();
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_scanner() {
        let mut scanner = Scanner::new("test".to_string());
        let tokens = scanner.scan_tokens();
        assert_eq!(tokens.len(), 2);
    }

    #[test]
    fn create_scanner_with_empty_string() {
        let mut scanner = Scanner::new("".to_string());
        let tokens = scanner.scan_tokens();
        assert_eq!(tokens.len(), 1);
    }

    #[test]
    fn create_scanner_with_whitespace() {
        let mut scanner = Scanner::new(" ".to_string());
        let tokens = scanner.scan_tokens();
        assert_eq!(tokens.len(), 1);
    }

    #[test]
    fn create_scanner_with_comment() {
        let mut scanner = Scanner::new("// comment".to_string());
        let tokens = scanner.scan_tokens();
        assert_eq!(tokens.len(), 1);
    }

    #[test]
    fn advance() {
        let mut scanner = Scanner::new("var".to_string());
        assert_eq!(scanner.advance(), 'v');
        assert_eq!(scanner.advance(), 'a');
        assert_eq!(scanner.advance(), 'r');
        assert_eq!(scanner.advance(), '\n');
    }

    #[test]
    #[should_panic(expected = "tried to advance past end of the file.")]
    fn advance_past_end() {
        let mut scanner = Scanner::new("".to_string());
        scanner.advance();
        scanner.advance();
    }

    #[test]
    fn peek() {
        let mut scanner = Scanner::new("var".to_string());
        assert_eq!(scanner.peek(), 'v');
        assert_eq!(scanner.peek(), 'v');
        scanner.advance();
        assert_eq!(scanner.peek(), 'a');
        assert_eq!(scanner.peek(), 'a');
    }

    #[test]
    #[should_panic(expected = "tried to peek past end of the file.")]
    fn peek_past_end() {
        let mut scanner = Scanner::new("".to_string());
        scanner.advance();
        scanner.peek();
    }

    #[test]
    fn peek_next() {
        let mut scanner = Scanner::new("var".to_string());
        assert_eq!(scanner.peek_next(), 'a');
        assert_eq!(scanner.peek_next(), 'a');
        scanner.advance();
        assert_eq!(scanner.peek_next(), 'r');
        assert_eq!(scanner.peek_next(), 'r');
    }

    #[test]
    #[should_panic(expected = "tried to peek next past end of the file.")]
    fn peek_next_past_end() {
        let scanner = Scanner::new("".to_string());
        scanner.peek_next();
    }

    #[test]
    fn match_next() {
        let mut scanner = Scanner::new("var".to_string());
        assert!(scanner.match_next('v'));
        assert!(!scanner.match_next('x'));
        assert!(scanner.match_next('a'));
    }
}
