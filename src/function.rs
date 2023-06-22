use std::fmt::{Debug, Display};
use std::cell::RefCell;
use std::rc::Rc;

use crate::environment::Environment;
use crate::interpreter::Interpreter;
use crate::object::{Object, Callable};
use crate::error::{RuntimeError, ReturnType};
use crate::stmt::Stmt;
use crate::token::Token;
use crate::literal::Literal;

/// Represents a function.
/// This is a struct that wraps the function's name, parameters, and body.
/// It also contains a reference to the environment in which it was defined.
#[derive(Debug, Clone)]
pub struct Function {
    pub name: Token,
    params: Vec<Token>,
    body: Vec<Stmt>,
    closure: Rc<RefCell<Environment>>,
    is_initializer: bool,
}

impl Function {
    /// Creates a new function.
    pub fn new(stmt: Stmt, closure: Rc<RefCell<Environment>>, is_initializer: bool) -> Self {
        match stmt {
            Stmt::Function(data) => Function {
                name: data.name,
                params: data.params,
                body: data.body,
                closure,
                is_initializer,
            },
            _ => panic!("Expected function statement"),
        }
    }

    /// Binds the function to an instance by wrapping its environment.
    /// This is used to allow the function to access the instance's fields.
    pub fn bind(&mut self, instance: Object) -> Self {
        let mut environment = Environment::new(Some(Rc::clone(&self.closure)));
        environment.define("this", instance);

        Function {
            name: self.name.clone(),
            params: self.params.clone(),
            body: self.body.clone(),
            closure: Rc::new(RefCell::new(environment)),
            is_initializer: self.is_initializer,
        }
    }
}

impl Callable for Function {
    /// Calls the function (or method) and returns its return value.
    /// Function returns are handled by a special return type ([`ReturnError`](crate::error::ReturnError)).
    /// A closure is created for the function's environment and arguments are bound to it.
    ///
    /// Note: Initializer methods will return the instance that they were called on.
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Object>) -> Result<Object, RuntimeError> {
        let environment = Rc::new(RefCell::new(
            Environment::new(Some(Rc::clone(&self.closure)))
        ));

        self.params.iter().zip(arguments.iter()).for_each(|(param, arg)| {
            environment.borrow_mut().define(&param.lexeme, arg.to_owned());
        });

        match interpreter.execute_block(&self.body, environment) {
            Ok(_) => {
                if self.is_initializer {
                    return self.closure.borrow().get_at(0, &Token::from("this"));
                }

                Ok(Object::from(Literal::Null))
            },
            Err(err) => {
                if self.is_initializer {
                    return self.closure.borrow().get_at(0, &Token::from("this"));
                }

                if let ReturnType::Return(err) = err {
                    return Ok(err.value);
                } else {
                    unreachable!();
                }
            },
        }
    }

    fn arity(&self) -> usize {
        self.params.len()
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<function {}>", self.name.lexeme)
    }
}

/// Represents a native function.
/// This is a special type of function that is used to implement built-in functions.
/// These functions are external to the language and are implemented in Rust.
/// They are used to provide access to the host environment.
///
/// Native functions are defined in the
/// [`NativeFunction::get_globals`](NativeFunction::get_globals) method and used
/// during the initialization of the [`Interpreter`](crate::interpreter::Interpreter::new).
/// These functions will be available to the user in the global scope.
///
/// Current native functions:
/// - `clock()` - Returns the current time in milliseconds.
/// - `input()` - Reads a line of string from the standard input.
#[derive(Clone)]
pub struct NativeFunction {
    pub name: Token,
    function: fn(&mut Interpreter, Vec<Object>) -> Result<Object, RuntimeError>,
}

impl Callable for NativeFunction {
    /// Calls the native function and returns its return value.
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Object>) -> Result<Object, RuntimeError> {
        (self.function)(interpreter, arguments)
    }

    fn arity(&self) -> usize {
        0
    }
}

impl NativeFunction {
    /// Returns a list of native functions with their implementations.
    pub fn get_globals() -> Vec<NativeFunction> {
        vec![
            NativeFunction {
                name: Token::from("clock"),
                function: |_, _| {
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis();
                    Ok(Object::from(now as f64))
                },
            },
            NativeFunction {
                name: Token::from("input"),
                function: |_, _| {
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    input.pop();  // Remove newline
                    Ok(Object::from(input))
                },
            },
        ]
    }
}

impl Display for NativeFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native function {}>", self.name.lexeme)
    }
}

impl Debug for NativeFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native function {}>", self.name.lexeme)
    }
}
