use crate::expr::{ExprVisitor, UnaryData, BinaryData, GroupingData, Expr, LogicalData, AssignData, VariableData, CallData};
use crate::stmt::{StmtVisitor, Stmt};
use crate::literal::Literal;

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
    fn visit_literal_expr(&mut self, literal: &Literal) -> String {
        literal.to_string() // Uses fmt::Display impl for Literal
    }

    fn visit_logical_expr(&mut self, logical: &LogicalData) -> String {
        parenthesize!(self, &logical.operator.lexeme, &logical.left, &logical.right)
    }

    fn visit_unary_expr(&mut self, unary: &UnaryData) -> String {
        parenthesize!(self, &unary.operator.lexeme, &unary.expr)
    }

    fn visit_binary_expr(&mut self, binary: &BinaryData) -> String {
        parenthesize!(self, &binary.operator.lexeme, &binary.left, &binary.right)
    }

    fn visit_grouping_expr(&mut self, grouping: &GroupingData) -> String {
        parenthesize!(self, "group", grouping.expr)
    }

    fn visit_variable_expr(&mut self, variable: &VariableData) -> String {
        variable.name.lexeme.clone()
    }

    fn visit_assign_expr(&mut self, assign: &AssignData) -> String {
        parenthesize!(self, format!("= {}", &assign.name.lexeme).as_str(), assign.value)
    }

    fn visit_call_expr(&mut self, call: &CallData) -> String {
        let mut string = String::new();
        string += &call.callee.accept(self);
        string += "(";
        for arg in &call.arguments {
            string += &arg.accept(self);
            string += " ";
        }
        string = string.trim_end().to_string();
        string += ")";
        string
    }
}

impl StmtVisitor<String> for ASTPrinter {
    fn visit_expression_stmt(&mut self, stmt: &Stmt) -> String {
        if let Stmt::Expression(data) = stmt {
            parenthesize!(self, "expr", data.expr)
        } else {
            unreachable!()
        }
    }

    fn visit_function_stmt(&mut self, stmt: &Stmt) -> String {
        if let Stmt::Function(data) = stmt {
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
            string += &data.body.iter().map(|stmt| { stmt.accept(self) }).collect::<Vec<String>>().join(" ");
            string += " })";

            string
        } else {
            unreachable!()
        }
    }

    fn visit_if_stmt(&mut self, stmt: &Stmt) -> String {
        if let Stmt::If(data) = stmt {
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
        } else {
            unreachable!()
        }
    }

    fn visit_print_stmt(&mut self, stmt: &Stmt) -> String {
        if let Stmt::Print(data) = stmt {
            parenthesize!(self, "print", data.expr)
        } else {
            unreachable!()
        }
    }

    fn visit_var_stmt(&mut self, stmt: &Stmt) -> String {
        if let Stmt::Var(data) = stmt {
            let mut string = String::new();
            string += "(var ";
            string += &data.name.lexeme;
            if let Some(initializer) = &data.initializer {
                string += " = ";
                string += &initializer.accept(self);
            }
            string += ")";

            string
        } else {
            unreachable!()
        }
    }

    fn visit_while_stmt(&mut self, stmt: &Stmt) -> String {
        if let Stmt::While(data) = stmt {
            parenthesize!(self, "while", data.condition, data.body)
        } else {
            unreachable!()
        }
    }

    fn visit_block_stmt(&mut self, stmt: &Stmt) -> String {
        if let Stmt::Block(data) = stmt {
            let mut string = String::new();
            string += "{";
            for stmt in &data.statements {
                string += " ";
                string += &stmt.accept(self);
            }
            string += " }";

            string
        } else {
            unreachable!()
        }
    }
}
