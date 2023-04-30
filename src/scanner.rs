use substring::Substring;

use crate::token::{Token, Type, Location};
use crate::literal::Literal;
use crate::error::{Error, ScanError};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    column_offset: usize,
}

impl Scanner {
    /// Creates a new scanner.
    pub fn new(source: String) -> Scanner {
        Scanner { source, tokens: vec!(), start: 0, current: 0, line: 0, column_offset: 0 }
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
        let temp = self.current;
        self.current += 1;

        match self.source.chars().nth(temp) {
            Some(char) => char,
            None => panic!("tried to advance past end of the file."),
        }
    }

    /// Returns the next character without consuming it.
    fn peek(&self) -> char {
        match self.source.chars().nth(self.current) {
            Some(char) => char,
            None => panic!("tried to peek past end of the file."),
        }
    }

    /// Returns the next next character without consuming it.
    fn peek_next(&self) -> char {
        match self.source.chars().nth(self.current + 1) {
            Some(char) => char,
            None => panic!("tried to peek next past end of the file."),
        }
    }

    /// Returns if the next character is the expected character.
    fn match_next(&mut self, expected: char) -> bool {
        match self.source.chars().nth(self.current) {
            Some(char) if char == expected => {
                self.current += 1;
                true
            },
            Some(_) => false,
            None => false,
        }
    }

    /// Adds a new token to the list of tokens.
    fn add_token(&mut self, r#type: Type, literal: Option<Literal>) {
        let text = self.source.substring(self.start, self.current);
        self.tokens.push(
            Token::new(
                r#type,
                String::from(text),
                literal,
                Location::new(self.line, self.start - self.column_offset)
            )
        );
    }

    /// Returns if the scanner has reached the end of the file.
    fn is_at_end(&self) -> bool {
       self.current >= self.source.len()
    }

    /// Handles a string literal.
    fn string(&mut self) {
        let start = (self.line, self.start);

        while !self.is_at_end() && self.peek() != '"' {
            if self.peek() == '\n' {
                self.line += 1;
                self.column_offset = self.current;
            }

            self.advance();
        }

        if self.is_at_end() {
            ScanError {
                location: Location::new(start.0, start.1),
                message: String::from("Unterminated string"),
            }.throw();
            return;
        }

        self.advance();  // Move to the closing double quotes.

        // Literal does not include the double quotes unlike the lexeme.
        let value = self.source.substring(self.start + 1, self.current - 1);
        self.add_token(Type::String, Some(Literal::String(String::from(value))));
    }

    /// Handles a number literal.
    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' {
            if self.peek_next().is_ascii_digit() {
                self.advance();  // Consume the dot.

                while self.peek().is_ascii_digit() {
                    self.advance();
                }
            } else {
                ScanError {
                    location: Location::new(self.line, self.start),
                    message: String::from("Unterminated number"),
                }.throw();
                return;
            }
        }

        let value: f32 = self.source.substring(self.start, self.current).parse().unwrap();
        self.add_token(Type::Number, Some(Literal::Number(value)));
    }

    /// Handles an identifier or a keyword.
    fn identifier(&mut self) {
        // is_alphanumeric does not include underscores.
        while matches!(self.peek(), c if c.is_alphanumeric() || c == '_') {
            self.advance();
        }

        let value = self.source.substring(self.start, self.current);
        let token_type = match value {
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

        self.add_token(token_type, None);
    }

    /// Scans the next token.
    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            // One character tokens
            '(' => self.add_token(Type::LeftParen, None),
            ')' => self.add_token(Type::RightParen, None),
            '{' => self.add_token(Type::LeftBrace, None),
            '}' => self.add_token(Type::RightBrace, None),
            ',' => self.add_token(Type::Comma, None),
            '.' => self.add_token(Type::Dot, None),
            '-' => self.add_token(Type::Minus, None),
            '+' => self.add_token(Type::Plus, None),
            ';' => self.add_token(Type::Semicolon, None),
            '*' => self.add_token(Type::Star, None),

            // Two character tokens
            '!' => {
                if self.match_next('=') {
                    self.add_token(Type::BangEqual, None);
                } else {
                    self.add_token(Type::Bang, None)
                };
            },
            '=' => {
                if self.match_next('=') {
                    self.add_token(Type::EqualEqual, None);
                } else {
                    self.add_token(Type::Equal, None)
                };
            },
            '<' => {
                if self.match_next('=') {
                    self.add_token(Type::LessEqual, None);
                } else {
                    self.add_token(Type::Less, None)
                };
            },
            '>' => {
                if self.match_next('=') {
                    self.add_token(Type::GreaterEqual, None);
                } else {
                    self.add_token(Type::Greater, None)
                };
            },
            '/' => {
                if self.match_next('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(Type::Slash, None);
                }
            },

            // Ignore whitespace
            ' ' | '\r' | '\t' => {},

            // Update line counter
            '\n' => {
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
