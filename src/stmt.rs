use crate::expr::Expr;
use crate::token::Token;

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

#[derive(Debug, PartialEq)]
pub struct VarData {
    pub name: Token,
    pub initializer: Option<Expr>,
}

#[derive(Debug, PartialEq)]
pub struct BlockData {
    pub statements: Vec<Stmt>,
}

/// Represents a statement in the language
#[derive(Debug, PartialEq)]
pub enum Stmt {
    Expression(ExpressionData),
    Print(PrintData),
    Var(VarData),
    Block(BlockData),
}

impl Stmt {
    /// Accepts a visitor and returns the result of the visit.
    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
        match self {
            Stmt::Expression(_) => visitor.visit_expression_stmt(self),
            Stmt::Print(_) => visitor.visit_print_stmt(self),
            Stmt::Var(_) => visitor.visit_var_stmt(self),
            Stmt::Block(_) => visitor.visit_block_stmt(self),
        }
    }
}

pub trait StmtVisitor<T> {
    fn visit_expression_stmt(&mut self, stmt: &Stmt) -> T;
    fn visit_print_stmt(&mut self, stmt: &Stmt) -> T;
    fn visit_var_stmt(&mut self, stmt: &Stmt) -> T;
    fn visit_block_stmt(&mut self, stmt: &Stmt) -> T;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::{Literal, Type};
    use crate::ast::ASTPrinter;

    #[test]
    fn test_expression_stmt() {
        let expr = Expr::Literal(Literal::Number(1.0));
        let stmt = Stmt::Expression(ExpressionData { expr });

        let mut ast = ASTPrinter {};

        assert_eq!(stmt.accept(&mut ast), "(expr 1)");
    }

    #[test]
    fn test_print_stmt() {
        let expr = Expr::Literal(Literal::Number(1.0));
        let stmt = Stmt::Print(PrintData { expr });

        let mut ast = ASTPrinter;

        assert_eq!(stmt.accept(&mut ast), "(print 1)");
    }

    #[test]
    fn test_var_stmt_with_initializer() {
        let name = Token::new(Type::Identifier, "a".to_string(), None, 1);
        let initializer = Some(Expr::Literal(Literal::Number(1.0)));
        let stmt = Stmt::Var(VarData { name, initializer });

        let mut ast = ASTPrinter;

        assert_eq!(stmt.accept(&mut ast), "var a = 1");
    }

    #[test]
    fn test_var_stmt_without_initializer() {
        let name = Token::new(Type::Identifier, "a".to_string(), None, 1);
        let initializer = None;
        let stmt = Stmt::Var(VarData { name, initializer });

        let mut ast = ASTPrinter;

        assert_eq!(stmt.accept(&mut ast), "var a");
    }

    #[test]
    fn test_block_stmt() {
        let stmts = vec![
            Stmt::Expression(ExpressionData {
                expr: Expr::Literal(Literal::Number(1.0)),
            }),
            Stmt::Print(PrintData {
                expr: Expr::Literal(Literal::Number(2.0)),
            }),
        ];
        let stmt = Stmt::Block(BlockData { statements: stmts });

        let mut ast = ASTPrinter;

        assert_eq!(
            stmt.accept(&mut ast),
            r#"{ (expr 1) (print 2) }"#
        )
    }
}

