use std::fmt;

/// Represents a token in the language.
#[derive(Debug, PartialEq, Clone, Copy)]
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

/// Represents a literal value in the language.
#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    String(String),
    Number(f32),
    Bool(bool),
    Null,
}

impl Literal {
    /// Returns the literal as a number.
    pub fn as_number(&self) -> f32 {
        match self {
            Literal::Number(n) => *n,
            Literal::Null => 0.0,
            Literal::Bool(b) => if *b { 1.0 } else { 0.0 },
            Literal::String(s) => {
                match s.parse::<f32>() {
                    Ok(n) => n,
                    Err(_) => 0.0,
                }
            }
        }
    }

    /// Returns the literal as a boolean.
    pub fn as_bool(&self) -> bool {
        match self {
            Literal::Bool(b) => *b,
            Literal::Null => false,
            _ => true,
        }
    }
}

impl From<&str> for Literal {
    fn from(s: &str) -> Self {
        Literal::String(String::from(s))
    }
}

impl From<String> for Literal {
    fn from(s: String) -> Self {
        Literal::String(s.clone())
    }
}

impl From<f32> for Literal {
    fn from(n: f32) -> Self {
        Literal::Number(n)
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::String(s) => write!(f, "{s}"),
            Literal::Number(n) => write!(f, "{n}"),
            Literal::Bool(b) => write!(f, "{b}"),
            Literal::Null => write!(f, "null"),
        }
    }
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
        assert_eq!(literal.as_bool(), true);
    }

    #[test]
    fn string_literal() {
        let literal = Literal::String(String::from("12.0"));

        assert_eq!(literal.as_number(), 12.0);
        assert_eq!(literal.as_bool(), true);
    }

    #[test]
    fn string_literal_invalid() {
        let literal = Literal::String(String::from("x"));

        assert_eq!(literal.as_number(), 0.0);
        assert_eq!(literal.as_bool(), true);
    }

    #[test]
    fn bool_literal() {
        let literal = Literal::Bool(true);

        assert_eq!(literal.as_number(), 1.0);
        assert_eq!(literal.as_bool(), true);
    }

    #[test]
    fn null_literal() {
        let literal = Literal::Null;

        assert_eq!(literal.as_number(), 0.0);
        assert_eq!(literal.as_bool(), false);
    }

    #[test]
    fn display_literal() {
        let literal = Literal::String(String::from("12.0"));

        assert_eq!(format!("{}", literal), "12.0");
    }
}

