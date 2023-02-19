use crate::token::{Token, Literal};

#[derive(Debug, PartialEq)]
pub struct UnaryData {
    pub operator: Token,
    pub expr: Box<Expr>,
}

#[derive(Debug, PartialEq)]
pub struct BinaryData {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, PartialEq)]
pub struct GroupingData {
    pub expr: Box<Expr>
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Literal(Literal),
    Unary(UnaryData),
    Binary(BinaryData),
    Grouping(GroupingData),
}

impl Expr {
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

    #[test]
    fn create_literal() {
        let expr = Expr::Literal(Literal::Number(12.0));
        assert!(matches!(expr, Expr::Literal(Literal::Number(22.0))))
    }

    #[test]
    fn create_unary() {
        let expr = Expr::Unary(UnaryData {
            operator: Token::new(Type::Minus, String::from("-"), None, 1),
            expr: Box::new(Expr::Literal(Literal::Number(12.0))),
        });

        assert!(matches!(expr, Expr::Unary(UnaryData { operator: Token { r#type: Type::Minus, .. }, expr: Box::new(Expr::Literal(Literal::Number(12.0))) })))
    }
}

