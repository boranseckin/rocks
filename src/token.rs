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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_token() {
        let r#type = Type::And;
        let lexeme = String::from("lex");
        let literal = Literal::String(String::from("xel"));
        let line = 12;

        let token = Token::new(r#type, lexeme, Some(literal), line);

        assert_eq!(token.r#type, Type::And);
        assert_eq!(token.lexeme, "lex");
        assert_eq!(token.literal, Some(Literal::String(String::from("xel"))));
        assert_eq!(token.line, 12);
    }

    #[test]
    fn number_literal() {
        let literal = Literal::Number(12.0);

        assert_eq!(literal.as_number(), 12.0);
        assert!(literal.as_bool());
    }

    #[test]
    fn string_literal() {
        let literal = Literal::String(String::from("12.0"));

        assert_eq!(literal.as_number(), 12.0);
        assert!(literal.as_bool());
    }

    #[test]
    fn string_literal_invalid() {
        let literal = Literal::String(String::from("x"));

        assert_eq!(literal.as_number(), 0.0);
        assert!(literal.as_bool());
    }

    #[test]
    fn bool_literal() {
        let literal = Literal::Bool(true);

        assert_eq!(literal.as_number(), 1.0);
        assert!(literal.as_bool());
    }

    #[test]
    fn null_literal() {
        let literal = Literal::Null;

        assert_eq!(literal.as_number(), 0.0);
        assert!(!literal.as_bool());
    }

    #[test]
    fn display_literal() {
        let literal = Literal::String(String::from("12.0"));

        assert_eq!(format!("{literal}"), "12.0");
    }

    #[test]
    fn from_str_literal() {
        let literal = Literal::from("12.0");

        assert_eq!(literal, Literal::String(String::from("12.0")));
    }

    #[test]
    fn from_string_literal() {
        let literal = Literal::from(String::from("12.0"));

        assert_eq!(literal, Literal::String(String::from("12.0")));
    }

    #[test]
    fn from_f32_literal() {
        let literal = Literal::from(12.0);

        assert_eq!(literal, Literal::Number(12.0));
    }

    #[test]
    fn display_token() {
        let token = Token::new(
            Type::And,
            "and".to_string(),
            None,
            1
        );

        assert_eq!(format!("{token}"), "And and None @ 1");
    }
}

