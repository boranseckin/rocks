use crate::expr::Expr;

/// Represents an expression statement's data in the language
#[derive(Debug, PartialEq)]
pub struct ExpressionData {
    pub expr: Expr,
}

/// Represents a print statement's data in the language
#[derive(Debug, PartialEq)]
pub struct PrintData {
    pub expr: Expr,
}

/// Represents a statement in the language
#[derive(Debug, PartialEq)]
pub enum Stmt {
    Expression(ExpressionData),
    Print(PrintData),
}

impl Stmt {
    /// Accepts a visitor and returns the result of the visit.
    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
        match self {
            Stmt::Expression(_) => visitor.visit_expression_stmt(self),
            Stmt::Print(_) => visitor.visit_print_stmt(self),
        }
    }
}

pub trait StmtVisitor<T> {
    fn visit_expression_stmt(&mut self, stmt: &Stmt) -> T;
    fn visit_print_stmt(&mut self, stmt: &Stmt) -> T;
}
