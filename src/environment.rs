use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::object::Object;
use crate::token::Token;
use crate::error::{RuntimeError, Error};

#[derive(Debug, Clone)]
pub struct Environment {
    pub enclosing: Option<Rc<RefCell<Environment>>>,
    pub variables: HashMap<String, Object>,
}

impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
        Environment {
            enclosing,
            variables: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &str, value: Object) {
        self.variables.insert(name.to_string(), value);
    }

    fn ancestor(&self, distance: usize) -> Rc<RefCell<Environment>> {
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

    pub fn assign_at(&mut self, distance: usize, name: &Token, value: Object) {
        if distance > 0 {
            self.ancestor(distance).borrow_mut().variables.insert(name.lexeme.clone(), value);
        } else {
            self.variables.insert(name.lexeme.clone(), value);
        }
    }

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

    pub fn get_at(&self, distance: usize, name: &Token) -> Result<Object, RuntimeError> {
        if distance > 0 {
            match self.ancestor(distance).borrow().variables.get(&name.lexeme) {
                Some(variable) => Ok(variable.clone()),
                None => Err(RuntimeError {
                    token: name.clone(),
                    message: format!("Undefined variable '{}'", name.lexeme),
                }),
            }
        } else {
            match self.variables.get(&name.lexeme) {
                Some(variable) => Ok(variable.clone()),
                None => Err(RuntimeError {
                    token: name.clone(),
                    message: format!("Undefined variable '{}'", name.lexeme),
                }),
            }
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new(None)
    }
}
