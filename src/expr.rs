use crate::token::Token;
use crate::literal::Literal;

/// Represents a [`logical`](Expr::Logical) expression's data in the language.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LogicalData {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

/// Represents an [`unary`](Expr::Unary) expression's data in the language.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UnaryData {
    pub operator: Token,
    pub expr: Box<Expr>,
}

/// Represents a [`binary`](Expr::Binary) expression's data in the language.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BinaryData {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

/// Represents a [`grouping`](Expr::Grouping) expression's data in the language.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GroupingData {
    pub expr: Box<Expr>,
}

/// Represents a [`variable`](Expr::Variable) expression's data in the language.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VariableData {
    pub name: Token,
}

/// Represents an [`assign`](Expr::Assign) expression's data in the language.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AssignData {
    pub name: Token,
    pub value: Box<Expr>,
}

/// Represents a [`call`](Expr::Call) expression's data in the language.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CallData {
    pub callee: Box<Expr>,
    pub paren: Token,
    pub arguments: Vec<Expr>,
}

/// Represents a [`get`](Expr::Get) expression's data in the language.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GetData {
    pub object: Box<Expr>,
    pub name: Token,
}

/// Represents a [`set`](Expr::Set) expression's data in the language.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SetData {
    pub object: Box<Expr>,
    pub name: Token,
    pub value: Box<Expr>,
}

/// Represents a [`this`](Expr::This) expression's data in the language.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ThisData {
    pub keyword: Token,
}

/// Represents a [`super`](Expr::Super) expression's data in the language.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SuperData {
    pub keyword: Token,
    pub method: Token,
}

/// Represents an expression in the language.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expr {
    /// A [`literal`](crate::literal::Literal) value.
    /// - `1`
    /// - `"hello"`
    /// - `true`
    /// - `null`
    Literal(Literal),

    /// A logical expression.
    /// - `true and false`
    /// - `1 or "hello"`
    Logical(LogicalData),

    /// An unary expression.
    /// - `-1`
    /// - `!true`
    Unary(UnaryData),

    /// A binary expression.
    /// - `1 + 2`
    /// - `1 != 2`
    /// - `1 <= 2`
    /// - `1 / 2`
    Binary(BinaryData),

    /// A grouping expression.
    /// - `(1 + 2)`
    /// - `(true and false) or (1 <= 2)`
    /// - `((1 + 2) * 3) / 4`
    Grouping(GroupingData),

    /// A variable expression.
    /// - `x`
    Variable(VariableData),

    /// An assignment expression.
    /// - `x = 1`
    /// - `x = "hello"`
    /// - `x = func()`
    Assign(AssignData),

    /// A call expression.
    /// - `func()`
    /// - `func(arg1, 23)`
    /// - `instance.method()`
    Call(CallData),

    /// A get expression.
    /// - `instance.property`
    /// - `instance.property.method()`
    Get(GetData),

    /// A set expression.
    /// - `instance.property = 1`
    /// - `instance.property = "hello"`
    Set(SetData),

    /// A this expression.
    /// - `this`
    /// - `this.property`
    This(ThisData),

    /// A super expression.
    /// - `super.method()`
    /// - `super.method(arg1, 23)`
    Super(SuperData),
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
            Super(_) => visitor.visit_super_expr(self),
        }
    }
}

// TODO: Add error handling.
/// A visitor for expressions.
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
    fn visit_super_expr(&mut self, expr: &Expr) -> T;
}
