use std::mem;
use std::collections::HashMap;

use crate::error::{Error, ResolveError};
use crate::expr::{Expr, ExprVisitor};
use crate::stmt::{Stmt, StmtVisitor};
use crate::interpreter::Interpreter;
use crate::token::Token;

enum FunctionType {
    None,
    Function,
    Initializer,
    Method,
}

enum ClassType {
    None,
    Class,
    Subclass
}

pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    scopes: Vec<HashMap<String, bool>>,
    current_function: FunctionType,
    current_class: ClassType,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        Resolver {
            interpreter,
            scopes: vec![],
            current_function: FunctionType::None,
            current_class: ClassType::None,
        }
    }

    fn resolve_expr(&mut self, expr: &Expr) {
        expr.accept(self);
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) {
        stmt.accept(self);
    }

    pub fn resolve(&mut self, statements: &Vec<Stmt>) {
        for statement in statements {
            self.resolve_stmt(statement)
        }
    }

    fn resolve_function(&mut self, function: &Stmt, r#type: FunctionType) {
        let Stmt::Function(function) = function else { unreachable!() };

        let enclosing_function = mem::replace(&mut self.current_function, r#type);

        self.begin_scope();
        for param in &function.params {
            self.declare(param);
            self.define(param);
        }
        self.resolve(&function.body);
        self.end_scope();

        self.current_function = enclosing_function;
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }

        let scope = self.scopes.last_mut().expect("stack to be not empty");
        if scope.contains_key(&name.lexeme) {
            ResolveError {
                token: name.clone(),
                message: format!("A variable is already defined with name '{}' in this scope", name.lexeme),
            }.throw();
        }
        scope.insert(name.lexeme.to_owned(), false);
    }

    fn define(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }

        self.scopes
            .last_mut()
            .expect("stack to be not empty")
            .insert(name.lexeme.to_owned(), true);
    }

    fn resolve_local(&mut self, name: &Token) {
        for (i, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(&name.lexeme) {
                self.interpreter.resolve(name, i);
                return;
            }
        }
    }
}

impl<'a> ExprVisitor<()> for Resolver<'a> {
    fn visit_variable_expr(&mut self, expr: &Expr) {
        let Expr::Variable(variable) = expr else { unreachable!() };

        if let Some(scope) = self.scopes.last() {
            if let Some(entry) = scope.get(&variable.name.lexeme) {
                if !entry {
                    ResolveError {
                        token: variable.name.to_owned(),
                        message: "Cannot read local variable in its own initializer".to_string(),
                    }.throw();
                }
            }
        }

        self.resolve_local(&variable.name);
    }

    fn visit_assign_expr(&mut self, expr: &Expr) {
        let Expr::Assign(assign) = expr else { unreachable!() };

        self.resolve_expr(&assign.value);
        self.resolve_local(&assign.name);
    }

    fn visit_literal_expr(&mut self, expr: &Expr) {
        let Expr::Literal(_) = expr else { unreachable!() };

        return;
    }

    fn visit_logical_expr(&mut self, expr: &Expr) {
        let Expr::Logical(logical) = expr else { unreachable!() };

        self.resolve_expr(&logical.left);
        self.resolve_expr(&logical.right);
    }

    fn visit_unary_expr(&mut self, expr: &Expr) {
        let Expr::Unary(unary) = expr else { unreachable!() };

        self.resolve_expr(&unary.expr);
    }

    fn visit_binary_expr(&mut self, expr: &Expr) {
        let Expr::Binary(binary) = expr else { unreachable!() };

        self.resolve_expr(&binary.left);
        self.resolve_expr(&binary.right);
    }

    fn visit_grouping_expr(&mut self, expr: &Expr) {
        let Expr::Grouping(grouping) = expr else { unreachable!() };

        self.resolve_expr(&grouping.expr);
    }

    fn visit_call_expr(&mut self, expr: &Expr) {
        let Expr::Call(call) = expr else { unreachable!() };

        self.resolve_expr(&call.callee);

        for argument in &call.arguments {
            self.resolve_expr(argument);
        }
    }

    fn visit_get_expr(&mut self, expr: &Expr) {
        let Expr::Get(get) = expr else { unreachable!() };

        self.resolve_expr(&get.object);
    }

    fn visit_set_expr(&mut self, expr: &Expr) {
        let Expr::Set(set) = expr else { unreachable!() };

        self.resolve_expr(&set.value);
        self.resolve_expr(&set.object);
    }

    fn visit_this_expr(&mut self, expr: &Expr) {
        let Expr::This(this) = expr else { unreachable!() };

        if let ClassType::None = self.current_class {
            ResolveError {
                token: this.keyword.clone(),
                message: "Cannot use 'this' outside of a class".to_string(),
            }.throw();

            return;
        }

        self.resolve_local(&this.keyword);
    }

    fn visit_super_expr(&mut self, expr: &Expr) {
        let Expr::Super(super_expr) = expr else { unreachable!() };

        match self.current_class {
            ClassType::Subclass => (),
            ClassType::None => ResolveError {
                token: super_expr.keyword.clone(),
                message: "Cannot use 'super' outside of a class".to_string()
            }.throw(),
            _ => ResolveError {
                token: super_expr.keyword.clone(),
                message: "Cannot use 'super' in a class with no superclass".to_string(),
            }.throw(),
        }

        self.resolve_local(&super_expr.keyword);
    }
}

impl<'a> StmtVisitor<()> for Resolver<'a> {
    fn visit_block_stmt(&mut self, stmt: &Stmt) {
        let Stmt::Block(block) = stmt else { unreachable!() };

        self.begin_scope();
        self.resolve(&block.statements);
        self.end_scope();
    }

    fn visit_var_stmt(&mut self, stmt: &Stmt) {
        let Stmt::Var(var) = stmt else { unreachable!() };

        self.declare(&var.name);
        if let Some(initializer) = &var.initializer {
            self.resolve_expr(initializer);
        }
        self.define(&var.name);
    }

    fn visit_function_stmt(&mut self, stmt: &Stmt) {
        let Stmt::Function(function) = stmt else { unreachable!() };

        self.declare(&function.name);
        self.define(&function.name);

        self.resolve_function(stmt, FunctionType::Function);
    }

    fn visit_expression_stmt(&mut self, stmt: &Stmt) {
        let Stmt::Expression(expr) = stmt else { unreachable!() };

        self.resolve_expr(&expr.expr);
    }

    fn visit_if_stmt(&mut self, stmt: &Stmt) {
        let Stmt::If(if_stmt) = stmt else { unreachable!() };

        self.resolve_expr(&if_stmt.condition);
        self.resolve_stmt(&if_stmt.then_branch);
        if let Some(else_branch) = &if_stmt.else_branch {
            self.resolve_stmt(else_branch);
        }
    }

    fn visit_print_stmt(&mut self, stmt: &Stmt) {
        let Stmt::Print(print) = stmt else { unreachable!() };

        self.resolve_expr(&print.expr);
    }

    fn visit_return_stmt(&mut self, stmt: &Stmt) {
        let Stmt::Return(return_stmt) = stmt else { unreachable!() };

        if let FunctionType::None = self.current_function {
            ResolveError {
                token: return_stmt.keyword.clone(),
                message: "Cannot return from top-level code".to_string(),
            }.throw();
        }

        if let Some(value) = &return_stmt.value {
            if let FunctionType::Initializer = self.current_function {
                ResolveError {
                    token: return_stmt.keyword.clone(),
                    message: "Cannot return a value from an initializer".to_string(),
                }.throw();
                return;
            }

            self.resolve_expr(value);
        }
    }

    fn visit_break_stmt(&mut self, stmt: &Stmt) {
        let Stmt::Break(_) = stmt else { unreachable!() };
    }

    fn visit_while_stmt(&mut self, stmt: &Stmt) {
        let Stmt::While(while_stmt) = stmt else { unreachable!() };

        self.resolve_expr(&while_stmt.condition);
        self.resolve_stmt(&while_stmt.body);
    }

    fn visit_class_stmt(&mut self, stmt: &Stmt) {
        let Stmt::Class(class_stmt) = stmt else { unreachable!() };

        let enclosing_class = mem::replace(&mut self.current_class, ClassType::Class);

        self.declare(&class_stmt.name);
        self.define(&class_stmt.name);

        if let Some(ref superclass) = class_stmt.superclass {
            if let Expr::Variable(variable) = superclass {
                if class_stmt.name.lexeme == variable.name.lexeme {
                    ResolveError {
                        token: variable.name.clone(),
                        message: "A class cannot inherit from itself".to_string(),
                    }.throw();
                }
            } else {
                unreachable!();
            }

            self.current_class = ClassType::Subclass;

            self.resolve_expr(superclass);

            self.begin_scope();
            self.scopes
                .last_mut()
                .expect("stack to be not empty")
                .insert("super".to_string(), true);
        }

        self.begin_scope();
        self.scopes
            .last_mut()
            .expect("stack to be not empty")
            .insert("this".to_string(), true);

        for method in &class_stmt.methods {
            if let Stmt::Function(function) = method {
                let decleration = if function.name.lexeme.eq("init") {
                    FunctionType::Initializer
                } else {
                    FunctionType::Method
                };
                self.resolve_function(method, decleration);
            } else {
                unreachable!();
            }
        }

        self.end_scope();

        if class_stmt.superclass.is_some() {
            self.end_scope();
        }

        self.current_class = enclosing_class;
    }
}
