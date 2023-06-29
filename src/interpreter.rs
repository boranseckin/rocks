use std::cmp::Ordering;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use crate::class::Class;
use crate::environment::Environment;
use crate::error::{self, Error, ReturnType, RuntimeError, ReturnError, BreakError};
use crate::expr::{Expr, ExprVisitor};
use crate::function::{NativeFunction, Function};
use crate::object::{Object, Callable};
use crate::stmt::{Stmt, StmtVisitor};
use crate::token::{Type, Token};
use crate::literal::Literal;

pub struct Interpreter<'w> {
    // Interior mutability with multiple owners
    environment: Rc<RefCell<Environment>>,
    globals: Rc<RefCell<Environment>>,
    locals: HashMap<Token, usize>,
    writer: Box<dyn std::io::Write + 'w>,
}

impl<'w> Interpreter<'w> {
    pub fn new<W: std::io::Write>(writer: &'w mut W) -> Self {
        let globals = Rc::new(RefCell::new(Environment::default()));

        NativeFunction::get_globals().iter().for_each(|native| {
            globals.borrow_mut().define(&native.name.lexeme, Object::from(native.clone()));
        });

        Interpreter {
            environment: Rc::clone(&globals),
            globals: Rc::clone(&globals),
            locals: HashMap::new(),
            writer: Box::new(writer),
        }
    }

    pub fn interpret(&mut self, statements: &Vec<Stmt>) {
        for statement in statements {
            self.execute(statement).unwrap_or_else(|error| {
                if let ReturnType::Error(error) = error {
                    error.throw();
                }
            });
        }
    }
 
    fn execute(&mut self, stmt: &Stmt) -> Result<(), ReturnType> {
        stmt.accept(self)
    }

    pub fn resolve(&mut self, name: &Token, depth: usize) {
        self.locals.insert(name.clone(), depth);
    }

    fn lookup_variable(&mut self, name: &Token) -> Result<Object, ReturnType> {
        let variable = match self.locals.get(name) {
            Some(distance) => self.environment.borrow().get_at(*distance, name),
            None => self.globals.borrow().get(name),
        };

        return match variable {
            Ok(value) => Ok(value),
            Err(error) => Err(ReturnType::Error(error)),
        };
    }

    pub fn execute_block(
        &mut self,
        statements: &Vec<Stmt>,
        environment: Rc<RefCell<Environment>>
    ) -> Result<(), ReturnType> {
        let previous = self.environment.clone();
        self.environment = environment;

        for statement in statements {
            if let Err(return_type) = self.execute(statement) {
                self.environment = previous;
                return Err(return_type);
            }
        }

        self.environment = previous;

        Ok(())
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Object, ReturnType> {
        expr.accept(self)
    }
}

impl<'w> Default for Interpreter<'w> {
    fn default() -> Self {
        Self::new(Box::leak(Box::new(std::io::stdout())))
    }
}

impl<'w> ExprVisitor<Result<Object, ReturnType>> for Interpreter<'w> {
    fn visit_literal_expr(&mut self, expr: &Expr) -> Result<Object, ReturnType> {
        let Expr::Literal(literal) = expr else { unreachable!() };
        Ok(Object::Literal(literal.clone()))
    }

    fn visit_logical_expr(&mut self, expr: &Expr) -> Result<Object, ReturnType> {
        let Expr::Logical(logical) = expr else { unreachable!() };
        let left = self.evaluate(&logical.left)?;

        match logical.operator.r#type {
            Type::Or => if left.as_bool().is_some_and(|x| x) { return Ok(left) },
            Type::And => if !left.as_bool().is_some_and(|x| x) { return Ok(left) },
            _ => unreachable!(),
        };

        self.evaluate(&logical.right)
    }

    fn visit_unary_expr(&mut self, expr: &Expr) -> Result<Object, ReturnType> {
        let Expr::Unary(unary) = expr else { unreachable!() };
        let right = self.evaluate(&unary.expr)?;

        let error_message = format!(
            "Unary operation '{}' is not supported for {} type",
            unary.operator.lexeme.clone(),
            right.type_str()
        );

        let result = match unary.operator.r#type {
            Type::Minus => -right,
            Type::Bang => !right,
            _ => unreachable!(),
        };

        if let Some(result) = result {
            Ok(result)
        } else {
            return Err(ReturnType::Error(RuntimeError {
                token: unary.operator.clone(),
                message: error_message,
            }));
        }
    }

    fn visit_binary_expr(&mut self, expr: &Expr) -> Result<Object, ReturnType> {
        let Expr::Binary(binary) = expr else { unreachable!() };
        let left = self.evaluate(&binary.left)?;
        let right = self.evaluate(&binary.right)?;

        let error_message = format!(
            "Binary operation '{}' is not supported between {} type and {} type",
            binary.operator.lexeme.clone(),
            left.type_str(),
            right.type_str()
        );

        let result = match binary.operator.r#type {
            Type::Plus => left + right,
            Type::Minus => left - right,
            Type::Slash => left / right,
            Type::Star => left * right,

            Type::EqualEqual => Some(Object::Literal(Literal::Bool(left == right))),
            Type::BangEqual => Some(Object::Literal(Literal::Bool(left != right))),

            Type::Greater => left.partial_cmp(&right)
                .map(|x| Object::Literal(Literal::Bool(x == Ordering::Greater))),
            Type::Less => left.partial_cmp(&right)
                .map(|x| Object::Literal(Literal::Bool(x == Ordering::Less))),
            Type::GreaterEqual => left.partial_cmp(&right)
                .map(|x| Object::Literal(Literal::Bool(x == Ordering::Greater || x == Ordering::Equal))),
            Type::LessEqual => left.partial_cmp(&right)
                .map(|x| Object::Literal(Literal::Bool(x == Ordering::Less || x == Ordering::Equal))),

            _ => { unreachable!() }
        };

        if let Some(result) = result {
            return Ok(result);
        } else {
            return Err(ReturnType::Error(RuntimeError {
                token: binary.operator.clone(),
                message: error_message,
            }));
        }
    }

    fn visit_call_expr(&mut self, expr: &Expr) -> Result<Object, ReturnType> {
        let Expr::Call(call) = expr else { unreachable!() };
        let callee = self.evaluate(call.callee.as_ref())?;

        // Early return if callee is not found
        // TODO: handle the case where the callee is "null"
        if let Object::Literal(Literal::Null) = callee {
            return Ok(Object::from(Literal::Null));
        }

        // Collect will fail if Result::Err is is found
        let arguments = call.arguments
            .iter()
            .map(|expr| self.evaluate(expr))
            .collect::<Result<Vec<Object>, ReturnType>>()?;

        match callee {
            Object::Function(function) => {
                if arguments.len() != function.arity() {
                    return Err(ReturnType::Error(RuntimeError {
                        token: call.paren.clone(),
                        message: format!("Expected {} arguments but got {}", function.arity(), arguments.len()),
                    }));
                }

                return match function.call(self, arguments) {
                    Ok(value) => Ok(value),
                    Err(error) => {
                        // TODO: look into implementing call stack on error
                        // error.token = call.paren.clone();
                        return Err(ReturnType::Error(error));
                    }
                };
            },
            Object::NativeFunction(function) => {
                if arguments.len() != function.arity() {
                    return Err(ReturnType::Error(RuntimeError {
                        token: call.paren.clone(),
                        message: format!("Expected {} arguments but got {}", function.arity(), arguments.len()),
                    }));
                }

                return match function.call(self, arguments) {
                    Ok(result) => Ok(result),
                    Err(error) => {
                        // error.token = call.paren.clone();
                        return Err(ReturnType::Error(error));
                    }
                };
            },
            Object::Class(class) => {
                if arguments.len() != class.borrow().arity() {
                    return Err(ReturnType::Error(RuntimeError {
                        token: call.paren.clone(),
                        message: format!("Expected {} arguments but got {}", class.borrow().arity(), arguments.len()),
                    }));
                }

                return match class.borrow().call(self, arguments) {
                    Ok(result) => Ok(result),
                    Err(error) => {
                        // error.token = call.paren.clone();
                        return Err(ReturnType::Error(error));
                    }
                };
            },
            _ => {
                return Err(ReturnType::Error(RuntimeError {
                    token: call.paren.clone(),
                    message: "Can only call functions and classes".to_string(),
                }));
            }
        }
    }

    fn visit_grouping_expr(&mut self, expr: &Expr) -> Result<Object, ReturnType> {
        let Expr::Grouping(grouping) = expr else { unreachable!() };
        self.evaluate(&grouping.expr)
    }

    fn visit_variable_expr(&mut self, expr: &Expr) -> Result<Object, ReturnType> {
        let Expr::Variable(variable) = expr else { unreachable!() };
        self.lookup_variable(&variable.name)
    }

    fn visit_assign_expr(&mut self, expr: &Expr) -> Result<Object, ReturnType> {
        let Expr::Assign(assign) = expr else { unreachable!() };
        let value = self.evaluate(&assign.value)?;

        if let Some(distance) = self.locals.get(&assign.name) {
            self.environment.borrow_mut().assign_at(*distance, &assign.name, value.clone());
        } else {
            self.globals.borrow_mut().assign(&assign.name, value.clone());
        }

        Ok(value)
    }

    fn visit_get_expr(&mut self, expr: &Expr) -> Result<Object, ReturnType> {
        let Expr::Get(get) = expr else { unreachable!() };
        let object = self.evaluate(&get.object)?;

        if let Object::Instance(ref instance) = object {
            return match instance.borrow().get(&get.name, &object) {
                Ok(value) => Ok(value),
                Err(error) => Err(ReturnType::Error(error)),
            }
        }

        return Err(ReturnType::Error(RuntimeError {
            token: get.name.clone(),
            message: "Only instances have properties".to_owned(),
        }));
    }

    fn visit_set_expr(&mut self, expr: &Expr) -> Result<Object, ReturnType> {
        let Expr::Set(set) = expr else { unreachable!() };

        let object = self.evaluate(&set.object)?;

        if let Object::Instance(instance) = object {
            let value = self.evaluate(&set.value)?;
            instance.borrow_mut().set(&set.name, value.clone());
            return Ok(value);
        } else {
            return Err(ReturnType::Error(RuntimeError {
                token: set.name.clone(),
                message: "Only instances can have fields".to_string(),
            }));
        }
    }

    fn visit_this_expr(&mut self, expr: &Expr) -> Result<Object, ReturnType> {
        let Expr::This(this) = expr else { unreachable!() };

        self.lookup_variable(&this.keyword)
    }

    fn visit_super_expr(&mut self, expr: &Expr) -> Result<Object, ReturnType> {
        let Expr::Super(super_expr) = expr else { unreachable!() };

        // Resolver would have catched if super was used incorrectly.
        // It is okay to unwrap here.
        let distance = self.locals.get(&super_expr.keyword).unwrap();
        let superclass = match self.environment.borrow().get_at(*distance, &super_expr.keyword) {
            Ok(value) => Ok(value),
            Err(error) => Err(ReturnType::Error(error)),
        }?;

        let object = match self.environment.borrow().get_at(distance - 1, &Token::from("this")) {
            Ok(value) => Ok(value),
            Err(error) => Err(ReturnType::Error(error)),
        }?;

        if let Object::Class(superclass) = superclass {
            let method = superclass.borrow().get_method(&super_expr.method.lexeme);

            if let Some(mut method) = method {
                return Ok(Object::from(method.bind(object)));
            } else {
                return Err(ReturnType::Error(RuntimeError {
                    token: super_expr.method.clone(),
                    message: format!("Undefined property '{}'", super_expr.method.lexeme)
                }));
            }
        } else {
            unreachable!();
        }
    }
}

impl<'w> StmtVisitor<Result<(), ReturnType>> for Interpreter<'w> {
    fn visit_expression_stmt(&mut self, stmt: &Stmt) -> Result<(), ReturnType> {
        let Stmt::Expression(data) = stmt else { unreachable!() };
        self.evaluate(&data.expr)?;

        Ok(())
    }

    fn visit_function_stmt(&mut self, stmt: &Stmt) -> Result<(), ReturnType> {
        let Stmt::Function(_) = stmt else { unreachable!() };

        let function = Function::new(stmt.to_owned(), Rc::clone(&self.environment), false);

        self.environment.borrow_mut().define(&function.name.lexeme.clone(), Object::from(function));

        Ok(())
    }

    fn visit_if_stmt(&mut self, stmt: &Stmt) -> Result<(), ReturnType> {
        let Stmt::If(data) = stmt else { unreachable!() };
        if self.evaluate(&data.condition)?.as_bool().is_some_and(|x| x) {
            self.execute(&data.then_branch)
        } else if let Some(else_branch) = &data.else_branch {
            self.execute(else_branch)
        } else {
            Ok(())
        }
    }

    fn visit_print_stmt(&mut self, stmt: &Stmt) -> Result<(), ReturnType> {
        let Stmt::Print(data) = stmt else { unreachable!() };
        let value = self.evaluate(&data.expr)?;

        // Make sure evaluate didn't throw an error
        if error::did_error() {
            return Ok(());
        }

        writeln!(self.writer, "{value}").expect("writer to not fail on write");

        Ok(())
    }

    fn visit_return_stmt(&mut self, stmt: &Stmt) -> Result<(), ReturnType> {
        let Stmt::Return(data) = stmt else { unreachable!() };

        let value = if let Some(expr) = &data.value {
            self.evaluate(expr)?
        } else {
            Object::from(Literal::Null)
        };

        Err(ReturnType::Return(ReturnError { value }))
    }

    fn visit_break_stmt(&mut self, stmt: &Stmt) -> Result<(), ReturnType> {
        let Stmt::Break(_) = stmt else { unreachable!() };

        Err(ReturnType::Break(BreakError {}))
    }

    fn visit_var_stmt(&mut self, stmt: &Stmt) -> Result<(), ReturnType> {
        let Stmt::Var(data) = stmt else { unreachable!() };
        let value = match &data.initializer {
            Some(value) => self.evaluate(value)?,
            None => Object::from(Literal::Null),
        };

        self.environment.borrow_mut().define(&data.name.lexeme, value);

        Ok(())
    }

    fn visit_while_stmt(&mut self, stmt: &Stmt) -> Result<(), ReturnType> {
        let Stmt::While(data) = stmt else { unreachable!() };
        while self.evaluate(&data.condition)?.as_bool().is_some_and(|x| x) {
            match self.execute(&data.body) {
                Err(ReturnType::Break(_)) => break,
                Err(err)=> return Err(err),
                _ => {},
            }
        }

        Ok(())
    }

    fn visit_block_stmt(&mut self, stmt: &Stmt) -> Result<(), ReturnType> {
        let Stmt::Block(data) = stmt else { unreachable!() };
        self.execute_block(
            &data.statements,
            Rc::new(RefCell::new(Environment::new(Some(Rc::clone(&self.environment)))))
        )
    }

    fn visit_class_stmt(&mut self, stmt: &Stmt) -> Result<(), ReturnType> {
        let Stmt::Class(data) = stmt else { unreachable!() };

        let superclass = match data.superclass.as_ref() {
            Some(class) => Some(self.evaluate(class)?),
            None => None,
        };

        if let Some(ref superclass) = superclass {
            match superclass {
                Object::Class(_) => (),
                _ => {
                    return Err(ReturnType::Error(RuntimeError {
                        // This is reporting the lexeme of the class name,
                        // it is non-trivial to get the superclass name.
                        token: data.name.clone(),
                        message: "Superclass must be a class".to_string()
                    }));
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
