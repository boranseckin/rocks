use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::cell::RefCell;
use std::rc::Rc;

use crate::error::RuntimeError;
use crate::function::Function;
use crate::object::{Callable, Object};
use crate::interpreter::Interpreter;
use crate::token::Token;

/// Represents a class in the language.
/// ##### Instantiation
/// Using `()` operator after the class name will create a new instance of the class. The arguments
/// passed to the operator will be passed to the `init` method of the class. The `init` method is
/// optional and can be omitted. If the `init` method is omitted, the class will inherit the `init`
/// method of its superclass (if any).
/// ##### Methods
/// Methods are functions that are defined inside a class. They can be called on instances of the
/// class. Methods can be defined by using the `()` operator after the method name. Methods can
/// take any number of arguments.
/// ##### Self Reference
/// Using `this` keyword will refer to the instance of the class that the method is being called on.
/// ##### Inheritance
/// Using `<` operator will create a new class with the left-hand side as the superclass.
/// The superclass is an optional object to another class that this class inherits from.
/// When inheriting from a superclass, the subclass will inherit all the methods of the superclass.
/// Prefixing a method call with `super` will call the superclass's method of the same name.
#[derive(Debug, Clone)]
pub struct Class {
    name: String,
    superclass: Option<Object>,
    methods: HashMap<String, Function>,
}

impl Class {
    /// Creates a new class with the given name, superclass (if any), and methods.
    pub fn new(name: String, superclass: Option<Object>, methods: HashMap<String, Function>) -> Self {
        Class { name, superclass, methods }
    }

    /// Returns the method with the given name. If the method is not defined, it will return `None`.
    /// If the method is not defined in this class, it will inherit the method from its superclass
    /// (if any).
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
    /// Returns the arity of the `init` method of the class. If the `init` method is not defined,
    /// it will return 0.
    fn arity(&self) -> usize {
        if let Some(initializer) = self.get_method("init") {
            initializer.arity()
        } else {
            0
        }
    }

    /// Creates a new instance of the class and calls the `init` method on it.
    /// If the `init` method is not defined, it will inherit the `init` method of its superclass
    /// (if any).
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Object>) -> Result<Object, RuntimeError> {
        let instance = Object::from(Instance::from(&Rc::new(RefCell::new(self.clone()))));

        if let Some(mut initializer) = self.get_method("init") {
            initializer.bind(instance.clone()).call(interpreter, arguments)?;
        }

        return Ok(instance);
    }
}

/// Represents an instance of a class.
/// ##### Fields
/// Fields are variables that are defined inside a class. They can be accessed by using the `.` operator
/// after the instance name. Fields can be defined by using the `=` operator after the field name.
/// ##### Methods
/// Methods are functions that are defined inside a class. They can be called on instances of the
/// class. Methods can be called by using the `()` operator after the method name.
#[derive(Debug, Clone)]
pub struct Instance {
    class: Rc<RefCell<Class>>,
    fields: HashMap<String, Object>,
}

impl Instance {
    /// Returns the value of the field with the given name. If the field is not defined, it will
    /// return an error. If the field is not defined in this class, it will inherit the field from
    /// its superclass (if any).
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

    /// Sets the value of the field with the given name.
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
