use std::fmt::Debug;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

use crate::object::{Object, Shared};
use crate::token::Token;
use crate::error::{RuntimeError, Error};

/// Represents an environment in which variables are stored.
/// The environment is a hash map of variable names to their values.
/// Each environment has a reference to its enclosing environment.
/// This is an optional reference to implement lexical scoping and closures.
#[derive(Clone)]
pub struct Environment {
    /// Using an Rc and Refcell here allows us to have multiple mutable references
    /// to the same environment.
    pub enclosing: Option<Shared<Environment>>,
    variables: HashMap<String, Object>,
}

impl Environment {
    /// Creates a new environment with the given enclosing environment.
    pub fn new(enclosing: Option<Shared<Environment>>) -> Self {
        Environment {
            enclosing,
            variables: HashMap::new(),
        }
    }

    pub fn as_shared(self) -> Shared<Self> {
        Rc::new(RefCell::new(self))
    }

    /// Defines a new variable in the environment with the given name and value.
    pub fn define(&mut self, name: &str, value: Object) {
        self.variables.insert(name.to_string(), value);
    }

    /// Accesses the ancestor environment at the given distance.
    fn ancestor(&self, distance: usize) -> Shared<Environment> {
        let parent = self.enclosing.clone()
            .unwrap_or_else(|| panic!("enclosing environment to exist at depth {}", 1));
        let mut environment = Rc::clone(&parent);

        for i in 1..distance {
            let parent = environment.borrow().enclosing.clone()
                .unwrap_or_else(|| panic!("enclosing environment to exist at depth {}", i));
            environment = Rc::clone(&parent);
        }

        environment
    }

    /// Assigns the given value to the variable with the given name.
    /// If the variable is not define in this environment but is defined in an enclosing environment,
    /// it will try to recursively assign the value to the variable in the enclosing environment.
    /// If the variable is not defined in this environment or any enclosing environment, it will
    /// throw a runtime error.
    pub fn assign(&mut self, name: &Token, value: Object) {
        if self.variables.contains_key(&name.lexeme) {
            self.variables.insert(name.lexeme.clone(), value);
            return;
        }

        if let Some(enclosing) = &mut self.enclosing {
            enclosing.borrow_mut().assign(name, value);
            return;
        }

        RuntimeError {
            token: name.clone(),
            message: format!("Undefined variable '{}'", name.lexeme),
        }.throw();
    }

    /// Works like [`Environment::assign`] but assigns the value to the variable in the ancestor
    /// environment at the given distance.
    pub fn assign_at(&mut self, distance: usize, name: &Token, value: Object) {
        if distance == 0 {
            return self.assign(name, value);
        }

        self.ancestor(distance).borrow_mut().assign(name, value);
    }

    /// Returns the value of the variable with the given name.
    /// If the variable is not defined in this environment but is defined in an enclosing environment,
    /// it will try to recursively get the value of the variable in the enclosing environment.
    /// If the variable is not defined in this environment or any enclosing environment, it will
    /// throw a runtime error.
    pub fn get(&self, name: &Token) -> Result<Object, RuntimeError> {
        if let Some(variable) = self.variables.get(&name.lexeme) {
            return Ok(variable.clone());
        }

        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow().get(name);
        }

        Err(RuntimeError {
            token: name.clone(),
            message: format!("Undefined variable '{}'", name.lexeme)
        })
    }

    /// Works like [`Environment::get`] but gets the value of the variable in the ancestor
    /// environment at the given distance.
    pub fn get_at(&self, distance: usize, name: &Token) -> Result<Object, RuntimeError> {
        if distance == 0 {
            return self.get(name);
        }

        return self.ancestor(distance).borrow().get(name);
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new(None)
    }
}

impl Debug for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Environment")
            .field("enclosing", &self.enclosing)
            .field("variables", &self.variables.keys())
            .finish()
    }
}
