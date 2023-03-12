use std::rc::Rc;
use std::cell::RefCell;

use crate::environment::Environment;
use crate::error::{rloxError, RuntimeError, self};
use crate::expr::{self, Expr, ExprVisitor};
use crate::stmt::{Stmt, StmtVisitor};
use crate::token::{Literal, Type};

pub struct Interpreter {
    // Interior mutability with multiple owners
    environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter { environment: Rc::new(RefCell::new(Environment::new(None))) }
    }

    pub fn interpret(&mut self, statements: &Vec<Stmt>) {
        for statement in statements {
            self.execute(statement);
        }
    }

    // TODO: Add return result
    fn execute(&mut self, stmt: &Stmt) {
        stmt.accept(self)
    }

    fn execute_block(
        &mut self,
        statements: &Vec<Stmt>,
        environment: Rc<RefCell<Environment>>
    ) {
        let previous = self.environment.clone();
        self.environment = environment;

        for statement in statements {
            self.execute(statement);
        }

        self.environment = previous;
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

    fn visit_logical_expr(&mut self, logical: &expr::LogicalData) -> Literal {
        let left = self.evaluate(&logical.left);

        match logical.operator.r#type {
            Type::Or => if left.as_bool() { return left },
            Type::And => if !left.as_bool() { return left },
            _ => unreachable!(),
        };

        self.evaluate(&logical.right)
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
        self.environment
            .borrow()
            .get(&variable.name)
            .unwrap_or_else(|error| {
                error.throw();
                Literal::Null
            })
    }

    fn visit_assign_expr(&mut self, assign: &expr::AssignData) -> Literal {
        let value = self.evaluate(&assign.value);
        self.environment.borrow_mut().assign(&assign.name, value.to_owned());
        value
    }
}

impl StmtVisitor<()> for Interpreter {
    fn visit_expression_stmt(&mut self, stmt: &Stmt) {
        let Stmt::Expression(data) = stmt else { unreachable!() };
        self.evaluate(&data.expr);
    }

    fn visit_if_stmt(&mut self, stmt: &Stmt) {
        let Stmt::If(data) = stmt else { unreachable!() };
        if self.evaluate(&data.condition).as_bool() {
            self.execute(&data.then_branch);
        } else if let Some(else_branch) = &data.else_branch {
            self.execute(else_branch);
        }
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

        self.environment.borrow_mut().define(&data.name.lexeme, value);
    }

    fn visit_while_stmt(&mut self, stmt: &Stmt) {
        let Stmt::While(data) = stmt else { unreachable!() };
        while self.evaluate(&data.condition).as_bool() {
            self.execute(&data.body);
        }
    }

    fn visit_block_stmt(&mut self, stmt: &Stmt) {
        let Stmt::Block(data) = stmt else { unreachable!() };
        self.execute_block(
            &data.statements,
            Rc::new(RefCell::new(Environment::new(Some(Rc::clone(&self.environment)))))
        );
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
    fn evaluate_logical() {
        let mut interpreter = Interpreter::new();
        let expr = Expr::Logical(expr::LogicalData {
            left: Box::new(Expr::Literal(Literal::Bool(true))),
            operator: Token::new(Type::And, String::from("and"), None, 1),
            right: Box::new(Expr::Literal(Literal::Bool(true))),
        });
        assert_eq!(interpreter.evaluate(&expr), Literal::Bool(true));
    }

    #[test]
    fn evaluate_logical_short_circuit() {
        let mut interpreter = Interpreter::new();
        let expr = Expr::Logical(expr::LogicalData {
            left: Box::new(Expr::Literal(Literal::Bool(false))),
            operator: Token::new(Type::And, String::from("and"), None, 1),
            right: Box::new(Expr::Literal(Literal::Bool(true))),
        });
        assert_eq!(interpreter.evaluate(&expr), Literal::Bool(false));
    }

    #[test]
    fn evaluate_logical_nested() {
        let mut interpreter = Interpreter::new();
        let expr = Expr::Logical(expr::LogicalData {
            left: Box::new(Expr::Literal(Literal::Bool(true))),
            operator: Token::new(Type::Or, String::from("or"), None, 1),
            right: Box::new(Expr::Logical(expr::LogicalData {
                left: Box::new(Expr::Literal(Literal::Bool(true))),
                operator: Token::new(Type::And, String::from("and"), None, 1),
                right: Box::new(Expr::Literal(Literal::Bool(true))),
            })),
        });
        assert_eq!(interpreter.evaluate(&expr), Literal::Bool(true));
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

    #[test]
    fn evaluate_assign() {
        let mut interpreter = Interpreter::new();
        interpreter.environment.borrow_mut().define("a", Literal::Number(0.0));
        let expr = Expr::Assign(expr::AssignData {
            name: Token::new(Type::Identifier, String::from("a"), None, 1),
            value: Box::new(Expr::Literal(Literal::Number(12.0))),
        });
        assert_eq!(interpreter.evaluate(&expr), Literal::Number(12.0));
        assert_eq!(
            interpreter.environment.borrow().get(&Token::new(Type::Identifier, String::from("a"), None, 1)).unwrap(),
            Literal::Number(12.0)
        );
    }
}

