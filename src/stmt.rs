use crate::expr::Expr;
use crate::token::Token;

/// Represents an [`expression`](Stmt::Expression) statement's data in the language.
#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionData {
    pub expr: Expr,
}

/// Represents a [`function`](Stmt::Function) statement's data in the language.
#[derive(Debug, PartialEq, Clone)]
pub struct FunctionData {
    /// The function's name.
    pub name: Token,
    /// The function's parameters.
    pub params: Vec<Token>,
    /// The function's body.
    pub body: Vec<Stmt>,
}

/// Represents an [`if`](Stmt::If) statement's data in the language.
#[derive(Debug, PartialEq, Clone)]
pub struct IfData {
    /// The condition to check.
    pub condition: Expr,
    /// The statement to execute if the condition is true.
    pub then_branch: Box<Stmt>,
    /// The statement to execute if the condition is false (optional).
    pub else_branch: Option<Box<Stmt>>,
}

/// Represents a [`print`](Stmt::Print) statement's data in the language.
#[derive(Debug, PartialEq, Clone)]
pub struct PrintData {
    /// The expression to print.
    pub expr: Expr,
}

/// Represents a [`return`](Stmt::Return) statement's data in the language.
#[derive(Debug, PartialEq, Clone)]
pub struct ReturnData {
    /// The 'return' keyword.
    pub keyword: Token,
    /// The value to return (optional).
    pub value: Option<Expr>,
}

/// Represents a [`break`](Stmt::Break) statement's data in the language.
#[derive(Debug, PartialEq, Clone)]
pub struct BreakData {
    pub keyword: Token,
}

/// Represents a [`var`](Stmt::Var) statement's data in the language.
#[derive(Debug, PartialEq, Clone)]
pub struct VarData {
    /// The variable's name.
    pub name: Token,
    /// The variable's initializer (optional).
    /// If the initializer is `None`, the variable is initialized to `null`.
    pub initializer: Option<Expr>,
}

/// Represents a [`while`](Stmt::While) statement's data in the language.
#[derive(Debug, PartialEq, Clone)]
pub struct WhileData {
    /// The condition to check.
    pub condition: Expr,
    /// The statement to execute while the condition is true.
    pub body: Box<Stmt>,
}

/// Represents a [`block`](Stmt::Block) statement's data in the language.
#[derive(Debug, PartialEq, Clone)]
pub struct BlockData {
    /// The statements in the block.
    pub statements: Vec<Stmt>,
}

/// Represents a [`class`](Stmt::Class) statement's data in the language.
#[derive(Debug, PartialEq, Clone)]
pub struct ClassData {
    /// The class's name.
    pub name: Token,
    /// The class's superclass (optional).
    pub superclass: Option<Expr>,
    /// The class's methods.
    pub methods: Vec<Stmt>,
}

/// Represents a statement in the language.
#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    /// An [`expression`](crate::expr::Expr) statement.
    /// This is the only statement that evaluates to a value.
    Expression(ExpressionData),

    /// A function statement.
    /// This is used to construct a [`function`](crate::function::Function)
    /// (or a [`method`](crate::class::Class)) value.
    Function(FunctionData),

    /// An if statement.
    /// This is used to conditionally execute a statement.
    If(IfData),

    /// A print statement.
    /// This is used to print a value to the standard output.
    Print(PrintData),

    /// A return statement.
    /// This is used to return from a function (optionally with a value).
    Return(ReturnData),

    /// A break statement.
    /// This is used to break out of a loop.
    Break(BreakData),

    /// A var statement.
    /// This is used to declare a variable.
    Var(VarData),

    /// A while statement.
    /// This is used to conditionally execute a statement repeatedly.
    While(WhileData),

    /// A block statement.
    /// This is used to group statements together like a function body.
    Block(BlockData),

    /// A class statement.
    /// This is used to declare a class.
    Class(ClassData),
}

impl Stmt {
    /// Accepts a visitor and returns the result of the visit.
    /// This is used to implement the visitor pattern.
    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
        use Stmt::*;

        match self {
            Expression(_) => visitor.visit_expression_stmt(self),
            Function(_) => visitor.visit_function_stmt(self),
            If(_) => visitor.visit_if_stmt(self),
            Print(_) => visitor.visit_print_stmt(self),
            Return(_) => visitor.visit_return_stmt(self),
            Break(_) => visitor.visit_break_stmt(self),
            Var(_) => visitor.visit_var_stmt(self),
            While(_) => visitor.visit_while_stmt(self),
            Block(_) => visitor.visit_block_stmt(self),
            Class(_) => visitor.visit_class_stmt(self),
        }
    }
}

/// A visitor for statements.
pub trait StmtVisitor<T> {
    fn visit_expression_stmt(&mut self, stmt: &Stmt) -> T;
    fn visit_function_stmt(&mut self, stmt: &Stmt) -> T;
    fn visit_if_stmt(&mut self, stmt: &Stmt) -> T;
    fn visit_print_stmt(&mut self, stmt: &Stmt) -> T;
    fn visit_return_stmt(&mut self, stmt: &Stmt) -> T;
    fn visit_break_stmt(&mut self, stmt: &Stmt) -> T;
    fn visit_var_stmt(&mut self, stmt: &Stmt) -> T;
    fn visit_while_stmt(&mut self, stmt: &Stmt) -> T;
    fn visit_block_stmt(&mut self, stmt: &Stmt) -> T;
    fn visit_class_stmt(&mut self, stmt: &Stmt) -> T;
}
