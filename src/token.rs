use std::{fmt, hash::Hash};

use crate::literal::Literal;

/// Represents a token in the language.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Type {
  // Single-character tokens.
  LeftParen, RightParen, LeftBrace, RightBrace,
  Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

  // One or two character tokens.
  Bang, BangEqual,
  Equal, EqualEqual,
  Greater, GreaterEqual,
  Less, LessEqual,

  // Literals.
  Identifier, String, Number,

  // Keywords.
  And, Class, Else, False, Fun, For, If, Null, Or,
  Print, Return, Super, This, True, Var, While,

  EOF
}

/// Represents a token in the language.
#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub r#type: Type,
    pub lexeme: String, // String representation of the token
    pub literal: Option<Literal>,
    pub line: usize,
}

impl Token {
    /// Creates a new token.
    pub fn new(
        r#type: Type,
        lexeme: String,
        literal: Option<Literal>,
        line: usize
    ) -> Token {
        Token { r#type, lexeme, literal, line }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?} {} {:#?} @ {}", self.r#type, self.lexeme, self.literal, self.line)
    }
}

impl Hash for Token {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.r#type.hash(state);
        self.lexeme.hash(state);
        self.line.hash(state);
    }
}

impl Eq for Token {}
