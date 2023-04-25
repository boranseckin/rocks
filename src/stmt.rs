use crate::expr::Expr;
use crate::token::Token;

/// Represents an expression statement's data in the language
#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionData {
    pub expr: Expr,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionData {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
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
pub struct ReturnData {
    pub keyword: Token,
    pub value: Option<Expr>,
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

#[derive(Debug, PartialEq, Clone)]
pub struct ClassData {
    pub name: Token,
    pub superclass: Option<Expr>,
    pub methods: Vec<Stmt>,
}

/// Represents a statement in the language
#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    Expression(ExpressionData),
    Function(FunctionData),
    If(IfData),
    Print(PrintData),
    Return(ReturnData),
    Var(VarData),
    While(WhileData),
    Block(BlockData),
    Class(ClassData),
}

impl Stmt {
    /// Accepts a visitor and returns the result of the visit.
    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
        use Stmt::*;

        match self {
            Expression(_) => visitor.visit_expression_stmt(self),
            Function(_) => visitor.visit_function_stmt(self),
            If(_) => visitor.visit_if_stmt(self),
            Print(_) => visitor.visit_print_stmt(self),
            Return(_) => visitor.visit_return_stmt(self),
            Var(_) => visitor.visit_var_stmt(self),
            While(_) => visitor.visit_while_stmt(self),
            Block(_) => visitor.visit_block_stmt(self),
            Class(_) => visitor.visit_class_stmt(self),
        }
    }
}

pub trait StmtVisitor<T> {
    fn visit_expression_stmt(&mut self, stmt: &Stmt) -> T;
    fn visit_function_stmt(&mut self, stmt: &Stmt) -> T;
    fn visit_if_stmt(&mut self, stmt: &Stmt) -> T;
    fn visit_print_stmt(&mut self, stmt: &Stmt) -> T;
    fn visit_return_stmt(&mut self, stmt: &Stmt) -> T;
    fn visit_var_stmt(&mut self, stmt: &Stmt) -> T;
    fn visit_while_stmt(&mut self, stmt: &Stmt) -> T;
    fn visit_block_stmt(&mut self, stmt: &Stmt) -> T;
    fn visit_class_stmt(&mut self, stmt: &Stmt) -> T;
}
