use crate::environment::Environment;
use crate::error::{rloxError, RuntimeError, self};
use crate::expr::{self, Expr, ExprVisitor};
use crate::stmt::{Stmt, StmtVisitor};
use crate::token::{Literal, Type};

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter { environment: Environment::new() }
    }

    pub fn interpret(&mut self, statements: &Vec<Stmt>) {
        for statement in statements {
            self.execute(statement);
        }
    }

    fn execute(&mut self, stmt: &Stmt) {
        stmt.accept(self)
    }

    fn evaluate(&mut self, expr: &Expr) -> Literal {
        expr.accept(self)
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl ExprVisitor<Literal> for Interpreter {
    fn visit_literal_expr(&mut self, literal: &Literal) -> Literal {
        literal.clone()
    }

    fn visit_unary_expr(&mut self, unary: &expr::UnaryData) -> Literal {
        let right = self.evaluate(&unary.expr);

        match unary.operator.r#type {
            Type::Minus => Literal::Number(-right.as_number()),
            Type::Bang => Literal::Bool(!right.as_bool()),
            _ => unreachable!(),
        }
    }

    fn visit_binary_expr(&mut self, binary: &expr::BinaryData) -> Literal {
        let left = self.evaluate(&binary.left);
        let right = self.evaluate(&binary.right);

        match binary.operator.r#type {
            Type::Greater       => Literal::Bool(left.as_number() > right.as_number()),
            Type::GreaterEqual  => Literal::Bool(left.as_number() >= right.as_number()),
            Type::Less          => Literal::Bool(left.as_number() < right.as_number()),
            Type::LessEqual     => Literal::Bool(left.as_number() <= right.as_number()),
            Type::EqualEqual    => Literal::Bool(left.as_number() == right.as_number()),
            Type::BangEqual     => Literal::Bool(left.as_number() != right.as_number()),
            Type::Slash         => Literal::Number(left.as_number() / right.as_number()),
            Type::Star          => Literal::Number(left.as_number() * right.as_number()),
            Type::Minus         => Literal::Number(left.as_number() - right.as_number()),
            Type::Plus          => match (left, right) {
                (Literal::Number(l), Literal::Number(r)) => Literal::Number(l + r),
                (Literal::String(l), Literal::String(r)) => Literal::String(l + &r),
                _ => {
                    RuntimeError {
                        token: binary.operator.clone(),
                        message: "Tried to add two unsupported types".to_string(),
                    }.throw();
                    Literal::Null
                }
            },
            _ => unreachable!(),
        }
    }

    fn visit_grouping_expr(&mut self, grouping: &expr::GroupingData) -> Literal {
        self.evaluate(&grouping.expr)
    }

    fn visit_variable_expr(&mut self, variable: &expr::VariableData) -> Literal {
        self.environment.get(&variable.name).unwrap_or_else(|error| {
            error.throw();
            Literal::Null
        })
    }
}

impl StmtVisitor<()> for Interpreter {
    fn visit_expression_stmt(&mut self, stmt: &Stmt) {
        let Stmt::Expression(data) = stmt else { unreachable!() };
        self.evaluate(&data.expr);
    }

    fn visit_print_stmt(&mut self, stmt: &Stmt) {
        let Stmt::Print(data) = stmt else { unreachable!() };
        let value = self.evaluate(&data.expr);

        // Make sure evaluate didn't throw an error
        if error::did_error() {
            return;
        }

        println!("{value}");
    }

    fn visit_var_stmt(&mut self, stmt: &Stmt) {
        let Stmt::Var(data) = stmt else { unreachable!() };
        let value = match &data.initializer {
            Some(value) => self.evaluate(value),
            None => Literal::Null,
        };

        self.environment.define(&data.name.lexeme, value);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::token::Token;

    #[test]
    fn evaluate_literal() {
        let mut interpreter = Interpreter::new();
        let expr = Expr::Literal(Literal::Number(12.0));
        assert_eq!(interpreter.evaluate(&expr), Literal::Number(12.0));
    }

    #[test]
    fn evaluate_unary() {
        let mut interpreter = Interpreter::new();
        let expr = Expr::Unary(expr::UnaryData {
            operator: Token::new(Type::Minus, String::from("-"), None, 1),
            expr: Box::new(Expr::Literal(Literal::Number(12.0))),
        });
        assert_eq!(interpreter.evaluate(&expr), Literal::Number(-12.0));
    }

    #[test]
    fn evaluate_binary() {
        let mut interpreter = Interpreter::new();
        let expr = Expr::Binary(expr::BinaryData {
            left: Box::new(Expr::Literal(Literal::Number(12.0))),
            operator: Token::new(Type::Minus, String::from("-"), None, 1),
            right: Box::new(Expr::Literal(Literal::Number(12.0))),
        });
        assert_eq!(interpreter.evaluate(&expr), Literal::Number(0.0));
    }

    #[test]
    fn evaluate_grouping() {
        let mut interpreter = Interpreter::new();
        let expr = Expr::Grouping(expr::GroupingData {
            expr: Box::new(Expr::Literal(Literal::Number(12.0))),
        });
        assert_eq!(interpreter.evaluate(&expr), Literal::Number(12.0));
    }

    #[test]
    fn evaluate_complex() {
        let mut interpreter = Interpreter::new();
        let expr = Expr::Binary(expr::BinaryData {
            left: Box::new(Expr::Literal(Literal::Number(6.0))),
            operator: Token::new(Type::Minus, String::from("-"), None, 1),
            right: Box::new(Expr::Binary(expr::BinaryData {
                left: Box::new(Expr::Literal(Literal::Number(12.0))),
                operator: Token::new(Type::Minus, String::from("-"), None, 1),
                right: Box::new(Expr::Literal(Literal::Number(24.0))),
            })),
        });
        assert_eq!(interpreter.evaluate(&expr), Literal::Number(18.0));
    }

    #[test]
    fn evaluate_string() {
        let mut interpreter = Interpreter::new();
        let expr = Expr::Binary(expr::BinaryData {
            left: Box::new(Expr::Literal(Literal::String(String::from("Hello")))),
            operator: Token::new(Type::Plus, String::from("+"), None, 1),
            right: Box::new(Expr::Literal(Literal::String(String::from("World")))),
        });
        assert_eq!(interpreter.evaluate(&expr), Literal::String(String::from("HelloWorld")));
    }

    #[test]
    fn evaluate_string_and_number() {
        let mut interpreter = Interpreter::new();
        let expr = Expr::Binary(expr::BinaryData {
            left: Box::new(Expr::Literal(Literal::String(String::from("Hello")))),
            operator: Token::new(Type::Plus, String::from("+"), None, 1),
            right: Box::new(Expr::Literal(Literal::Number(12.0))),
        });
        assert_eq!(interpreter.evaluate(&expr), Literal::Null);
    }

    #[test]
    fn evaluate_greater() {
        let mut interpreter = Interpreter::new();
        let expr = Expr::Binary(expr::BinaryData {
            left: Box::new(Expr::Literal(Literal::Number(12.0))),
            operator: Token::new(Type::Greater, String::from(">"), None, 1),
            right: Box::new(Expr::Literal(Literal::Number(12.0))),
        });
        assert_eq!(interpreter.evaluate(&expr), Literal::Bool(false));
    }

    #[test]
    fn evaluate_greater_equal() {
        let mut interpreter = Interpreter::new();
        let expr = Expr::Binary(expr::BinaryData {
            left: Box::new(Expr::Literal(Literal::Number(12.0))),
            operator: Token::new(Type::GreaterEqual, String::from(">="), None, 1),
            right: Box::new(Expr::Literal(Literal::Number(12.0))),
        });
        assert_eq!(interpreter.evaluate(&expr), Literal::Bool(true));
    }

    #[test]
    fn evaluate_less() {
        let mut interpreter = Interpreter::new();
        let expr = Expr::Binary(expr::BinaryData {
            left: Box::new(Expr::Literal(Literal::Number(12.0))),
            operator: Token::new(Type::Less, String::from("<"), None, 1),
            right: Box::new(Expr::Literal(Literal::Number(12.0))),
        });
        assert_eq!(interpreter.evaluate(&expr), Literal::Bool(false));
    }

    #[test]
    fn evaluate_less_equal() {
        let mut interpreter = Interpreter::new();
        let expr = Expr::Binary(expr::BinaryData {
            left: Box::new(Expr::Literal(Literal::Number(12.0))),
            operator: Token::new(Type::LessEqual, String::from("<="), None, 1),
            right: Box::new(Expr::Literal(Literal::Number(12.0))),
        });
        assert_eq!(interpreter.evaluate(&expr), Literal::Bool(true));
    }

    #[test]
    fn evaluate_equal() {
        let mut interpreter = Interpreter::new();
        let expr_true = Expr::Binary(expr::BinaryData {
            left: Box::new(Expr::Literal(Literal::Number(12.0))),
            operator: Token::new(Type::EqualEqual, String::from("=="), None, 1),
            right: Box::new(Expr::Literal(Literal::Number(12.0))),
        });
        assert_eq!(interpreter.evaluate(&expr_true), Literal::Bool(true));

        let expr_false = Expr::Binary(expr::BinaryData {
            left: Box::new(Expr::Literal(Literal::Number(12.0))),
            operator: Token::new(Type::EqualEqual, String::from("=="), None, 1),
            right: Box::new(Expr::Literal(Literal::Number(13.0))),
        });
        assert_eq!(interpreter.evaluate(&expr_false), Literal::Bool(false));
    }

    #[test]
    fn evaluate_not_equal() {
        let mut interpreter = Interpreter::new();
        let expr = Expr::Binary(expr::BinaryData {
            left: Box::new(Expr::Literal(Literal::Number(12.0))),
            operator: Token::new(Type::BangEqual, String::from("!="), None, 1),
            right: Box::new(Expr::Literal(Literal::Number(12.0))),
        });
        assert_eq!(interpreter.evaluate(&expr), Literal::Bool(false));
    }
}

