use std::fmt::{Debug, Display};
use std::cell::RefCell;
use std::rc::Rc;

use crate::class::{Class, Instance};
use crate::error::RuntimeError;
use crate::function::{Function, NativeFunction};
use crate::literal::Literal;
use crate::interpreter::Interpreter;

/// Represents an object that can be stored in a variable or returned from a function.
/// This is an enum that wraps all the possible types of values in the language.
#[derive(Debug, Clone)]
pub enum Object {
    Literal(Literal),
    Function(Function),
    NativeFunction(NativeFunction),
    Class(Rc<RefCell<Class>>),
    Instance(Rc<RefCell<Instance>>),
}

impl Object {
    /// Returns the object as a number if it is a literal.
    pub fn as_number(&self) -> f64 {
        match self {
            Object::Literal(literal) => literal.as_number(),
            _ => panic!("Cannot convert non-literal object to number"),
        }
    }

    /// Returns the object as a boolean if it is a literal.
    pub fn as_bool(&self) -> bool {
        match self {
            Object::Literal(literal) => literal.as_bool(),
            _ => panic!("Cannot convert non-literal object to bool"),
        }
    }
}

impl From<Literal> for Object {
    fn from(literal: Literal) -> Self {
        Object::Literal(literal)
    }
}

impl From<f64> for Object {
    fn from(number: f64) -> Self {
        Object::Literal(Literal::Number(number))
    }
}

impl From<bool> for Object {
    fn from(boolean: bool) -> Self {
        Object::Literal(Literal::Bool(boolean))
    }
}

impl From<String> for Object {
    fn from(string: String) -> Self {
        Object::Literal(Literal::String(string))
    }
}

impl From<&str> for Object {
    fn from(string: &str) -> Self {
        Object::Literal(Literal::String(string.to_owned()))
    }
}

impl From<Function> for Object {
    fn from(value: Function) -> Self {
        Object::Function(value)
    }
}

impl From<NativeFunction> for Object {
    fn from(value: NativeFunction) -> Self {
        Object::NativeFunction(value)
    }
}

impl From<Rc<RefCell<Class>>> for Object {
    fn from(value: Rc<RefCell<Class>>) -> Self {
        Object::Class(value)
    }
}

impl From<Instance> for Object {
    fn from(value: Instance) -> Self {
        Object::Instance(Rc::new(RefCell::new(value)))
    }
}

impl From<Rc<RefCell<Instance>>> for Object {
    fn from(value: Rc<RefCell<Instance>>) -> Self {
        Object::Instance(value)
    }
}

/// Partial equality is defined as equality of the underlying literal values.
/// If the objects are not literals, they are not equal.
impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::Literal(left), Object::Literal(right)) => left == right,
            _ => false,
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Literal(literal) => write!(f, "{literal}"),
            Object::Function(function) => write!(f, "{function}"),
            Object::NativeFunction(function) => write!(f, "{function}"),
            Object::Class(class) => write!(f, "{}", class.borrow()),
            Object::Instance(instance) => write!(f, "{}", instance.borrow()),
        }
    }
}

/// Represents a callable object in the language.
pub trait Callable: Debug {
    /// Calls the object with the given arguments and the current state of the interpreter.
    /// The interpreter is passed in as a mutable refernece so that the object can access
    /// the environment and call other functions. It returns the result of the call or an
    /// error if the call failed for some reason.
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Object>) -> Result<Object, RuntimeError>;

    /// Returns the arity of the object.
    fn arity(&self) -> usize;
}
