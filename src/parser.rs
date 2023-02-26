use crate::error::{rloxError, ParseError};
use crate::token::{Token, Type, Literal};
use crate::expr::{Expr, BinaryData, UnaryData, GroupingData, VariableData};
use crate::stmt::{Stmt, PrintData, ExpressionData, VarData};

type ParseResult = Result<Expr, ParseError>;

/// Parses the tokens and returns the resulting expression.
///
/// - Program     -> Decleration* EOF ;
/// - Decleration -> VarDecl | Statement ;
/// - Statement   -> ExprStmt | PrintStmt ;
/// - VarDecl     -> "var" IDENTIFIER ( "=" Expression )? ";" ;
/// - ExprStmt    -> Expression ";" ;
/// - PrintStmt   -> "print" Expression ";" ;
/// - Expression  -> Equality ;
/// - Equality    -> Comparison ( ( "!=" | "==" ) Comparison )* ;
/// - Comparison  -> Term ( ( ">" | ">=" | "<" | "<=" ) Term )* ;
/// - Term        -> Factor ( ( "+" | "-" ) Factor )* ;
/// - Factor      -> Unary ( ( "*" | "/" ) Unary )* ;
/// - Unary       -> ( "!" | "-" ) Unary | Primary ;
/// - Primary     -> NUMBER | STRING | false | true | null | "(" Expression ")" | IDENTIFIER ;
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
    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            if let Some(stmt) = self.decleration() {
                statements.push(stmt);
            }
        }

        statements
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

        Err(ParseError {
            token: self.previous().clone(),
            message: message.to_string(),
        }) 
    }

    /// Parses a decleration.
    fn decleration(&mut self) -> Option<Stmt> {
        let statement = if self.matches(vec![Type::Var]) {
            self.var_decleration()
        } else {
            self.statement()
        };

        match statement {
            Ok(stmt) => Some(stmt),
            Err(error) => {
                error.throw();
                self.synchronize();
                None
            }
        }
    }

    /// Parses a variable decleration.
    fn var_decleration(&mut self) -> Result<Stmt, ParseError> {
        let name = self.consume(Type::Identifier, "Expect variable name")?.clone();

        let mut initializer: Option<Expr> = None;
        if self.matches(vec![Type::Equal]) {
            match self.expression() {
                Ok(expr) => initializer = Some(expr),
                Err(error) => return Err(error),
            };
        }

        self.consume(Type::Semicolon, "Expect ';' after variable decleration")?;
        Ok(Stmt::Var(VarData { name, initializer }))
    }

    /// Parses an expression.
    fn expression(&mut self) -> ParseResult {
        self.equality()
    }

    /// Parses a statement.
    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.matches(vec![Type::Print]) {
            return self.print_statement();
        }

        self.expression_statement()
    }

    /// Parses a print statement.
    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = match self.expression() {
            Ok(expr) => expr,
            Err(error) => return Err(error),
        };

        self.consume(Type::Semicolon, "Expect ';' after value")?;

        Ok(Stmt::Print(PrintData { expr }))
    }

    /// Parses an expression statement.
    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = match self.expression() {
            Ok(expr) => expr,
            Err(error) => return Err(error),
        };

        self.consume(Type::Semicolon, "Expect ';' after expression")?;

        Ok(Stmt::Expression(ExpressionData { expr }))
    }

    /// Parses an equality expression.
    fn equality(&mut self) -> ParseResult {
        let mut expr = match self.comparison() {
            Ok(expr) => expr,
            Err(error) => return Err(error),
        };

        while self.matches(vec![Type::BangEqual, Type::EqualEqual]) {
            let operator = self.previous().clone();
            let right = match self.comparison() {
                Ok(expr) => expr,
                Err(error) => return Err(error),
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
            Err(error) => return Err(error),
        };

        while self.matches(vec![Type::Greater, Type::GreaterEqual, Type::Less, Type::LessEqual]) {
            let operator = self.previous().clone();
            let right = match self.term() {
                Ok(expr) => expr,
                Err(error) => return Err(error),
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
            Err(error) => return Err(error),
        };

        while self.matches(vec![Type::Minus, Type::Plus]) {
            let operator = self.previous().clone();
            let right = match self.factor() {
                Ok(expr) => expr,
                Err(error) => return Err(error),
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
            Err(error) => return Err(error),
        };

        while self.matches(vec![Type::Slash, Type::Star]) {
            let operator = self.previous().clone();
            let right = match self.unary() {
                Ok(expr) => expr,
                Err(error) => return Err(error),
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
                Err(error) => return Err(error),
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

        if self.matches(vec![Type::Identifier]) {
            return Ok(Expr::Variable(VariableData {
                name: self.previous().clone()
            }))
        }

        if self.matches(vec![Type::LeftParen]) {
            let expr = match self.expression() {
                Ok(expr) => expr,
                Err(error) => return Err(error),
            };

            match self.consume(Type::RightParen, "Expected ')' after expression") {
                Ok(_) => (),
                Err(error) => return Err(error),
            };

            return Ok(Expr::Grouping(GroupingData { expr: Box::new(expr) }));
        }

        Err(ParseError {
            token: self.peek().clone(),
            message: "Expected expression".to_string()
        })
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

    #[test]
    fn test_print_stmt() {
        let mut parser = Parser::new(vec![
            Token::new(Type::Print, "print".to_string(), None, 1),
            Token::new(Type::Number, "123".to_string(), Some(Literal::Number(123.0)), 1),
            Token::new(Type::Semicolon, ";".to_string(), None, 1),
            Token::new(Type::EOF, "".to_string(), None, 1)
        ]);

        let stmt = parser.statement().unwrap();

        assert_eq!(stmt, Stmt::Print(PrintData {
            expr: Expr::Literal(Literal::Number(123.0))
        }));
    }

    #[test]
    fn test_expression_stmt() {
        let mut parser = Parser::new(vec![
            Token::new(Type::Number, "123".to_string(), Some(Literal::Number(123.0)), 1),
            Token::new(Type::Semicolon, ";".to_string(), None, 1),
            Token::new(Type::EOF, "".to_string(), None, 1)
        ]);

        let stmt = parser.statement().unwrap();

        assert_eq!(stmt, Stmt::Expression(ExpressionData {
            expr: Expr::Literal(Literal::Number(123.0))
        }));
    }

    #[test]
    fn test_var_stmt() {
        let mut parser = Parser::new(vec![
            Token::new(Type::Var, "var".to_string(), None, 1),
            Token::new(Type::Identifier, "a".to_string(), None, 1),
            Token::new(Type::Equal, "=".to_string(), None, 1),
            Token::new(Type::Number, "123".to_string(), Some(Literal::Number(123.0)), 1),
            Token::new(Type::Semicolon, ";".to_string(), None, 1),
            Token::new(Type::EOF, "".to_string(), None, 1)
        ]);

        parser.advance();
        let stmt = parser.var_decleration().unwrap();

        assert_eq!(stmt, Stmt::Var(VarData {
            name: Token::new(Type::Identifier, "a".to_string(), None, 1),
            initializer: Some(Expr::Literal(Literal::Number(123.0)))
        }));
    }

    #[test]
    fn test_decleration() {
        let mut parser = Parser::new(vec![
            Token::new(Type::Var, "var".to_string(), None, 1),
            Token::new(Type::Identifier, "a".to_string(), None, 1),
            Token::new(Type::Equal, "=".to_string(), None, 1),
            Token::new(Type::Number, "123".to_string(), Some(Literal::Number(123.0)), 1),
            Token::new(Type::Semicolon, ";".to_string(), None, 1),
            Token::new(Type::EOF, "".to_string(), None, 1)
        ]);

        let stmt = parser.decleration().unwrap();

        assert_eq!(stmt, Stmt::Var(VarData {
            name: Token::new(Type::Identifier, "a".to_string(), None, 1),
            initializer: Some(Expr::Literal(Literal::Number(123.0)))
        }));
    }
}

