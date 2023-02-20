use substring::Substring;

use crate::token::{Token, Type, Literal};
use crate::error::{rloxError, ScanError};

pub struct Scanner {
    source: String,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    /// Creates a new scanner.
    pub fn new(source: String) -> Scanner {
        Scanner { source, tokens: vec!(), start: 0, current: 0, line: 1 }
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
                self.line
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
                self.line
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
            }

            self.advance();
        }

        if self.is_at_end() {
            ScanError {
                line: start.0,
                location: start.1,
                message: String::from("Unterminated string"),
            }.throw();
            return;
        }

        println!("string: {}", self.source.substring(self.start, self.current));

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
                    line: self.line,
                    location: self.start,
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
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        let value = self.source.substring(self.start, self.current);
        let token_type = match value {
            "and"    => Type::And,
            "class"  => Type::Class,
            "else"   => Type::Else,
            "false"  => Type::False,
            "for"    => Type::For,
            "fun"    => Type::Fun,
            "if"     => Type::If,
            "null"   => Type::Null,
            "or"     => Type::Or,
            "print"  => Type::Print,
            "return" => Type::Return,
            "super"  => Type::Super,
            "this"   => Type::This,
            "true"   => Type::True,
            "var"    => Type::Var,
            "while"  => Type::While,
            _        => Type::Identifier,
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
            '\n' => self.line += 1,

            // String
            '"' => self.string(),

            _ => {
                // Numbers
                if c.is_ascii_digit() {
                    self.number();
                // Identifiers
                } else if c.is_alphabetic() || c == '_' {
                    self.identifier();
                // Unknown
                } else {
                    ScanError {
                        line: self.line,
                        location: self.current,
                        message: format!("Unexpected character '{}'", c),
                    }.throw();
                }
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn advance() {
        let mut scanner = Scanner::new(String::from("abcdefg"));

        assert_eq!(scanner.current, 0);

        let result = scanner.advance();
        assert_eq!(result, 'a');
        assert_eq!(scanner.current, 1);

        let result = scanner.advance();
        assert_eq!(result, 'b');
        assert_eq!(scanner.current, 2);
    }

    #[test]
    #[should_panic(expected = "tried to advance past end of the file.")]
    fn advance_eof() {
        let mut scanner = Scanner::new(String::from("a"));

        scanner.advance();
        scanner.advance();
    }

    #[test]
    fn match_next_truthy() {
        let mut scanner = Scanner::new(String::from("!="));
        scanner.advance();  // Move to the first char
        let result = scanner.match_next('=');
        assert!(result); 
        assert_eq!(scanner.current, 2);
    }

    #[test]
    fn match_next_faulty() {
        let mut scanner = Scanner::new(String::from("!a"));
        scanner.advance();  // Move to the first char

        let result = scanner.match_next('=');
        assert!(!result);
        assert_eq!(scanner.current, 1);  // Should not move the current
    }

    #[test]
    fn match_next_eof() {
        let mut scanner = Scanner::new(String::from("a"));
        scanner.advance();  // Move to the first char

        let result = scanner.match_next('b');
        assert!(!result);
        assert_eq!(scanner.current, 1);  // Should not move the current
    }

    #[test]
    fn peek() {
        let mut scanner = Scanner::new(String::from("abc"));
        scanner.advance();

        let result = scanner.peek();
        assert_eq!(result, 'b');
        assert_eq!(scanner.current, 1);  // Should not move the current
    }

    #[test]
    #[should_panic(expected = "tried to peek past end of the file.")]
    fn peek_eof() {
        let mut scanner = Scanner::new(String::from("a"));
        scanner.advance();
        scanner.peek();
     }

    #[test]
    fn peek_next() {
        let mut scanner = Scanner::new(String::from("abc"));
        scanner.advance();

        let result = scanner.peek_next();
        assert_eq!(result, 'c');
        assert_eq!(scanner.current, 1);  // Should not move the current
    }

    #[test]
    #[should_panic(expected = "tried to peek next past end of the file.")]
    fn peek_next_eof() {
        let scanner = Scanner::new(String::from("a"));
        scanner.peek_next();
     }

    #[test]
    fn is_at_end() {
        let mut scanner = Scanner::new(String::from("ab"));

        scanner.advance();
        assert!(!scanner.is_at_end());

        scanner.advance();
        assert!(scanner.is_at_end());
    }

    #[test]
    fn add_token() {
        let mut scanner = Scanner::new(String::from("a"));
        scanner.add_token(Type::Identifier, None);

        assert_eq!(scanner.tokens.len(), 1);
        assert_eq!(scanner.tokens[0].r#type, Type::Identifier);
        assert_eq!(scanner.tokens[0].literal, None);
    }

    #[test]
    fn string() {
        let mut scanner = Scanner::new(String::from("\"hello\""));
        scanner.current = 1; // Skip the first quote
        scanner.string();

        assert_eq!(scanner.tokens.len(), 1);
        assert_eq!(scanner.tokens[0].r#type, Type::String);
        assert_eq!(scanner.tokens[0].literal, Some(Literal::String(String::from("hello"))));
    }

    #[test]
    fn string_unterminated() {
        let mut scanner = Scanner::new(String::from("\"hello\n"));
        scanner.current = 1; // Skip the first quote
        scanner.string();

        assert_eq!(scanner.tokens.len(), 0);
    }

    #[test]
    fn number() {
        let mut scanner = Scanner::new(String::from("123\n"));
        scanner.number();

        assert_eq!(scanner.tokens.len(), 1);
        assert_eq!(scanner.tokens[0].r#type, Type::Number);
        assert_eq!(scanner.tokens[0].literal, Some(Literal::Number(123.0)));
    }

    #[test]
    fn identifier() {
        let mut scanner = Scanner::new(String::from("abc\n"));
        scanner.identifier();

        assert_eq!(scanner.tokens.len(), 1);
        assert_eq!(scanner.tokens[0].r#type, Type::Identifier);
        assert_eq!(scanner.tokens[0].lexeme, String::from("abc"));
        assert_eq!(scanner.tokens[0].literal, None);
    }

    #[test]
    fn keyword() {
        let mut scanner = Scanner::new(String::from("var\n"));
        scanner.identifier();

        assert_eq!(scanner.tokens.len(), 1);
        assert_eq!(scanner.tokens[0].r#type, Type::Var);
        assert_eq!(scanner.tokens[0].lexeme, String::from("var"));
        assert_eq!(scanner.tokens[0].literal, None);
    }

    #[test]
    fn scan_tokens() {
        let mut scanner = Scanner::new(String::from("var a = 123;\n"));
        scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 6);
        assert_eq!(scanner.tokens[0].r#type, Type::Var);
        assert_eq!(scanner.tokens[1].r#type, Type::Identifier);
        assert_eq!(scanner.tokens[2].r#type, Type::Equal);
        assert_eq!(scanner.tokens[3].r#type, Type::Number);
        assert_eq!(scanner.tokens[4].r#type, Type::Semicolon);
        assert_eq!(scanner.tokens[5].r#type, Type::EOF);
    }

    #[test]
    fn scan_tokens_with_comments() {
        let mut scanner = Scanner::new(String::from("var a = 123; // This is a comment\n"));
        scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 6);
        assert_eq!(scanner.tokens[0].r#type, Type::Var);
        assert_eq!(scanner.tokens[1].r#type, Type::Identifier);
        assert_eq!(scanner.tokens[2].r#type, Type::Equal);
        assert_eq!(scanner.tokens[3].r#type, Type::Number);
        assert_eq!(scanner.tokens[4].r#type, Type::Semicolon);
        assert_eq!(scanner.tokens[5].r#type, Type::EOF);
    }
}

