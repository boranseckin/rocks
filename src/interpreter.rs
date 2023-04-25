use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use crate::class::Class;
use crate::environment::Environment;
use crate::error::{self, Error, RuntimeError, ReturnError};
use crate::expr::{Expr, ExprVisitor};
use crate::function::{NativeFunction, Function};
use crate::object::{Object, Callable};
use crate::stmt::{Stmt, StmtVisitor};
use crate::token::{Type, Token};
use crate::literal::Literal;

pub struct Interpreter {
    // Interior mutability with multiple owners
    environment: Rc<RefCell<Environment>>,
    globals: Rc<RefCell<Environment>>,
    locals: HashMap<Token, usize>,
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Rc::new(RefCell::new(Environment::default()));

        NativeFunction::get_globals().iter().for_each(|native| {
            globals.borrow_mut().define(&native.name.lexeme, Object::from(native.clone()));
        });

        Interpreter {
            environment: Rc::clone(&globals),
            globals: Rc::clone(&globals),
            locals: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, statements: &Vec<Stmt>) {
        for statement in statements {
            self.execute(statement).unwrap_or(/* Do nothing */ ());
        }
    }
 
    fn execute(&mut self, stmt: &Stmt) -> Result<(), ReturnError> {
        stmt.accept(self)
    }

    pub fn resolve(&mut self, name: &Token, depth: usize) {
        self.locals.insert(name.clone(), depth);
    }

    fn lookup_variable(&mut self, name: &Token) -> Object {
        if let Some(distance) = self.locals.get(name) {
            self.environment.borrow().get_at(*distance, name).unwrap_or_else(|err| {
                err.throw();
                Object::Literal(Literal::Null)
            })
        } else {
            self.globals.borrow().get(name).unwrap_or_else(|err| {
                err.throw();
                Object::Literal(Literal::Null)
            })
        }
    }

    pub fn execute_block(
        &mut self,
        statements: &Vec<Stmt>,
        environment: Rc<RefCell<Environment>>
    ) -> Result<(), ReturnError> {
        let previous = self.environment.clone();
        self.environment = environment;

        for statement in statements {
            self.execute(statement)?;
        }

        self.environment = previous;

        Ok(())
    }

    fn evaluate(&mut self, expr: &Expr) -> Object {
        expr.accept(self)
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl ExprVisitor<Object> for Interpreter {
    fn visit_literal_expr(&mut self, expr: &Expr) -> Object {
        let Expr::Literal(expr) = expr else { unreachable!() };
        Object::Literal(expr.clone())
    }

    fn visit_logical_expr(&mut self, expr: &Expr) -> Object {
        let Expr::Logical(expr) = expr else { unreachable!() };
        let left = self.evaluate(&expr.left);

        match expr.operator.r#type {
            Type::Or => if left.as_bool() { return left },
            Type::And => if !left.as_bool() { return left },
            _ => unreachable!(),
        };

        self.evaluate(&expr.right)
    }

    fn visit_unary_expr(&mut self, expr: &Expr) -> Object {
        let Expr::Unary(expr) = expr else { unreachable!() };
        let right = self.evaluate(&expr.expr);

        match expr.operator.r#type {
            Type::Minus => Object::Literal(Literal::Number(-right.as_number())),
            Type::Bang => Object::Literal(Literal::Bool(!right.as_bool())),
            _ => unreachable!(),
        }
    }

    fn visit_binary_expr(&mut self, expr: &Expr) -> Object {
        let Expr::Binary(expr) = expr else { unreachable!() };
        let left = self.evaluate(&expr.left);
        let right = self.evaluate(&expr.right);

        match expr.operator.r#type {
            Type::Greater       => Object::from(left.as_number() > right.as_number()),
            Type::GreaterEqual  => Object::from(left.as_number() >= right.as_number()),
            Type::Less          => Object::from(left.as_number() < right.as_number()),
            Type::LessEqual     => Object::from(left.as_number() <= right.as_number()),
            Type::EqualEqual    => Object::from(left.as_number() == right.as_number()),
            Type::BangEqual     => Object::from(left.as_number() != right.as_number()),
            Type::Slash         => Object::from(left.as_number() / right.as_number()),
            Type::Star          => Object::from(left.as_number() * right.as_number()),
            Type::Minus         => Object::from(left.as_number() - right.as_number()),
            Type::Plus          => match (left, right) {
                (Object::Literal(Literal::Number(l)), Object::Literal(Literal::Number(r))) => Object::from(l + r),
                (Object::Literal(Literal::String(l)), Object::Literal(Literal::String(r))) => Object::from(l + &r),
                _ => {
                    RuntimeError {
                        token: expr.operator.clone(),
                        message: "Tried to add two unsupported types".to_string(),
                    }.throw();
                    Object::from(Literal::Null)
                }
            },
            _ => unreachable!(),
        }
    }

    fn visit_call_expr(&mut self, expr: &Expr) -> Object {
        let Expr::Call(expr) = expr else { unreachable!() };
        let callee = self.evaluate(expr.callee.as_ref());

        let arguments: Vec<Object> = expr.arguments
            .iter()
            .map(|expr| self.evaluate(expr))
            .collect();

        match callee {
            Object::Function(function) => {
                if arguments.len() != function.arity() {
                    RuntimeError {
                        token: expr.paren.clone(),
                        message: format!("Expected {} arguments but got {}", function.arity(), arguments.len()),
                    }.throw();
                    return Object::from(Literal::Null);
                }

                function.call(self, arguments).unwrap_or_else(|mut error| {
                    error.token = expr.paren.clone();
                    error.throw();
                    Object::from(Literal::Null)
                })
            },
            Object::NativeFunction(function) => {
                if arguments.len() != function.arity() {
                    RuntimeError {
                        token: expr.paren.clone(),
                        message: format!("Expected {} arguments but got {}", function.arity(), arguments.len()),
                    }.throw();
                    return Object::from(Literal::Null);
                }

                function.call(self, arguments).unwrap_or_else(|mut error| {
                    error.token = expr.paren.clone();
                    error.throw();
                    Object::from(Literal::Null)
                })
            },
            Object::Class(class) => {
                if arguments.len() != class.borrow().arity() {
                    RuntimeError {
                        token: expr.paren.clone(),
                        message: format!("Expected {} arguments but got {}", class.borrow().arity(), arguments.len()),
                    }.throw();
                    return Object::from(Literal::Null);
                }

                class.borrow().call(self, arguments).unwrap_or_else(|mut error| {
                    error.token = expr.paren.clone();
                    error.throw();
                    Object::from(Literal::Null)
                })
            },
            _ => {
                RuntimeError {
                    token: expr.paren.clone(),
                    message: "Can only call functions and classes".to_string(),
                }.throw();
                Object::from(Literal::Null)
            }
        }
    }

    fn visit_grouping_expr(&mut self, expr: &Expr) -> Object {
        let Expr::Grouping(expr) = expr else { unreachable!() };
        self.evaluate(&expr.expr)
    }

    fn visit_variable_expr(&mut self, expr: &Expr) -> Object {
        let Expr::Variable(expr) = expr else { unreachable!() };
        self.lookup_variable(&expr.name)
    }

    fn visit_assign_expr(&mut self, expr: &Expr) -> Object {
        let Expr::Assign(expr) = expr else { unreachable!() };
        let value = self.evaluate(&expr.value);

        if let Some(distance) = self.locals.get(&expr.name) {
            self.environment.borrow_mut().assign_at(*distance, &expr.name, value.clone());
        } else {
            self.globals.borrow_mut().assign(&expr.name, value.clone());
        }

        value
    }

    fn visit_get_expr(&mut self, expr: &Expr) -> Object {
        let Expr::Get(expr) = expr else { unreachable!() };
        let object = self.evaluate(&expr.object);

        if let Object::Instance(ref instance) = object {
            return instance.borrow().get(&expr.name, &object).unwrap_or_else(|err| {
                err.throw();
                todo!("Make this a real runtime error");
            });
        }

        RuntimeError {
            token: expr.name.clone(),
            message: "Only instances have properties".to_owned(),
        }.throw();

        todo!("Make this a real runtime error");
    }

    fn visit_set_expr(&mut self, expr: &Expr) -> Object {
        let Expr::Set(expr) = expr else { unreachable!() };

        let object = self.evaluate(&expr.object);

        match object {
            Object::Instance(instance) => {
                let value = self.evaluate(&expr.value);
                instance.borrow_mut().set(&expr.name, value.clone());
                return value;
            },
            _ => {
                RuntimeError {
                    token: expr.name.clone(),
                    message: "Only instances can have fields".to_string(),
                }.throw();

                todo!("Make this a real runtime error");
            }
        }
    }

    fn visit_this_expr(&mut self, expr: &Expr) -> Object {
        let Expr::This(expr) = expr else { unreachable!() };

        self.lookup_variable(&expr.keyword)
    }

    fn visit_super_expr(&mut self, expr: &Expr) -> Object {
        let Expr::Super(super_expr) = expr else { unreachable!() };

        // Resolver would have catched if super was used incorrectly.
        // It is okay to unwrap everythin here.
        let distance = self.locals.get(&super_expr.keyword).unwrap();
        let superclass = self.environment.borrow().get_at(*distance, &Token::from("super")).unwrap();

        let object = self.environment.borrow().get_at(distance - 1, &Token::from("this")).unwrap();

        // TODO: Make this unwrapping better
        if let Object::Class(superclass) = superclass {
            let method = superclass.borrow().get_method(&super_expr.method.lexeme);
            if let Some(mut method) = method {
                return Object::from(method.bind(object));
            } else {
                RuntimeError {
                    token: super_expr.method.clone(),
                    message: format!("Undefined property '{}'", super_expr.method.lexeme)
                }.throw();
                return Object::Literal(Literal::Null);
            }
        } else {
            unreachable!();
        }
    }
}

impl StmtVisitor<Result<(), ReturnError>> for Interpreter {
    fn visit_expression_stmt(&mut self, stmt: &Stmt) -> Result<(), ReturnError> {
        let Stmt::Expression(data) = stmt else { unreachable!() };
        self.evaluate(&data.expr);

        Ok(())
    }

    fn visit_function_stmt(&mut self, stmt: &Stmt) -> Result<(), ReturnError> {
        let Stmt::Function(_) = stmt else { unreachable!() };

        let function = Function::new(stmt.to_owned(), Rc::clone(&self.environment), false);

        self.environment.borrow_mut().define(&function.name.lexeme.clone(), Object::from(function));

        Ok(())
    }

    fn visit_if_stmt(&mut self, stmt: &Stmt) -> Result<(), ReturnError> {
        let Stmt::If(data) = stmt else { unreachable!() };
        if self.evaluate(&data.condition).as_bool() {
            self.execute(&data.then_branch)
        } else if let Some(else_branch) = &data.else_branch {
            self.execute(else_branch)
        } else {
            Ok(())
        }
    }

    fn visit_print_stmt(&mut self, stmt: &Stmt) -> Result<(), ReturnError> {
        let Stmt::Print(data) = stmt else { unreachable!() };
        let value = self.evaluate(&data.expr);

        // Make sure evaluate didn't throw an error
        if error::did_error() {
            return Ok(());
        }

        println!("{value}");

        Ok(())
    }

    fn visit_return_stmt(&mut self, stmt: &Stmt) -> Result<(), ReturnError> {
        let Stmt::Return(data) = stmt else { unreachable!() };

        let value = if let Some(expr) = &data.value {
            self.evaluate(expr)
        } else {
            Object::from(Literal::Null)
        };

        Err(ReturnError { value })
    }

    fn visit_var_stmt(&mut self, stmt: &Stmt) -> Result<(), ReturnError> {
        let Stmt::Var(data) = stmt else { unreachable!() };
        let value = match &data.initializer {
            Some(value) => self.evaluate(value),
            None => Object::from(Literal::Null),
        };

        self.environment.borrow_mut().define(&data.name.lexeme, value);

        Ok(())
    }

    fn visit_while_stmt(&mut self, stmt: &Stmt) -> Result<(), ReturnError> {
        let Stmt::While(data) = stmt else { unreachable!() };
        while self.evaluate(&data.condition).as_bool() {
            self.execute(&data.body)?;
        }

        Ok(())
    }

    fn visit_block_stmt(&mut self, stmt: &Stmt) -> Result<(), ReturnError> {
        let Stmt::Block(data) = stmt else { unreachable!() };
        self.execute_block(
            &data.statements,
            Rc::new(RefCell::new(Environment::new(Some(Rc::clone(&self.environment)))))
        )
    }

    fn visit_class_stmt(&mut self, stmt: &Stmt) -> Result<(), ReturnError> {
        let Stmt::Class(data) = stmt else { unreachable!() };

        let superclass = data.superclass.as_ref().map(|superclass| self.evaluate(superclass));
        if let Some(ref superclass) = superclass {
            match superclass {
                Object::Class(_) => {},
                _ => {
                    RuntimeError {
                        // This is reporting the lexeme of the class name,
                        // it is non-trivial to get the superclass name.
                        token: data.name.clone(),
                        message: "Superclass must be a class".to_string()
                    }.throw();
                },
            }
        }

        self.environment.borrow_mut().define(&data.name.lexeme, Object::Literal(Literal::Null));

        if let Some(ref superclass) = superclass {
            let mut environment = Environment::new(Some(Rc::clone(&self.environment)));
            environment.define("super", superclass.clone());
            self.environment = Rc::new(RefCell::new(environment));
        }

        let mut methods: HashMap<String, Function> = HashMap::new();
        for method in &data.methods {
            if let Stmt::Function(function) = method {
                let function = Function::new(
                    method.clone(),
                    Rc::clone(&self.environment),
                    function.name.lexeme.eq("init")
                );
                methods.insert(function.name.lexeme.clone(), function);
            } else {
                unreachable!();
            }
        }

        let class = Class::new(data.name.lexeme.clone(), superclass.clone(), methods);

        if superclass.is_some() {
            let enclosing = self.environment.borrow().enclosing.clone().expect("enclosing to exist");
            self.environment = enclosing;
        }

        self.environment.borrow_mut().assign(&data.name, Object::from(Rc::new(RefCell::new(class))));

        Ok(())
    }
}
