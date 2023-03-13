use crate::token::Token;
use crate::literal::Literal;

#[derive(Debug, PartialEq, Clone)]
pub struct LogicalData {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

/// Represents a unary expression's data in the language.
#[derive(Debug, PartialEq, Clone)]
pub struct UnaryData {
    pub operator: Token,
    pub expr: Box<Expr>,
}

/// Represents a binary expression's data in the language.
#[derive(Debug, PartialEq, Clone)]
pub struct BinaryData {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

/// Represents a grouping expression's data in the language.
#[derive(Debug, PartialEq, Clone)]
pub struct GroupingData {
    pub expr: Box<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableData {
    pub name: Token,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AssignData {
    pub name: Token,
    pub value: Box<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CallData {
    pub callee: Box<Expr>,
    pub paren: Token,
    pub arguments: Vec<Expr>,
}

/// Represents an expression in the language.
#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Literal(Literal), // Literal is defined in token.rs
    Logical(LogicalData),
    Unary(UnaryData),
    Binary(BinaryData),
    Grouping(GroupingData),
    Variable(VariableData),
    Assign(AssignData),
    Call(CallData),
}

impl Expr {
    /// Accepts a visitor and returns the result of the visit.
    pub fn accept<T>(&self, visitor: &mut impl ExprVisitor<T>) -> T {
        use Expr::*;

        match self {
            Literal(args) => visitor.visit_literal_expr(args),
            Logical(args) => visitor.visit_logical_expr(args),
            Unary(args) => visitor.visit_unary_expr(args),
            Binary(args) => visitor.visit_binary_expr(args),
            Grouping(args) => visitor.visit_grouping_expr(args),
            Variable(args) => visitor.visit_variable_expr(args),
            Assign(args) => visitor.visit_assign_expr(args),
            Call(args) => visitor.visit_call_expr(args),
        }
    }
}

pub trait ExprVisitor<T> {
    fn visit_literal_expr(&mut self, literal: &Literal) -> T;
    fn visit_logical_expr(&mut self, logical: &LogicalData) -> T;
    fn visit_unary_expr(&mut self, unary: &UnaryData) -> T;
    fn visit_binary_expr(&mut self, binary: &BinaryData) -> T;
    fn visit_grouping_expr(&mut self, grouping: &GroupingData) -> T;
    fn visit_variable_expr(&mut self, variable: &VariableData) -> T;
    fn visit_assign_expr(&mut self, assign: &AssignData) -> T;
    fn visit_call_expr(&mut self, call: &CallData) -> T;
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{token::Type, ast::ASTPrinter};

    #[test]
    fn create_literal() {
        let expr = Expr::Literal(Literal::Number(12.0)); 
        let _literal = Literal::Number(12.0);
        assert!(matches!(expr, Expr::Literal(Literal::Number(_literal))))
    }

    #[test]
    fn create_logical() {
        let expr = Expr::Logical(LogicalData {
            left: Box::new(Expr::Literal(Literal::Bool(true))),
            operator: Token::new(Type::And, "and".to_string(), None, 1),
            right: Box::new(Expr::Literal(Literal::Bool(false))),
        });

        let _literal = Literal::Bool(true);
        let _literal2 = Literal::Bool(false);

        if let Expr::Logical(data) = expr {
            assert_eq!(data.operator.r#type, Type::And);
            assert_eq!(data.operator.lexeme, "and");
            assert_eq!(data.operator.line, 1);
            assert!(matches!(*data.left, Expr::Literal(Literal::Bool(_literal))));
            assert!(matches!(*data.right, Expr::Literal(Literal::Bool(_literal2))));
        } else {
            panic!("Expected logical expression");
        }
    }

    #[test]
    fn create_unary() {
        let expr = Expr::Unary(UnaryData {
            operator: Token::new(Type::Minus, String::from("-"), None, 1),
            expr: Box::new(Expr::Literal(Literal::Number(12.0))),
        });

        let _literal = Literal::Number(12.0);

        if let Expr::Unary(data) = expr {
            assert_eq!(data.operator.r#type, Type::Minus);
            assert_eq!(data.operator.lexeme, "-");
            assert_eq!(data.operator.line, 1);
            assert!(matches!(*data.expr, Expr::Literal(Literal::Number(_literal))));
        } else {
            panic!("Expected unary expression");
        }
    }

    #[test]
    fn create_binary() {
        let expr = Expr::Binary(BinaryData {
            left: Box::new(Expr::Literal(Literal::Number(12.0))),
            operator: Token::new(Type::Minus, String::from("-"), None, 1),
            right: Box::new(Expr::Literal(Literal::Number(12.0))),
        });

        let _literal = Literal::Number(12.0);

        if let Expr::Binary(data) = expr {
            assert_eq!(data.operator.r#type, Type::Minus);
            assert_eq!(data.operator.lexeme, "-");
            assert_eq!(data.operator.line, 1);
            assert!(matches!(*data.left, Expr::Literal(Literal::Number(_literal))));
            assert!(matches!(*data.right, Expr::Literal(Literal::Number(_literal))));
        } else {
            panic!("Expected binary expression");
        }
    }

    #[test]
    fn create_grouping() {
        let expr = Expr::Grouping(GroupingData {
            expr: Box::new(Expr::Literal(Literal::Number(12.0))),
        });

        let _literal = Literal::Number(12.0);

        if let Expr::Grouping(data) = expr {
            assert!(matches!(*data.expr, Expr::Literal(Literal::Number(_literal))));
        } else {
            panic!("Expected grouping expression");
        }
    }

    #[test]
    fn accept_literal() {
        let expr_num = Expr::Literal(Literal::Number(12.4));
        let expr_str = Expr::Literal(Literal::String(String::from("hello")));

        let mut ast = ASTPrinter {};

        assert_eq!(expr_num.accept(&mut ast), "12.4");
        assert_eq!(expr_str.accept(&mut ast), "hello");
    }

    #[test]
    fn accept_logical() {
        let expr = Expr::Logical(LogicalData {
            left: Box::new(Expr::Literal(Literal::Bool(true))),
            operator: Token::new(Type::Or, String::from("or"), None, 1),
            right: Box::new(Expr::Literal(Literal::Bool(false))),
        });

        let mut ast = ASTPrinter {};

        assert_eq!(expr.accept(&mut ast), "(or true false)");
    }

    #[test]
    fn accept_unary() {
        let expr = Expr::Unary(UnaryData {
            operator: Token::new(Type::Bang, String::from("!"), None, 1),
            expr: Box::new(Expr::Literal(Literal::Bool(false))),
        });

        let mut ast = ASTPrinter {};

        assert_eq!(expr.accept(&mut ast), "(! false)");
    }

    #[test]
    fn accept_binary() {
        let expr = Expr::Binary(BinaryData {
            left: Box::new(Expr::Literal(Literal::Number(53.6))),
            operator: Token::new(Type::Minus, String::from("-"), None, 1),
            right: Box::new(Expr::Literal(Literal::Number(23.3))),
        });

        let mut ast = ASTPrinter {};

        assert_eq!(expr.accept(&mut ast), "(- 53.6 23.3)");
    }

    #[test]
    fn accept_grouping() {
        let expr = Expr::Grouping(GroupingData {
            expr: Box::new(Expr::Literal(Literal::Null)),
        });

        let mut ast = ASTPrinter {};

        assert_eq!(expr.accept(&mut ast), "(group null)");
    }

    #[test]
    fn accept_nested() {
        let expr = Expr::Binary(BinaryData {
            left: Box::new(Expr::Unary(UnaryData {
                operator: Token::new(Type::Bang, String::from("!"), None, 1),
                expr: Box::new(Expr::Literal(Literal::Bool(false))),
            })),
            operator: Token::new(Type::Minus, String::from("-"), None, 1),
            right: Box::new(Expr::Literal(Literal::Number(23.3))),
        });

        let mut ast = ASTPrinter {};

        assert_eq!(expr.accept(&mut ast), "(- (! false) 23.3)");
    }

    #[test]
    fn accept_nested_grouping() {
        let expr = Expr::Binary(BinaryData {
            left: Box::new(Expr::Grouping(GroupingData {
                expr: Box::new(Expr::Literal(Literal::Number(53.6))),
            })),
            operator: Token::new(Type::Minus, String::from("-"), None, 1),
            right: Box::new(Expr::Literal(Literal::Number(23.3))),
        });

        let mut ast = ASTPrinter {};

        assert_eq!(expr.accept(&mut ast), "(- (group 53.6) 23.3)");
    }

    #[test]
    fn accept_nested_grouping2() {
        let expr = Expr::Binary(BinaryData {
            left: Box::new(Expr::Grouping(GroupingData {
                expr: Box::new(Expr::Literal(Literal::Number(53.6))),
            })),
            operator: Token::new(Type::Minus, String::from("-"), None, 1),
            right: Box::new(Expr::Grouping(GroupingData {
                expr: Box::new(Expr::Literal(Literal::Number(23.3))),
            })),
        });

        let mut ast = ASTPrinter {};

        assert_eq!(expr.accept(&mut ast), "(- (group 53.6) (group 23.3))");
    }

    #[test]
    fn accept_nested_grouping3() {
        let expr = Expr::Binary(BinaryData {
            left: Box::new(Expr::Grouping(GroupingData {
                expr: Box::new(Expr::Binary(BinaryData {
                    left: Box::new(Expr::Literal(Literal::Number(53.6))),
                    operator: Token::new(Type::Minus, String::from("-"), None, 1),
                    right: Box::new(Expr::Literal(Literal::Number(23.3))),
                })),
            })),
            operator: Token::new(Type::Minus, String::from("-"), None, 1),
            right: Box::new(Expr::Grouping(GroupingData {
                expr: Box::new(Expr::Literal(Literal::Number(23.3))),
            })),
        });

        let mut ast = ASTPrinter {};

        assert_eq!(expr.accept(&mut ast), "(- (group (- 53.6 23.3)) (group 23.3))");
    }

    #[test]
    fn accept_nested_grouping4() {
        let expr = Expr::Binary(BinaryData {
            left: Box::new(Expr::Grouping(GroupingData {
                expr: Box::new(Expr::Binary(BinaryData {
                    left: Box::new(Expr::Literal(Literal::Number(53.6))),
                    operator: Token::new(Type::Minus, String::from("-"), None, 1),
                    right: Box::new(Expr::Literal(Literal::Number(23.3))),
                })),
            })),
            operator: Token::new(Type::Minus, String::from("-"), None, 1),
            right: Box::new(Expr::Grouping(GroupingData {
                expr: Box::new(Expr::Binary(BinaryData {
                    left: Box::new(Expr::Literal(Literal::Number(53.6))),
                    operator: Token::new(Type::Minus, String::from("-"), None, 1),
                    right: Box::new(Expr::Literal(Literal::Number(23.3))),
                })),
            })),
        });

        let mut ast = ASTPrinter {};

        assert_eq!(
            expr.accept(&mut ast),
            "(- (group (- 53.6 23.3)) (group (- 53.6 23.3)))"
        );
    }

    #[test]
    fn accept_variable() {
        let expr = Expr::Variable(VariableData {
            name: Token::new(Type::Identifier, String::from("a"), None, 1),
        });

        let mut ast = ASTPrinter {};

        assert_eq!(expr.accept(&mut ast), "a");
    }

    #[test]
    fn accept_assign() {
        let expr = Expr::Assign(AssignData {
            name: Token::new(Type::Identifier, String::from("a"), None, 1),
            value: Box::new(Expr::Literal(Literal::Number(23.3))),
        });

        let mut ast = ASTPrinter {};

        assert_eq!(expr.accept(&mut ast), "(= a 23.3)");
    }

    #[test]
    fn accept_call() {
        let expr = Expr::Call(CallData {
            callee: Box::new(Expr::Variable(VariableData {
                name: Token::new(Type::Identifier, String::from("a"), None, 1),
            })),
            paren: Token::new(Type::RightParen, String::from(")"), None, 1),
            arguments: vec![],
        });

        let mut ast = ASTPrinter {};

        assert_eq!(expr.accept(&mut ast), "a()");
    }

    #[test]
    fn accept_call_with_argument() {
        let expr = Expr::Call(CallData {
            callee: Box::new(Expr::Variable(VariableData {
                name: Token::new(Type::Identifier, String::from("a"), None, 1),
            })),
            paren: Token::new(Type::RightParen, String::from(")"), None, 1),
            arguments: vec![Expr::Literal(Literal::Number(23.3))],
        });

        let mut ast = ASTPrinter {};

        assert_eq!(expr.accept(&mut ast), "a(23.3)");
    }
}

