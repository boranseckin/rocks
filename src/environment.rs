use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::object::Object;
use crate::token::Token;
use crate::error::{RuntimeError, rloxError};

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
            .expect(&format!("enclosing environment to exist at depth {}", 1));
        let mut environment = Rc::clone(&parent);

        for i in 1..distance {
            let parent = environment.borrow().enclosing.clone()
                .expect(&format!("enclosing environment to exist at depth {}", i));
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

        let message = format!("Undefined variable '{}'", name.lexeme);
        Err(RuntimeError { token: name.clone(), message })
    }

    pub fn get_at(&self, distance: usize, name: &Token) -> Result<Object, RuntimeError> {
        if distance > 0 {
            match self.ancestor(distance).borrow().variables.get(&name.lexeme) {
                Some(variable) => Ok(variable.clone()),
                None => Err(RuntimeError {
                    token: name.clone(),
                    message: format!("Undefined variable {} '{}'", distance, name.lexeme),
                }),
            }
        } else {
            match self.variables.get(&name.lexeme) {
                Some(variable) => Ok(variable.clone()),
                None => Err(RuntimeError {
                    token: name.clone(),
                    message: format!("Undefined variable {} '{}'", distance, name.lexeme),
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::token::Type;

    #[test]
    fn default() {
        let env = Environment::default();
        assert!(env.enclosing.is_none());
        assert!(env.variables.is_empty());
    }

    #[test]
    fn test_define() {
        let mut env = Environment::new(None);
        env.define("a", Object::from(1.0));
        env.define("b", Object::from(2.0));

        assert_eq!(env.variables.get("a").unwrap(), &Object::from(1.0));
        assert_eq!(env.variables.get("b").unwrap(), &Object::from(2.0));
    }

    #[test]
    fn test_get() {
        let mut env = Environment::new(None);
        env.define("a", Object::from(1.0));
        env.define("b", Object::from(2.0));

        assert_eq!(env.get(&Token::new(Type::Identifier, "a".to_string(), None, 1)).unwrap(), Object::from(1.0));
        assert_eq!(env.get(&Token::new(Type::Identifier, "b".to_string(), None, 1)).unwrap(), Object::from(2.0));
    }

    #[test]
    fn test_get_undefined() {
        let mut env = Environment::new(None);
        env.define("a", Object::from(1.0));
        env.define("b", Object::from(2.0));

        let mut env2 = Environment::new(Some(Rc::new(RefCell::new(env))));
        env2.define("c", Object::from(3.0));
        env2.define("d", Object::from(4.0));

        assert!(env2.get(&Token::new(Type::Identifier, "e".to_string(), None, 1)).is_err());
    }

    #[test]
    fn test_get_enclosing() {
        let mut env = Environment::new(None);
        env.define("a", Object::from(1.0));
        env.define("b", Object::from(2.0));

        let mut env2 = Environment::new(Some(Rc::new(RefCell::new(env))));
        env2.define("c", Object::from(3.0));
        env2.define("d", Object::from(4.0));

        assert_eq!(env2.get(&Token::new(Type::Identifier, "a".to_string(), None, 1)).unwrap(), Object::from(1.0));
        assert_eq!(env2.get(&Token::new(Type::Identifier, "b".to_string(), None, 1)).unwrap(), Object::from(2.0));
        assert_eq!(env2.get(&Token::new(Type::Identifier, "c".to_string(), None, 1)).unwrap(), Object::from(3.0));
        assert_eq!(env2.get(&Token::new(Type::Identifier, "d".to_string(), None, 1)).unwrap(), Object::from(4.0));
    }

    #[test]
    fn test_get_shadowing() {
        let mut env = Environment::new(None);
        env.define("a", Object::from(1.0));
        env.define("b", Object::from(2.0));

        let mut env2 = Environment::new(Some(Rc::new(RefCell::new(env))));
        env2.define("a", Object::from(3.0));
        env2.define("b", Object::from(4.0));

        assert_eq!(env2.get(&Token::new(Type::Identifier, "a".to_string(), None, 1)).unwrap(), Object::from(3.0));
        assert_eq!(env2.get(&Token::new(Type::Identifier, "b".to_string(), None, 1)).unwrap(), Object::from(4.0));
    }

    #[test]
    fn test_assign() {
        let mut env = Environment::new(None);
        env.define("a", Object::from(1.0));
        env.define("b", Object::from(2.0));

        env.assign(&Token::new(Type::Identifier, "a".to_string(), None, 1), Object::from(3.0));
        env.assign(&Token::new(Type::Identifier, "b".to_string(), None, 1), Object::from(4.0));

        assert_eq!(env.variables.get("a").unwrap(), &Object::from(3.0));
        assert_eq!(env.variables.get("b").unwrap(), &Object::from(4.0));
    }

    #[test]
    fn test_assign_undefined() {
        let mut env = Environment::new(None);
        env.define("a", Object::from(1.0));
        env.define("b", Object::from(2.0));

        env.assign(&Token::new(Type::Identifier, "c".to_string(), None, 1), Object::from(3.0));
        assert!(env.variables.get("c").is_none());
   }

    #[test]
    fn test_assign_enclosing() {
        let mut env = Environment::new(None);
        env.define("a", Object::from(1.0));
        env.define("b", Object::from(2.0));

        let mut env2 = Environment::new(Some(Rc::new(RefCell::new(env))));
        env2.define("c", Object::from(3.0));
        env2.define("d", Object::from(4.0));

        env2.assign(&Token::new(Type::Identifier, "a".to_string(), None, 1), Object::from(5.0));
        env2.assign(&Token::new(Type::Identifier, "b".to_string(), None, 1), Object::from(6.0));
        env2.assign(&Token::new(Type::Identifier, "c".to_string(), None, 1), Object::from(7.0));
        env2.assign(&Token::new(Type::Identifier, "d".to_string(), None, 1), Object::from(8.0));

        assert_eq!(env2.get(&Token::new(Type::Identifier, "a".to_string(), None, 1)).unwrap(), Object::from(5.0));
        assert_eq!(env2.get(&Token::new(Type::Identifier, "b".to_string(), None, 1)).unwrap(), Object::from(6.0));
        assert_eq!(env2.get(&Token::new(Type::Identifier, "c".to_string(), None, 1)).unwrap(), Object::from(7.0));
        assert_eq!(env2.get(&Token::new(Type::Identifier, "d".to_string(), None, 1)).unwrap(), Object::from(8.0));
    }

    #[test]
    fn test_assign_shadowing() {
        let mut env = Environment::new(None);
        env.define("a", Object::from(1.0));
        env.define("b", Object::from(2.0));

        let mut env2 = Environment::new(Some(Rc::new(RefCell::new(env))));
        env2.define("a", Object::from(3.0));
        env2.define("b", Object::from(4.0));

        env2.assign(&Token::new(Type::Identifier, "a".to_string(), None, 1), Object::from(5.0));
        env2.assign(&Token::new(Type::Identifier, "b".to_string(), None, 1), Object::from(6.0));

        assert_eq!(env2.get(&Token::new(Type::Identifier, "a".to_string(), None, 1)).unwrap(), Object::from(5.0));
        assert_eq!(env2.get(&Token::new(Type::Identifier, "b".to_string(), None, 1)).unwrap(), Object::from(6.0));
    }
}

