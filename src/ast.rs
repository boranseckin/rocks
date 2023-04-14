use crate::literal::Literal;
use crate::expr::{ExprVisitor, Expr};
use crate::stmt::{StmtVisitor, Stmt};

/// Returns a string representation of the expression in paranthesize.
macro_rules! parenthesize {
    ( $self:ident, $name:expr, $( $x:expr ),+ ) => {
        {
            let mut string = String::new();
            string += "(";
            string += $name;
            $(
                string += " ";
                string += &$x.accept($self);
            )*
            string += ")";

            string
        }
    };
}

pub struct ASTPrinter;

impl ASTPrinter {
    /// Prints the expression using visitor pattern.
    pub fn print(&mut self, expr: Expr) -> String {
        expr.accept(self)
    }
}

impl ExprVisitor<String> for ASTPrinter {
    fn visit_literal_expr(&mut self, expr: &Expr) -> String {
        let Expr::Literal(literal) = expr else { unreachable!() };
        literal.to_string()
    }

    fn visit_logical_expr(&mut self, expr: &Expr) -> String {
        let Expr::Logical(data) = expr else { unreachable!() };
        parenthesize!(self, &data.operator.lexeme, &data.left, &data.right)
    }

    fn visit_unary_expr(&mut self, expr: &Expr) -> String {
        let Expr::Unary(data) = expr else { unreachable!() };
        parenthesize!(self, &data.operator.lexeme, &data.expr)
    }

    fn visit_binary_expr(&mut self, expr: &Expr) -> String {
        let Expr::Binary(data) = expr else { unreachable!() };
        parenthesize!(self, &data.operator.lexeme, &data.left, &data.right)
    }

    fn visit_grouping_expr(&mut self, expr: &Expr) -> String {
        let Expr::Grouping(data) = expr else { unreachable!() };
        parenthesize!(self, "group", data.expr)
    }

    fn visit_variable_expr(&mut self, expr: &Expr) -> String {
        let Expr::Variable(data) = expr else { unreachable!() };
        data.name.lexeme.clone()
    }

    fn visit_assign_expr(&mut self, expr: &Expr) -> String {
        let Expr::Assign(data) = expr else { unreachable!() };
        parenthesize!(self, format!("= {}", &data.name.lexeme).as_str(), data.value)
    }

    fn visit_call_expr(&mut self, expr: &Expr) -> String {
        let Expr::Call(data) = expr else { unreachable!() };
        let mut string = String::new();
        string += &data.callee.accept(self);
        string += "(";
        for arg in &data.arguments {
            string += &arg.accept(self);
            string += " ";
        }
        string = string.trim_end().to_string();
        string += ")";
        string
    }

    fn visit_get_expr(&mut self, expr: &Expr) -> String {
        let Expr::Get(data) = expr else { unreachable!() };
        parenthesize!(self, "get", data.object)
    }

    fn visit_set_expr(&mut self, _expr: &Expr) -> String {
        todo!()
    }

    fn visit_this_expr(&mut self, _expr: &Expr) -> String {
        "this".to_string()
    }
}

impl StmtVisitor<String> for ASTPrinter {
    fn visit_expression_stmt(&mut self, stmt: &Stmt) -> String {
        let Stmt::Expression(data) = stmt else { unreachable!() };
        parenthesize!(self, "expr", data.expr)
    }

    fn visit_function_stmt(&mut self, stmt: &Stmt) -> String {
        let Stmt::Function(data) = stmt else { unreachable!() };
        let mut string = String::new();
        string += "(fun ";
        string += &data.name.lexeme;
        string += "(";
        for param in &data.params {
            string += &param.lexeme;
            string += " ";
        }
        string = string.trim_end().to_string();
        string += ") { ";
        string += &data.body.iter()
            .map(|stmt| { stmt.accept(self) })
            .collect::<Vec<String>>()
            .join(" ");
        string += " })";
        string
    }

    fn visit_if_stmt(&mut self, stmt: &Stmt) -> String {
        let Stmt::If(data) = stmt else { unreachable!() };
        let mut string = String::new();
        string += "(if ";
        string += &data.condition.accept(self);
        string += " ";
        string += &data.then_branch.accept(self);
        if let Some(else_branch) = &data.else_branch {
            string += " else ";
            string += &else_branch.accept(self);
        }
        string += ")";
        string
    }

    fn visit_print_stmt(&mut self, stmt: &Stmt) -> String {
        let Stmt::Print(data) = stmt else { unreachable!() };
        parenthesize!(self, "print", data.expr)
    }

    fn visit_return_stmt(&mut self, stmt: &Stmt) -> String {
        let Stmt::Return(data) = stmt else { unreachable!() };
        parenthesize!(self, "return", data.value.clone().unwrap_or(Expr::Literal(Literal::Null)))
    }

    fn visit_var_stmt(&mut self, stmt: &Stmt) -> String {
        let Stmt::Var(data) = stmt else { unreachable!() };
        let mut string = String::new();
        string += "(var ";
        string += &data.name.lexeme;
        if let Some(initializer) = &data.initializer {
            string += " = ";
            string += &initializer.accept(self);
        }
        string += ")";
        string
    }

    fn visit_while_stmt(&mut self, stmt: &Stmt) -> String {
        let Stmt::While(data) = stmt else { unreachable!() };
        parenthesize!(self, "while", data.condition, data.body)
    }

    fn visit_block_stmt(&mut self, stmt: &Stmt) -> String {
        let Stmt::Block(data) = stmt else { unreachable!() };
        let mut string = String::new();
        string += "{";
        for stmt in &data.statements {
            string += " ";
            string += &stmt.accept(self);
        }
        string += " }";
        string
    }

    fn visit_class_stmt(&mut self, _stmt: &Stmt) -> String {
        todo!()
    }
}
