use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::rc::Rc;

use crate::error::RuntimeError;
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

    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<Object>) -> Result<Object, RuntimeError> {
        let instance = Instance::from(&Rc::new(RefCell::new(self.clone())));
        return Ok(Object::from(Rc::new(RefCell::new(instance))));
    }
}

#[derive(Clone)]
pub struct Instance {
    pub class: Rc<RefCell<Class>>,
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

    pub fn set(&mut self, name: &Token, value: Object) {
        self.fields.insert(name.lexeme.clone(), value);
    }
}

impl From<&Rc<RefCell<Class>>> for Instance {
    fn from(value: &Rc<RefCell<Class>>) -> Self {
        Instance { class: Rc::clone(value), fields: HashMap::new() }
    }
}

impl Debug for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<instance {}>", self.class.borrow().name)
    }
}

impl Display for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<instance {}>", self.class.borrow().name)
    }
}
