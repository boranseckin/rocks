use crate::expr::Expr;
use crate::function::Function;
use crate::token::Token;

/// Represents an expression statement's data in the language
#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionData {
    pub expr: Expr,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IfData {
    pub condition: Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}

/// Represents a print statement's data in the language
#[derive(Debug, PartialEq, Clone)]
pub struct PrintData {
    pub expr: Expr,
}

#[derive(Debug, PartialEq, Clone)]
pub struct VarData {
    pub name: Token,
    pub initializer: Option<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct WhileData {
    pub condition: Expr,
    pub body: Box<Stmt>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BlockData {
    pub statements: Vec<Stmt>,
}

/// Represents a statement in the language
#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    Expression(ExpressionData),
    Function(Function),
    If(IfData),
    Print(PrintData),
    Var(VarData),
    While(WhileData),
    Block(BlockData),
}

impl Stmt {
    /// Accepts a visitor and returns the result of the visit.
    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
        match self {
            Stmt::Expression(_) => visitor.visit_expression_stmt(self),
            Stmt::Function(_) => visitor.visit_function_stmt(self),
            Stmt::If(_) => visitor.visit_if_stmt(self),
            Stmt::Print(_) => visitor.visit_print_stmt(self),
            Stmt::Var(_) => visitor.visit_var_stmt(self),
            Stmt::While(_) => visitor.visit_while_stmt(self),
            Stmt::Block(_) => visitor.visit_block_stmt(self),
        }
    }
}

pub trait StmtVisitor<T> {
    fn visit_expression_stmt(&mut self, stmt: &Stmt) -> T;
    fn visit_function_stmt(&mut self, stmt: &Stmt) -> T;
    fn visit_if_stmt(&mut self, stmt: &Stmt) -> T;
    fn visit_print_stmt(&mut self, stmt: &Stmt) -> T;
    fn visit_var_stmt(&mut self, stmt: &Stmt) -> T;
    fn visit_while_stmt(&mut self, stmt: &Stmt) -> T;
    fn visit_block_stmt(&mut self, stmt: &Stmt) -> T;
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::token::Type;
    use crate::literal::Literal;
    use crate::ast::ASTPrinter;

    #[test]
    fn test_expression_stmt() {
        let expr = Expr::Literal(Literal::Number(1.0));
        let stmt = Stmt::Expression(ExpressionData { expr });

        let mut ast = ASTPrinter {};

        assert_eq!(stmt.accept(&mut ast), "(expr 1)");
    }

    #[test]
    fn test_if_stmt() {
        let condition = Expr::Literal(Literal::Number(1.0));
        let then_branch = Stmt::Expression(ExpressionData {
            expr: Expr::Literal(Literal::Number(2.0)),
        });
        let else_branch = Some(Box::new(Stmt::Expression(ExpressionData {
            expr: Expr::Literal(Literal::Number(3.0)),
        })));
        let stmt = Stmt::If(IfData {
            condition,
            then_branch: Box::new(then_branch),
            else_branch,
        });

        let mut ast = ASTPrinter;

        assert_eq!(stmt.accept(&mut ast), "(if 1 (expr 2) else (expr 3))");
    }

    #[test]
    fn test_if_stmt_without_else() {
        let condition = Expr::Literal(Literal::Number(1.0));
        let then_branch = Stmt::Expression(ExpressionData {
            expr: Expr::Literal(Literal::Number(2.0)),
        });
        let else_branch = None;
        let stmt = Stmt::If(IfData {
            condition,
            then_branch: Box::new(then_branch),
            else_branch,
        });

        let mut ast = ASTPrinter;

        assert_eq!(stmt.accept(&mut ast), "(if 1 (expr 2))");
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

        assert_eq!(stmt.accept(&mut ast), "(var a = 1)");
    }

    #[test]
    fn test_var_stmt_without_initializer() {
        let name = Token::new(Type::Identifier, "a".to_string(), None, 1);
        let initializer = None;
        let stmt = Stmt::Var(VarData { name, initializer });

        let mut ast = ASTPrinter;

        assert_eq!(stmt.accept(&mut ast), "(var a)");
    }

    #[test]
    fn test_while_stmt() {
        let condition = Expr::Literal(Literal::Bool(true));
        let body = Stmt::Expression(ExpressionData {
            expr: Expr::Literal(Literal::Number(2.0)),
        });
        let stmt = Stmt::While(WhileData {
            condition,
            body: Box::new(body),
        });

        let mut ast = ASTPrinter;

        assert_eq!(stmt.accept(&mut ast), "(while true (expr 2))");
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

        assert_eq!(stmt.accept(&mut ast), "{ (expr 1) (print 2) }")
    }

    #[test]
    fn test_function_stmt() {
        let name = Token::new(Type::Identifier, "a".to_string(), None, 1);
        let params = vec![Token::new(Type::Identifier, "b".to_string(), None, 1)];
        let body = vec![Stmt::Expression(ExpressionData {
            expr: Expr::Literal(Literal::Number(1.0)),
        })];
        let stmt = Stmt::Function(Function {
            name,
            params,
            body,
        });

        let mut ast = ASTPrinter;

        assert_eq!(stmt.accept(&mut ast), "(fun a(b) { (expr 1) })");
    }
}
