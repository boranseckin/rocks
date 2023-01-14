use std::fmt;

#[allow(non_camel_case_types)]
#[derive(Debug)]
#[derive(PartialEq, Clone, Copy)]
pub enum Type {
  // Single-character tokens.
  LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
  COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

  // One or two character tokens.
  BANG, BANG_EQUAL,
  EQUAL, EQUAL_EQUAL,
  GREATER, GREATER_EQUAL,
  LESS, LESS_EQUAL,

  // Literals.
  IDENTIFIER, STRING, NUMBER,

  // Keywords.
  AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
  PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

  EOF
}

#[derive(Debug)]
#[derive(PartialEq, Clone)]
pub enum Literal {
    String(String),
    Float(f32),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    r#type: Type,
    lexeme: String,
    literal: Option<Literal>,
    line: usize,
}

impl Token {
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
        let r#type = Type::AND;
        let lexeme = String::from("lex");
        let literal = Literal::String(String::from("xel"));
        let line = 12;

        let token = Token::new(r#type, lexeme, Some(literal), line);

        assert_eq!(token.r#type, Type::AND);
        assert_eq!(token.lexeme, "lex");
        assert_eq!(token.literal, Some(Literal::String(String::from("xel"))));
        assert_eq!(token.line, 12);
    }
}

