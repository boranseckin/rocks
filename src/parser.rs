use crate::parse_error;
use crate::token::{Token, Type, Literal};
use crate::expr::{Expr, BinaryData, UnaryData, GroupingData};

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

    pub fn parse(&mut self) -> Option<Expr> {
        match self.expression() {
            Ok(expr) => Some(expr),
            Err(_) => None,
        }
    }

    fn peek(&mut self) -> &Token {
        &self.tokens[self.current as usize]
    }

    fn previous(&mut self) -> &Token {
        &self.tokens[(self.current - 1) as usize]
    }

    fn is_at_end(&mut self) -> bool {
        self.peek().r#type == Type::EOF
    }

    fn check(&mut self, r#type: Type) -> bool {
        if self.is_at_end() {
            return false
        }

        self.peek().r#type == r#type
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn matches(&mut self, types: Vec<Type>) -> bool {
        for r#type in types {
            if self.check(r#type) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn consume(&mut self, r#type: Type, message: &str) -> Result<&Token, ParseError> {
        if self.check(r#type) {
            return Ok(self.advance());
        }

        error(self.peek(), message);
        Err(ParseError {})
    }

    fn expression(&mut self) -> ParseResult {
        self.equality()
    }

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

            expr = Expr::Binary(BinaryData { left: Box::new(expr), operator, right: Box::new(right) });
        }

        Ok(expr)
    }

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
            expr = Expr::Binary(BinaryData { left: Box::new(expr), operator, right: Box::new(right) });
        }

        Ok(expr)
    }

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
            expr = Expr::Binary(BinaryData { left: Box::new(expr), operator, right: Box::new(right) });
        }

        Ok(expr)
    }

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
            expr = Expr::Binary(BinaryData { left: Box::new(expr), operator, right: Box::new(right) });
        }

        Ok(expr)
    }

    fn unary(&mut self) -> ParseResult {
        if self.matches(vec![Type::Bang, Type::Minus]) {
            let operator = self.previous().clone();
            let right = match self.unary() {
                Ok(expr) => expr,
                Err(ParseError) => return Err(ParseError),
            };
            return Ok(Expr::Unary(UnaryData { operator, expr: Box::new(right) }));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.matches(vec![Type::False]) {
            return Ok(Expr::Literal(Literal::Bool(false)));
        }

        if self.matches(vec![Type::True]) {
            return Ok(Expr::Literal(Literal::Bool(true)));
        }

        if self.matches(vec![Type::Nil]) {
            return Ok(Expr::Literal(Literal::Nil));
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

#[derive(Debug, Clone)]
struct ParseError;

type ParseResult = Result<Expr, ParseError>;

fn error(token: &Token, message: &str) {
    parse_error(token, message);
}

