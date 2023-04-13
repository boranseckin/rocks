use std::collections::HashMap;
use std::fmt::{Debug, Display};

use crate::error::{RuntimeError};
use crate::object::{Callable, Object};
use crate::interpreter::Interpreter;
use crate::token::Token;

#[derive(Clone)]
pub struct Class {
    pub name: String,
}

impl Debug for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<class {}>", self.name)
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<class {}>", self.name)
    }
}

impl Callable for Class {
    fn arity(&self) -> usize {
        return 0;
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Object>) -> Result<Object, RuntimeError> {
        return Ok(Object::from(Instance::from(self.clone())));
    }
}

#[derive(Clone)]
pub struct Instance {
    pub class: Class,
    pub fields: HashMap<String, Object>,
}

impl Instance {
    pub fn get(&self, name: &Token) -> Result<Object, RuntimeError> {
        if self.fields.contains_key(&name.lexeme) {
            return Ok(self.fields.get(&name.lexeme).unwrap().clone());
        }

        Err(RuntimeError {
            token: name.clone(),
            message: format!("Undefined property '{}'", name.lexeme),
        })
    }
}

impl From<Class> for Instance {
    fn from(value: Class) -> Self {
        Instance { class: value, fields: HashMap::new() }
    }
}

impl Debug for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<instance {}>", self.class.name)
    }
}

impl Display for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<instance {}>", self.class.name)
    }
}
