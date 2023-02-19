use crate::parse_error;
use crate::token::{Token, Type, Literal};
use crate::expr::{Expr, BinaryData, UnaryData, GroupingData};

#[derive(Debug, Clone)]
struct ParseError;

type ParseResult = Result<Expr, ParseError>;

fn error(token: &Token, message: &str) {
    parse_error(token, message);
}

/// Parses the tokens and returns the resulting expression.
///
/// - Expression -> Equality ;
/// - Equality   -> Comparison ( ( "!=" | "==" ) Comparison )* ;
/// - Comparison -> Term ( ( ">" | ">=" | "<" | "<=" ) Term )* ;
/// - Term       -> Factor ( ( "+" | "-" ) Factor )* ;
/// - Factor     -> Unary ( ( "*" | "/" ) Unary )* ;
/// - Unary      -> ( "!" | "-" ) Unary | Primary ;
/// - Primary    -> NUMBER | STRING | false | true | null | "(" expression ")" ;
pub struct Parser {
    tokens: Vec<Token>,
    current: u32,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
        }
    }

    /// Parses the tokens and returns the resulting expression.
    pub fn parse(&mut self) -> Option<Expr> {
        match self.expression() {
            Ok(expr) => Some(expr),
            Err(_) => None,
        }
    }

    /// Returns the next token without consuming it.
    fn peek(&mut self) -> &Token {
        &self.tokens[self.current as usize]
    }

    /// Returns the previous token without consuming it.
    fn previous(&mut self) -> &Token {
        &self.tokens[(self.current - 1) as usize]
    }

    /// Returns if the parser has reached the end of the file.
    fn is_at_end(&mut self) -> bool {
        self.peek().r#type == Type::EOF
    }

    /// Returns if the next token is of the given type.
    fn check(&mut self, r#type: Type) -> bool {
        if self.is_at_end() {
            return false
        }

        self.peek().r#type == r#type
    }

    /// Consumes the next token and returns it.
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    /// Returns if the next token is of any of the given types.
    fn matches(&mut self, types: Vec<Type>) -> bool {
        for r#type in types {
            if self.check(r#type) {
                self.advance();
                return true;
            }
        }

        false
    }

    /// Consumes the next token if it is of the given type.
    fn consume(&mut self, r#type: Type, message: &str) -> Result<&Token, ParseError> {
        if self.check(r#type) {
            return Ok(self.advance());
        }

        error(self.peek(), message);
        Err(ParseError) 
    }

    /// Parses an expression.
    fn expression(&mut self) -> ParseResult {
        self.equality()
    }

    /// Parses an equality expression.
    fn equality(&mut self) -> ParseResult {
        let mut expr = match self.comparison() {
            Ok(expr) => expr,
            Err(ParseError) => return Err(ParseError),
        };

        while self.matches(vec![Type::BangEqual, Type::EqualEqual]) {
            let operator = self.previous().clone();
            let right = match self.comparison() {
                Ok(expr) => expr,
                Err(ParseError) => return Err(ParseError),
            };

            expr = Expr::Binary(BinaryData {
                left: Box::new(expr),
                operator,
                right: Box::new(right)
            });
        }

        Ok(expr)
    }

    /// Parses a comparison expression.
    fn comparison(&mut self) -> ParseResult {
        let mut expr = match self.term() {
            Ok(expr) => expr,
            Err(ParseError) => return Err(ParseError),
        };

        while self.matches(vec![Type::Greater, Type::GreaterEqual, Type::Less, Type::LessEqual]) {
            let operator = self.previous().clone();
            let right = match self.term() {
                Ok(expr) => expr,
                Err(ParseError) => return Err(ParseError),
            };

            expr = Expr::Binary(BinaryData {
                left: Box::new(expr),
                operator,
                right: Box::new(right)
            });
        }

        Ok(expr)
    }

    /// Parses a term expression.
    fn term(&mut self) -> ParseResult {
        let mut expr = match self.factor() {
            Ok(expr) => expr,
            Err(ParseError) => return Err(ParseError),
        };

        while self.matches(vec![Type::Minus, Type::Plus]) {
            let operator = self.previous().clone();
            let right = match self.factor() {
                Ok(expr) => expr,
                Err(ParseError) => return Err(ParseError),
            };

            expr = Expr::Binary(BinaryData {
                left: Box::new(expr),
                operator,
                right: Box::new(right)
            });
        }

        Ok(expr)
    }

    /// Parses a factor expression.
    fn factor(&mut self) -> ParseResult {
        let mut expr = match self.unary() {
            Ok(expr) => expr,
            Err(ParseError) => return Err(ParseError),
        };

        while self.matches(vec![Type::Slash, Type::Star]) {
            let operator = self.previous().clone();
            let right = match self.unary() {
                Ok(expr) => expr,
                Err(ParseError) => return Err(ParseError),
            };

            expr = Expr::Binary(BinaryData {
                left: Box::new(expr),
                operator,
                right: Box::new(right)
            });
        }

        Ok(expr)
    }

    /// Parses a unary expression.
    fn unary(&mut self) -> ParseResult {
        if self.matches(vec![Type::Bang, Type::Minus]) {
            let operator = self.previous().clone();
            let right = match self.unary() {
                Ok(expr) => expr,
                Err(ParseError) => return Err(ParseError),
            };

            return Ok(Expr::Unary(UnaryData {
                operator,
                expr: Box::new(right)
            }));
        }

        self.primary()
    }

    /// Parses a primary expression.
    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.matches(vec![Type::False]) {
            return Ok(Expr::Literal(Literal::Bool(false)));
        }

        if self.matches(vec![Type::True]) {
            return Ok(Expr::Literal(Literal::Bool(true)));
        }

        if self.matches(vec![Type::Null]) {
            return Ok(Expr::Literal(Literal::Null));
        }

        if self.matches(vec![Type::Number, Type::String]) {
            return Ok(Expr::Literal(self.previous().clone().literal
                .expect("number or string to have a literal value")));
        }

        if self.matches(vec![Type::LeftParen]) {
            let expr = match self.expression() {
                Ok(expr) => expr,
                Err(ParseError) => return Err(ParseError),
            };

            match self.consume(Type::RightParen, "Expect ')' after expression") {
                Ok(_) => (),
                Err(ParseError) => return Err(ParseError),
            };

            return Ok(Expr::Grouping(GroupingData { expr: Box::new(expr) }));
        }

        error(self.peek(), "Expression expected.");
        Err(ParseError)
    }

    #[allow(dead_code)]
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().r#type == Type::Semicolon {
                return;
            }

            match self.peek().r#type {
                Type::Class => return,
                Type::Fun => return,
                Type::Var => return,
                Type::For => return,
                Type::If => return,
                Type::While => return,
                Type::Print => return,
                Type::Return => return,
                _ => self.advance()
            };
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::token::Type;

    #[test]
    fn test_binary() {
        let mut parser = Parser::new(vec![
            Token::new(Type::Number, "123".to_string(), Some(Literal::Number(123.0)), 1),
            Token::new(Type::Plus, "+".to_string(), None, 1),
            Token::new(Type::Number, "456".to_string(), Some(Literal::Number(456.0)), 1),
            Token::new(Type::EOF, "".to_string(), None, 1)
        ]);

        let expr = parser.expression().unwrap();

        assert_eq!(expr, Expr::Binary(BinaryData {
            left: Box::new(Expr::Literal(Literal::Number(123.0))),
            operator: Token::new(Type::Plus, "+".to_string(), None, 1),
            right: Box::new(Expr::Literal(Literal::Number(456.0)))
        }));
    }

    #[test]
    fn test_unary() {
        let mut parser = Parser::new(vec![
            Token::new(Type::Minus, "-".to_string(), None, 1),
            Token::new(Type::Number, "123".to_string(), Some(Literal::Number(123.0)), 1),
            Token::new(Type::EOF, "".to_string(), None, 1)
        ]);

        let expr = parser.expression().unwrap();

        assert_eq!(expr, Expr::Unary(UnaryData {
            operator: Token::new(Type::Minus, "-".to_string(), None, 1),
            expr: Box::new(Expr::Literal(Literal::Number(123.0)))
        }));
    }

    #[test]
    fn test_grouping() {
        let mut parser = Parser::new(vec![
            Token::new(Type::LeftParen, "(".to_string(), None, 1),
            Token::new(Type::Number, "123".to_string(), Some(Literal::Number(123.0)), 1),
            Token::new(Type::RightParen, ")".to_string(), None, 1),
            Token::new(Type::EOF, "".to_string(), None, 1)
        ]);

        let expr = parser.expression().unwrap();

        assert_eq!(expr, Expr::Grouping(GroupingData {
            expr: Box::new(Expr::Literal(Literal::Number(123.0)))
        }));
    }

    #[test]
    fn test_precedence() {
        let mut parser = Parser::new(vec![
            Token::new(Type::Number, "1".to_string(), Some(Literal::Number(1.0)), 1),
            Token::new(Type::Minus, "-".to_string(), None, 1),
            Token::new(Type::Number, "2".to_string(), Some(Literal::Number(2.0)), 1),
            Token::new(Type::Star, "*".to_string(), None, 1),
            Token::new(Type::Number, "3".to_string(), Some(Literal::Number(3.0)), 1),
            Token::new(Type::Plus, "+".to_string(), None, 1),
            Token::new(Type::Number, "4".to_string(), Some(Literal::Number(4.0)), 1),
            Token::new(Type::EOF, "".to_string(), None, 1)
        ]);

        let expr = parser.expression().unwrap();

        assert_eq!(expr, Expr::Binary(BinaryData {
            left: Box::new(Expr::Binary(BinaryData {
                left: Box::new(Expr::Literal(Literal::Number(1.0))),
                operator: Token::new(Type::Minus, "-".to_string(), None, 1),
                right: Box::new(Expr::Binary(BinaryData {
                    left: Box::new(Expr::Literal(Literal::Number(2.0))),
                    operator: Token::new(Type::Star, "*".to_string(), None, 1),
                    right: Box::new(Expr::Literal(Literal::Number(3.0)))
                }))
            })),
            operator: Token::new(Type::Plus, "+".to_string(), None, 1),
            right: Box::new(Expr::Literal(Literal::Number(4.0)))
        }));
    }
}

