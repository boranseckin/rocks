use crate::token::{Token, Literal};

/// Represents a unary expression in the language.
#[derive(Debug, PartialEq)]
pub struct UnaryData {
    pub operator: Token,
    pub expr: Box<Expr>,
}

/// Represents a binary expression in the language.
#[derive(Debug, PartialEq)]
pub struct BinaryData {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

/// Represents a grouping expression in the language.
#[derive(Debug, PartialEq)]
pub struct GroupingData {
    pub expr: Box<Expr>
}

/// Represents an expression in the language.
#[derive(Debug, PartialEq)]
pub enum Expr {
    Literal(Literal), // Literal is defined in token.rs
    Unary(UnaryData),
    Binary(BinaryData),
    Grouping(GroupingData),
}

impl Expr {
    /// Accepts a visitor and returns the result of the visit.
    pub fn accept<T>(&self, visitor: &mut impl ExprVisitor<T>) -> T {
        use Expr::*;

        match self {
            Literal(args) => visitor.visit_literal_expr(args),
            Unary(args) => visitor.visit_unary_expr(args),
            Binary(args) => visitor.visit_binary_expr(args),
            Grouping(args) => visitor.visit_grouping_expr(args),
        }
    }
}

pub trait ExprVisitor<T> {
    fn visit_literal_expr(&mut self, literal: &Literal) -> T;
    fn visit_unary_expr(&mut self, unary: &UnaryData) -> T;
    fn visit_binary_expr(&mut self, binary: &BinaryData) -> T;
    fn visit_grouping_expr(&mut self, grouping: &GroupingData) -> T;
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
}

