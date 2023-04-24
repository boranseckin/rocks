use crate::token::Token;
use crate::literal::Literal;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LogicalData {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

/// Represents a unary expression's data in the language.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UnaryData {
    pub operator: Token,
    pub expr: Box<Expr>,
}

/// Represents a binary expression's data in the language.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BinaryData {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

/// Represents a grouping expression's data in the language.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GroupingData {
    pub expr: Box<Expr>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VariableData {
    pub name: Token,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AssignData {
    pub name: Token,
    pub value: Box<Expr>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CallData {
    pub callee: Box<Expr>,
    pub paren: Token,
    pub arguments: Vec<Expr>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GetData {
    pub object: Box<Expr>,
    pub name: Token,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SetData {
    pub object: Box<Expr>,
    pub name: Token,
    pub value: Box<Expr>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ThisData {
    pub keyword: Token,
}

/// Represents an expression in the language.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expr {
    Literal(Literal), // Literal is defined in token.rs
    Logical(LogicalData),
    Unary(UnaryData),
    Binary(BinaryData),
    Grouping(GroupingData),
    Variable(VariableData),
    Assign(AssignData),
    Call(CallData),
    Get(GetData),
    Set(SetData),
    This(ThisData),
}

impl Expr {
    /// Accepts a visitor and returns the result of the visit.
    pub fn accept<T>(&self, visitor: &mut impl ExprVisitor<T>) -> T {
        use Expr::*;

        match self {
            Literal(_) => visitor.visit_literal_expr(self),
            Logical(_) => visitor.visit_logical_expr(self),
            Unary(_) => visitor.visit_unary_expr(self),
            Binary(_) => visitor.visit_binary_expr(self),
            Grouping(_) => visitor.visit_grouping_expr(self),
            Variable(_) => visitor.visit_variable_expr(self),
            Assign(_) => visitor.visit_assign_expr(self),
            Call(_) => visitor.visit_call_expr(self),
            Get(_) => visitor.visit_get_expr(self),
            Set(_) => visitor.visit_set_expr(self),
            This(_) => visitor.visit_this_expr(self),
        }
    }
}

pub trait ExprVisitor<T> {
    fn visit_literal_expr(&mut self, expr: &Expr) -> T;
    fn visit_logical_expr(&mut self, expr: &Expr) -> T;
    fn visit_unary_expr(&mut self, expr: &Expr) -> T;
    fn visit_binary_expr(&mut self, expr: &Expr) -> T;
    fn visit_grouping_expr(&mut self, expr: &Expr) -> T;
    fn visit_variable_expr(&mut self, expr: &Expr) -> T;
    fn visit_assign_expr(&mut self, expr: &Expr) -> T;
    fn visit_call_expr(&mut self, expr: &Expr) -> T;
    fn visit_get_expr(&mut self, expr: &Expr) -> T;
    fn visit_set_expr(&mut self, expr: &Expr) -> T;
    fn visit_this_expr(&mut self, expr: &Expr) -> T;
}
