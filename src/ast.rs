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
        string += "(call ";
        string += &data.callee.accept(self);
        string += " (";
        for arg in &data.arguments {
            string += &arg.accept(self);
            string += " ";
        }
        string = string.trim_end().to_string();
        string += "))";
        string
    }

    fn visit_get_expr(&mut self, expr: &Expr) -> String {
        let Expr::Get(data) = expr else { unreachable!() };
        let mut string = String::new();
        string += "(get ";
        string += &data.object.accept(self);
        string += " ";
        string += &data.name.lexeme;
        string += ")";
        string
    }

    fn visit_set_expr(&mut self, expr: &Expr) -> String {
        let Expr::Set(data) = expr else { unreachable!() };
        let mut string = String::new();
        string += "(set ";
        string += &data.object.accept(self);
        string += " ";
        string += &data.name.lexeme;
        string += " ";
        string += &data.value.accept(self);
        string += ")";
        string
    }

    fn visit_this_expr(&mut self, _expr: &Expr) -> String {
        "this".to_string()
    }

    fn visit_super_expr(&mut self, expr: &Expr) -> String {
        let Expr::Super(data) = expr else { unreachable!() };
        let mut string = String::new();
        string += "(super ";
        string += &data.method.lexeme;
        string += ")";
        string
    }
}

impl StmtVisitor<String> for ASTPrinter {
    fn visit_expression_stmt(&mut self, stmt: &Stmt) -> String {
        let Stmt::Expression(data) = stmt else { unreachable!() };
        data.expr.accept(self)
    }

    fn visit_function_stmt(&mut self, stmt: &Stmt) -> String {
        let Stmt::Function(data) = stmt else { unreachable!() };
        let mut string = String::new();
        string += "(fun ";
        string += &data.name.lexeme;
        string += " (";
        for param in &data.params {
            string += &param.lexeme;
            string += " ";
        }
        string = string.trim_end().to_string();
        string += ") { ";
        for body in &data.body {
            string += &body.accept(self);
        }
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
        let mut string = String::new();
        string += "(while ";
        string += &data.condition.accept(self);
        string += " ";
        string += &data.body.accept(self);
        string += ")";
        string
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

    fn visit_class_stmt(&mut self, stmt: &Stmt) -> String {
        let Stmt::Class(data) = stmt else { unreachable!() };
        let mut string = String::new();
        string += "(class ";
        string += &data.name.lexeme;
        if let Some(superclass) = &data.superclass {
            string += " < ";
            string += &superclass.accept(self);
        }
        for method in &data.methods {
            string += " ";
            string += &method.accept(self);
        }
        string += ")";
        string
    }

    fn visit_break_stmt(&mut self, stmt: &Stmt) -> String {
        let Stmt::Break(_) = stmt else { unreachable!() };
        "break".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::Scanner;
    use crate::parser::Parser;

    #[test]
    fn test_ast_printer() {
        let source = "var a = 1; var b = 2; print a + b;";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut printer = ASTPrinter {};
        let ast = statements.iter()
            .map(|stmt| { stmt.accept(&mut printer) })
            .collect::<Vec<String>>()
            .join(" ");
        assert_eq!(ast, "(var a = 1) (var b = 2) (print (+ a b))");
    }

    #[test]
    fn test_ast_printer_with_grouping() {
        let source = "print (1 + 2) * 3;";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut printer = ASTPrinter {};
        let ast = statements.iter()
            .map(|stmt| { stmt.accept(&mut printer) })
            .collect::<Vec<String>>()
            .join(" ");
        assert_eq!(ast, "(print (* (group (+ 1 2)) 3))");
    }

    #[test]
    fn test_ast_printer_with_if() {
        let source = "if (a > 0) { print a; } else { print -a; }";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut printer = ASTPrinter {};
        let ast = statements.iter()
            .map(|stmt| { stmt.accept(&mut printer) })
            .collect::<Vec<String>>()
            .join(" ");
        assert_eq!(ast, "(if (> a 0) { (print a) } else { (print (- a)) })");
    }

    #[test]
    fn test_ast_printer_with_function() {
        let source = "fun add(a, b) { return a + b; }";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut printer = ASTPrinter {};
        let ast = statements.iter()
            .map(|stmt| { stmt.accept(&mut printer) })
            .collect::<Vec<String>>()
            .join(" ");
        assert_eq!(ast, "(fun add (a b) { (return (+ a b)) })");
    }

    #[test]
    fn test_ast_printer_with_class() {
        let source = "class Foo { bar() { print \"bar\"; } }";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut printer = ASTPrinter {};
        let ast = statements.iter()
            .map(|stmt| { stmt.accept(&mut printer) })
            .collect::<Vec<String>>()
            .join(" ");
        assert_eq!(ast, "(class Foo (fun bar () { (print bar) }))");
    }

    #[test]
    fn test_ast_printer_with_break() {
        let source = "while (true) { break; }";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut printer = ASTPrinter {};
        let ast = statements.iter()
            .map(|stmt| { stmt.accept(&mut printer) })
            .collect::<Vec<String>>()
            .join(" ");
        assert_eq!(ast, "(while true { break })");
    }

    #[test]
    fn test_ast_printer_with_assignment() {
        let source = "var a = 1; a = 2;";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut printer = ASTPrinter {};
        let ast = statements.iter()
            .map(|stmt| { stmt.accept(&mut printer) })
            .collect::<Vec<String>>()
            .join(" ");
        assert_eq!(ast, "(var a = 1) (= a 2)");
    }

    #[test]
    fn test_ast_printer_with_logical() {
        let source = "true and false or true;";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut printer = ASTPrinter {};
        let ast = statements.iter()
            .map(|stmt| { stmt.accept(&mut printer) })
            .collect::<Vec<String>>()
            .join(" ");
        assert_eq!(ast, "(or (and true false) true)");
    }

    #[test]
    fn test_ast_printer_with_call() {
        let source = "foo(1, 2);";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut printer = ASTPrinter {};
        let ast = statements.iter()
            .map(|stmt| { stmt.accept(&mut printer) })
            .collect::<Vec<String>>()
            .join("");
        assert_eq!(ast, "(call foo (1 2))");
    }

    #[test]
    fn test_ast_printer_with_get() {
        let source = "foo.bar;";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut printer = ASTPrinter {};
        let ast = statements.iter()
            .map(|stmt| { stmt.accept(&mut printer) })
            .collect::<Vec<String>>()
            .join("");
        assert_eq!(ast, "(get foo bar)");
    }

    #[test]
    fn test_ast_printer_with_set() {
        let source = "foo.bar = 1;";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut printer = ASTPrinter {};
        let ast = statements.iter()
            .map(|stmt| { stmt.accept(&mut printer) })
            .collect::<Vec<String>>()
            .join("");
        assert_eq!(ast, "(set foo bar 1)");
    }

    #[test]
    fn test_ast_printer_with_this() {
        let source = "this.foo;";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut printer = ASTPrinter {};
        let ast = statements.iter()
            .map(|stmt| { stmt.accept(&mut printer) })
            .collect::<String>();
        assert_eq!(ast, "(get this foo)");
    }

    #[test]
    fn test_ast_printer_with_super() {
        let source = "class a < b { init() { super.init(); } }";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        let mut printer = ASTPrinter {};
        let ast = statements.iter()
            .map(|stmt| { stmt.accept(&mut printer) })
            .collect::<String>();
        assert_eq!(ast, "(class a < b (fun init () { (call (super init) ()) }))");
    }
}
