use std::fmt::{self, Display};
use std::hash::Hash;

use crate::literal::Literal;

/// Represents a token type in the language.
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
    Print, Return, Break, Super, This, True, Var, While,

    EOF
}

/// Represents a location in the source code.
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

impl Location {
    /// Creates a new location.
    pub fn new(line: usize, column: usize) -> Self {
        Location { line, column }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

/// Represents a token in the language.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Token {
    /// Type of the token.
    pub r#type: Type,
    /// String representation of the token.
    pub lexeme: String,
    /// Literal value of the token (if any).
    pub literal: Option<Literal>,
    /// Location of the token in the source code.
    pub location: Location,
}

impl Token {
    /// Creates a new token.
    pub fn new(
        r#type: Type,
        lexeme: String,
        literal: Option<Literal>,
        location: Location,
    ) -> Token {
        Token { r#type, lexeme, literal, location }
    }
}

/// Convenience methods for creating dummy tokens.
impl From<&str> for Token {
    fn from(token: &str) -> Self {
        Token::new(Type::Identifier, token.to_string(), None, Location::new(0, 0))
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?} {} {:#?} @ [{}]", self.r#type, self.lexeme, self.literal, self.location)
    }
}

/// This is required for the `HashMap` to work for [`Interpreter::locals`](crate::interpreter::Interpreter).
/// Since no two tokens can have the same type, lexeme, and location, we can use them as the key for
/// the `HashMap`.
impl Hash for Token {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.r#type.hash(state);
        self.lexeme.hash(state);
        self.location.hash(state);
    }
}
