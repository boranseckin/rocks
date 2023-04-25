use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::cell::RefCell;
use std::rc::Rc;

use crate::error::RuntimeError;
use crate::function::Function;
use crate::object::{Callable, Object};
use crate::interpreter::Interpreter;
use crate::token::Token;

#[derive(Debug, Clone)]
pub struct Class {
    pub name: String,
    pub superclass: Option<Object>,
    pub methods: HashMap<String, Function>,
}

impl Class {
    pub fn new(name: String, superclass: Option<Object>, methods: HashMap<String, Function>) -> Self {
        Class { name, superclass, methods }
    }

    pub fn get_method(&self, name: &str) -> Option<Function> {
        if let Some(method) = self.methods.get(name) {
            return Some(method.clone());
        } else if let Some(Object::Class(ref superclass)) = self.superclass {
            return superclass.borrow().get_method(name);
        } else {
            return None;
        }
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<class {}>", self.name)
    }
}

impl Callable for Class {
    fn arity(&self) -> usize {
        if let Some(initializer) = self.get_method("init") {
            initializer.arity()
        } else {
            0
        }
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Object>) -> Result<Object, RuntimeError> {
        let instance = Object::from(Instance::from(&Rc::new(RefCell::new(self.clone()))));

        if let Some(mut initializer) = self.get_method("init") {
            initializer.bind(instance.clone()).call(interpreter, arguments)?;
        }

        return Ok(instance);
    }
}

#[derive(Debug, Clone)]
pub struct Instance {
    pub class: Rc<RefCell<Class>>,
    pub fields: HashMap<String, Object>,
}

impl Instance {
    pub fn get(&self, name: &Token, instance: &Object) -> Result<Object, RuntimeError> {
        if let Some(field) = self.fields.get(&name.lexeme) {
            Ok(field.clone())
        } else if let Some(mut method) = self.class.borrow().get_method(&name.lexeme) {
            Ok(Object::from(method.bind(instance.clone())))
        } else {
            Err(RuntimeError {
                token: name.clone(),
                message: format!("Undefined property '{}'", name.lexeme),
            })
        }
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

impl Display for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<instance {}>", self.class.borrow().name)
    }
}
