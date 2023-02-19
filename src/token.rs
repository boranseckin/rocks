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
  And, Class, Else, False, Fun, For, If, Nil, Or,
  Print, Return, Super, This, True, Var, While,

  EOF
}

/// Represents a literal value in the language.
#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    String(String),
    Number(f32),
    Bool(bool),
    Nil,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::String(s) => write!(f, "{s}"),
            Literal::Number(n) => write!(f, "{n}"),
            Literal::Bool(b) => write!(f, "{b}"),
            Literal::Nil => write!(f, "nil"),
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
}

