use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::rc::Rc;

use crate::error::RuntimeError;
use crate::function::Function;
use crate::object::{Callable, Object};
use crate::interpreter::Interpreter;
use crate::token::Token;

#[derive(Clone)]
pub struct Class {
    pub name: String,
    pub methods: HashMap<String, Function>,
}

impl Class {
    pub fn new(name: String, methods: HashMap<String, Function>) -> Self {
        Class { name, methods }
    }

    pub fn get_method(&self, name: &str) -> Option<Function> {
        return match self.methods.get(name) {
            Some(method) => Some(method.clone()),
            None => None,
        };
    }
}

impl Debug for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<class {} [{:p}]>", self.name, self)
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<class {} [{:p}]>", self.name, self)
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
        let instance = Instance::from(&Rc::new(RefCell::new(self.clone())));

        if let Some(mut initializer) = self.get_method("init") {
            let object = Object::from(instance.clone());
            initializer.bind(object).call(interpreter, arguments)?;
        }

        return Ok(Object::from(Rc::new(RefCell::new(instance))));
    }
}

#[derive(Clone)]
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

impl Debug for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<instance {} [{:p}]>", self.class.borrow().name, self)
    }
}

impl Display for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<instance {} [{:p}]>", self.class.borrow().name, self)
    }
}
