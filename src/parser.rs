use crate::error::{Error, ParseError};
use crate::token::{Token, Type};
use crate::literal::Literal;
use crate::expr::*;
use crate::stmt::*;

type ParseResult<T> = Result<T, ParseError>;

/// Returns if the next token is any of the given types.
macro_rules! matches {
    ( $self:ident, $( $type:expr ),+ ) => {
        {
            if $( $self.check($type) ) ||* {
                $self.advance();
                true
            } else {
                false
            }
        }
    }
}

/// Parses the tokens and returns the resulting expression.
///
/// - Program     -> Decleration* EOF ;
/// - Block       -> "{" Decleration* "}" ;
/// - Decleration -> ClassDecl | FunDecl | VarDecl | Statement ;
/// - ClassDecl   -> "class" IDENTIFIER ( "<" IDENTIFIER )? "{" Function* "}" ;
/// - FunDecl     -> "fun" Function ;
/// - VarDecl     -> "var" IDENTIFIER ( "=" Expression )? ";" ;
/// - Function    -> IDENTIFIER "(" Parameters? ")" Block ;
/// - Parameters  -> IDENTIFIER ( "," IDENTIFIER )* ;
/// - Statement   -> ExprStmt | ForStmt | IfStmt | PrintStmt | ReturnStmt | WhileStmt | Block ;
/// - ExprStmt    -> Expression ";" ;
/// - ForStmt     -> "for" "(" ( Decleration | ExprStmt | ";" ) Expression? ";" Expression? ")" Statement ;
/// - IfStmt      -> "if" "(" Expression ")" Statement ( "else" Statement )? ;
/// - PrintStmt   -> "print" Expression ";" ;
/// - ReturnStmt  -> "return" Expression? ";" ;
/// - WhileStmt   -> "while" "(" Expression ")" Statement ;
/// - Expression  -> Assignment ;
/// - Assignment  -> ( Call "." )? IDENTIFIER "=" Assignment | LogicOr ;
/// - LogicOr     -> LogicAnd ( "or" LogicAnd )* ;
/// - LogicAnd    -> Equality ( "and" Equality )* ;
/// - Equality    -> Comparison ( ( "!=" | "==" ) Comparison )* ;
/// - Comparison  -> Term ( ( ">" | ">=" | "<" | "<=" ) Term )* ;
/// - Term        -> Factor ( ( "+" | "-" ) Factor )* ;
/// - Factor      -> Unary ( ( "*" | "/" ) Unary )* ;
/// - Unary       -> ( "!" | "-" ) Unary | Primary ;
/// - Arguments   -> Expression ( "," Expression )* ;
/// - Call        -> Primary ( "(" Arguments? ")" | "." IDENTIFIER )* ;
/// - Primary     -> NUMBER | STRING | "false" | "true" | "null" | "this" | "(" Expression ")" | IDENTIFIER | "super" "." IDENTIFIER ;
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

    /// Consumes the next token if it is of the given type.
    fn consume(&mut self, r#type: Type, message: &str) -> ParseResult<&Token> {
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
        let statement = if matches!(self, Type::Class) {
           self.class_decleration()
        } else if matches!(self, Type::Fun) {
            self.function("function")
        } else if matches!(self, Type::Var) {
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

    /// Parses a class decleration
    fn class_decleration(&mut self) -> ParseResult<Stmt> {
        let name = self.consume(Type::Identifier, "Expect class name")?.clone();

        let superclass = if matches!(self, Type::Less) {
            self.consume(Type::Identifier, "Expect superclass name")?;
            Some(Expr::Variable(VariableData { name: self.previous().clone() }))
        } else {
            None
        };

        self.consume(Type::LeftBrace, "Expect '{' before class body")?;

        let mut methods: Vec<Stmt> = vec![];
        while !self.check(Type::RightBrace) && !self.is_at_end() {
            methods.push(self.function("method")?);
        }

        self.consume(Type::RightBrace, "Expect '}' after class body")?;

        Ok(Stmt::Class(ClassData { name, superclass, methods }))
    }

    /// Parses a variable decleration.
    fn var_decleration(&mut self) -> ParseResult<Stmt> {
        let name = self.consume(Type::Identifier, "Expect variable name")?.clone();

        let mut initializer: Option<Expr> = None;
        if matches!(self, Type::Equal) {
            match self.expression() {
                Ok(expr) => initializer = Some(expr),
                Err(error) => return Err(error),
            };
        }

        self.consume(Type::Semicolon, "Expect ';' after variable decleration")?;
        Ok(Stmt::Var(VarData { name, initializer }))
    }

    /// Parses a while statement.
    fn while_statement(&mut self) -> ParseResult<Stmt> {
        self.consume(Type::LeftParen, "Expect '(' after while.")?;
        let condition = self.expression()?;
        self.consume(Type::RightParen, "Expect ')' after condition.")?;
        let body = self.statement()?;

        Ok(Stmt::While(WhileData {
            condition,
            body: Box::new(body),
        }))
    }

    /// Parses an expression.
    fn expression(&mut self) -> ParseResult<Expr> {
        self.assignment()
    }

    /// Parses a statement.
    fn statement(&mut self) -> ParseResult<Stmt> {
        if matches!(self, Type::For) {
            return self.for_statement();
        }

        if matches!(self, Type::If) {
            return self.if_statement();
        }

        if matches!(self, Type::Print) {
            return self.print_statement();
        }

        if matches!(self, Type::Return) {
            return self.return_statement();
        }

        if matches!(self, Type::While) {
            return self.while_statement();
        }

        if matches!(self, Type::LeftBrace) {
            return Ok(Stmt::Block(BlockData { statements: self.block()? }));
        }

        self.expression_statement()
    }

    /// Parses a for statement.
    fn for_statement(&mut self) -> ParseResult<Stmt> {
        self.consume(Type::LeftParen, "Expect '(' after 'for'")?;

        let initializer: Option<Stmt>;
        if matches!(self, Type::Semicolon) {
            initializer = None;
        } else if matches!(self, Type::Var) {
            initializer = Some(self.var_decleration()?);
        } else {
            initializer = Some(self.expression_statement()?);
        }

        let condition = match !self.check(Type::Semicolon) {
            true => Some(self.expression()?),
            false => None,
        };
        self.consume(Type::Semicolon, "Expect ';' after loop condition")?;

        let increment = match !self.check(Type::RightParen) {
            true => Some(self.expression()?),
            false => None,
        };
        self.consume(Type::RightParen, "Expect ')' after loop clauses")?;

        let mut body = self.statement()?;

        // Execute the increment after the body.
        if let Some(increment) = increment {
            body = Stmt::Block(BlockData {
                statements: vec![
                    body,
                    Stmt::Expression(ExpressionData {
                        expr: increment
                    }),
                ],
            });
        }

        // Wrap the body into a while loop.
        // If there is no condition, use true.
        body = Stmt::While(WhileData {
            condition: condition.unwrap_or(Expr::Literal(Literal::Bool(true))),
            body: Box::new(body),
        });

        // Add the initializer before the loop if there is one.
        if let Some(initializer) = initializer {
            body = Stmt::Block(BlockData {
                statements: vec![
                    initializer,
                    body,
                ],
            });
        }

        Ok(body)
    }

    /// Parses an if statement.
    fn if_statement(&mut self) -> ParseResult<Stmt> {
        self.consume(Type::LeftParen, "Expect '(' after 'if'")?;
        let condition = self.expression()?;
        self.consume(Type::RightParen, "Expect ')' after if condition")?;

        let then_branch = Box::new(self.statement()?);
        let mut else_branch: Option<Box<Stmt>> = None;
        if matches!(self, Type::Else) {
            else_branch = Some(Box::new(self.statement()?));
        }

        Ok(Stmt::If(IfData { condition, then_branch, else_branch }))
    }

    /// Parses a print statement.
    fn print_statement(&mut self) -> ParseResult<Stmt> {
        let expr = match self.expression() {
            Ok(expr) => expr,
            Err(error) => return Err(error),
        };

        self.consume(Type::Semicolon, "Expect ';' after value")?;

        Ok(Stmt::Print(PrintData { expr }))
    }

    /// Parses a return statement.
    fn return_statement(&mut self) -> ParseResult<Stmt> {
        let keyword = self.previous().to_owned();

        let value = match self.check(Type::Semicolon) {
            true => None,
            false => Some(self.expression()?),
        };

        self.consume(Type::Semicolon, "Expect ';' after return value")?;
        Ok(Stmt::Return(ReturnData { keyword, value }))
    }

    /// Parses an expression statement.
    fn expression_statement(&mut self) -> ParseResult<Stmt> {
        let expr = match self.expression() {
            Ok(expr) => expr,
            Err(error) => return Err(error),
        };

        self.consume(Type::Semicolon, "Expect ';' after expression")?;

        Ok(Stmt::Expression(ExpressionData { expr }))
    }

    /// Parses a function decleration.
    fn function(&mut self, kind: &str) -> ParseResult<Stmt> {
        let name = self.consume(Type::Identifier, &format!("Expect {kind} name"))?.to_owned();

        self.consume(Type::LeftParen, &format!("Expect '(' after {kind} name"))?;

        let mut params = vec![];

        if !self.check(Type::RightParen) {
            loop {
                if params.len() >= 255 {
                    return Err(ParseError {
                        token: self.peek().to_owned(),
                        message: "Cannot have more than 255 parameters".to_string(),
                    });
                }

                params.push(self.consume(Type::Identifier, "Expect parameter name")?.to_owned());

                if !matches!(self, Type::Comma) {
                    break;
                }
            }
        }

        self.consume(Type::RightParen, "Expect ')' after parameters")?;

        self.consume(Type::LeftBrace, &format!("Expect '{{' before {kind} body"))?;

        let body = self.block()?;

        Ok(Stmt::Function(FunctionData { name, params, body }))
    }

    /// Parses a block statement.
    fn block(&mut self) -> ParseResult<Vec<Stmt>> {
        let mut statements = Vec::new();

        while !self.check(Type::RightBrace) && !self.is_at_end() {
            if let Some(stmt) = self.decleration() {
                statements.push(stmt);
            }
        }

        self.consume(Type::RightBrace, "Expect '}' after block")?;

        Ok(statements)
    }

    /// Parses an assignment expression.
    fn assignment(&mut self) -> ParseResult<Expr> {
        let expr = self.or()?;

        if matches!(self, Type::Equal) {
            let equals = self.previous().to_owned();
            let value = self.assignment()?;

            if let Expr::Variable(data) = expr {
                let name = data.name;

                return Ok(Expr::Assign(AssignData {
                    name,
                    value: Box::new(value)
                }));
            } else if let Expr::Get(data) = expr {
                return Ok(Expr::Set(SetData {
                    object: data.object,
                    name: data.name,
                    value: Box::new(value),
                }));
            }

            ParseError {
                token: equals,
                message: "Invalid assignment target".to_string()
            }.throw();
        }

        Ok(expr)
    }

    /// Parses an or expression.
    fn or(&mut self) -> ParseResult<Expr> {
        let mut expr = self.and()?;

        while matches!(self, Type::Or) {
            let operator = self.previous().clone();
            let right = self.and()?;
            expr = Expr::Logical(LogicalData {
                left: Box::new(expr),
                operator,
                right: Box::new(right)
            });
        }

        Ok(expr)
    }

    /// Parses and and expression.
    fn and(&mut self) -> ParseResult<Expr> {
        let mut expr = self.equality()?;

        while matches!(self, Type::And) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            expr = Expr::Logical(LogicalData {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    /// Parses an equality expression.
    fn equality(&mut self) -> ParseResult<Expr> {
        let mut expr = match self.comparison() {
            Ok(expr) => expr,
            Err(error) => return Err(error),
        };

        while matches!(self, Type::BangEqual, Type::EqualEqual) {
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
    fn comparison(&mut self) -> ParseResult<Expr> {
        let mut expr = match self.term() {
            Ok(expr) => expr,
            Err(error) => return Err(error),
        };

        while matches!(self, Type::Greater, Type::GreaterEqual, Type::Less, Type::LessEqual) {
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
    fn term(&mut self) -> ParseResult<Expr> {
        let mut expr = match self.factor() {
            Ok(expr) => expr,
            Err(error) => return Err(error),
        };

        while matches!(self, Type::Minus, Type::Plus) {
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
    fn factor(&mut self) -> ParseResult<Expr> {
        let mut expr = match self.unary() {
            Ok(expr) => expr,
            Err(error) => return Err(error),
        };

        while matches!(self, Type::Slash, Type::Star) {
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
    fn unary(&mut self) -> ParseResult<Expr> {
        if matches!(self, Type::Bang, Type::Minus) {
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

        self.call()
    }

    /// Parses a call arguments.
    fn finish_call(&mut self, callee: &Expr) -> ParseResult<Expr> {
        let mut arguments = vec![];

        if !self.check(Type::RightParen) {
            while { 
                if arguments.len() >= 255 {
                    ParseError {
                        token: self.peek().to_owned(),
                        message: "Cannot have more than 255 arguments".to_string(),
                    }.throw();
                }

                arguments.push(self.expression()?);
                matches!(self, Type::Comma)
            } {}
        }

        let paren = self.consume(Type::RightParen, "Expect ')' after arguments")?;

        Ok(Expr::Call(CallData {
            callee: Box::new(callee.to_owned()),
            paren: paren.to_owned(),
            arguments,
        }))
    }

    /// Parses a call expression.
    fn call(&mut self) -> ParseResult<Expr> {
        let mut expr = self.primary()?;

        loop {
            if matches!(self, Type::LeftParen) {
                expr = self.finish_call(&expr)?;
            } else if matches!(self, Type::Dot) {
                let name = self.consume(Type::Identifier, "Expected property name after '.'")?;
                expr = Expr::Get(GetData { object: Box::new(expr), name: name.clone() });
            } else {
                break;
            }
        }

        Ok(expr)
    }

    /// Parses a primary expression.
    fn primary(&mut self) -> ParseResult<Expr> {
        if matches!(self, Type::False) {
            return Ok(Expr::Literal(Literal::Bool(false)));
        }

        if matches!(self, Type::True) {
            return Ok(Expr::Literal(Literal::Bool(true)));
        }

        if matches!(self, Type::Null) {
            return Ok(Expr::Literal(Literal::Null));
        }

        if matches!(self, Type::Number, Type::String) {
            return Ok(Expr::Literal(self.previous().clone().literal
                .expect("number or string to have a literal value")));
        }

        if matches!(self, Type::Super) { 
            let keyword = self.previous().clone();
            self.consume(Type::Dot, "Expect '.' after 'super'")?;
            let method = self.consume(Type::Identifier, "Expect superclass method name")?.clone();

            return Ok(Expr::Super(SuperData { keyword, method }))
        }

        if matches!(self, Type::This) {
            return Ok(Expr::This(ThisData { keyword: self.previous().clone() }));
        }

        if matches!(self, Type::Identifier) {
            return Ok(Expr::Variable(VariableData {
                name: self.previous().clone()
            }))
        }

        if matches!(self, Type::LeftParen) {
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

    /// Tries to recover from a parse error.
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
