use crate::expr::{ExprVisitor, UnaryData, BinaryData, GroupingData, Expr};
use crate::token::Literal;

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

    fn visit_unary_expr(&mut self, unary: &UnaryData) -> String {
        parenthesize!(self, &unary.operator.lexeme, &unary.expr)
    }

    fn visit_binary_expr(&mut self, binary: &BinaryData) -> String {
        parenthesize!(self, &binary.operator.lexeme, &binary.left, &binary.right)
    }

    fn visit_grouping_expr(&mut self, grouping: &GroupingData) -> String {
        parenthesize!(self, "group", grouping.expr)
    }
}

