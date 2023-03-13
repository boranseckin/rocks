use std::fmt::Debug;
use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

use crate::environment::Environment;
use crate::interpreter::Interpreter;
use crate::object::{Object, Callable};
use crate::error::RuntimeError;
use crate::stmt::Stmt;
use crate::token::{Token, Type};
use crate::literal::Literal;

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
}

impl Callable for Function {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Object>) -> Result<Object, RuntimeError> {
        let mut environment = Environment::new(Some(Rc::clone(&interpreter.globals)));

        self.params.iter().zip(arguments.iter()).for_each(|(param, arg)| {
            environment.define(&param.lexeme, arg.to_owned());
        });

        interpreter.execute_block(&self.body, Rc::new(RefCell::new(environment)));

        Ok(Object::from(Literal::Null))
    }

    fn arity(&self) -> usize {
        self.params.len()
    }
}

impl From<Stmt> for Function {
    fn from(value: Stmt) -> Self {
        match value {
            Stmt::Function(function) => function,
            _ => panic!("Cannot convert {:?} to Function", value),
        }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<fn {}>", self.name.lexeme)
    }
}

#[derive(Clone)]
pub struct NativeFunction {
    pub name: Token,
    pub function: fn(&mut Interpreter, Vec<Object>) -> Result<Object, RuntimeError>,
}

impl Callable for NativeFunction {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Object>) -> Result<Object, RuntimeError> {
        (self.function)(interpreter, arguments)
    }

    fn arity(&self) -> usize {
        0
    }
}

impl NativeFunction {
    pub fn get_globals() -> Vec<NativeFunction> {
        vec![
            NativeFunction {
                name: Token::new(Type::Identifier, "clock".to_owned(), None, 0),
                function: |_, _| {
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis();
                    Ok(Object::from(now as f32))
                },
            },
            NativeFunction {
                name: Token::new(Type::Identifier, "input".to_owned(), None, 0),
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
        write!(f, "<native fn {}>", self.name.lexeme)
    }
}

impl Debug for NativeFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn {}>", self.name.lexeme)
    }
}
