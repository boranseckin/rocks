use std::collections::HashMap;

use crate::token::{Literal, Token};
use crate::error::{RuntimeError, rloxError};

#[derive(Debug, Clone)]
pub struct Environment {
    pub enclosing: Option<Box<Environment>>,
    pub variables: HashMap<String, Literal>,
}

impl Environment {
    pub fn new(enclosing: Option<Box<Environment>>) -> Self {
        Environment {
            enclosing,
            variables: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &str, value: Literal) {
        self.variables.insert(name.to_owned(), value);
    }

    pub fn assign(&mut self, name: &Token, value: Literal) {
        if self.variables.contains_key(&name.lexeme) {
            self.variables.insert(name.lexeme.to_owned(), value);
            return
        }

        if let Some(enclosing) = &mut self.enclosing {
            enclosing.assign(name, value);
            return
        }

        RuntimeError {
            token: name.to_owned(),
            message: format!("Undefined variable '{}'", name.lexeme),
        }.throw();
    }

    pub fn get(&self, name: &Token) -> Result<Literal, RuntimeError> {
        if let Some(variable) = self.variables.get(&name.lexeme) {
            return Ok(variable.clone());
        }

        if let Some(enclosing) = &self.enclosing {
            return enclosing.get(name);
        }

        let message = format!("Undefined variable '{}'", name.lexeme);
        Err(RuntimeError { token: name.clone(), message })
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Type;

    #[test]
    fn test_define() {
        let mut env = Environment::new(None);
        env.define("a", Literal::Number(1.0));
        env.define("b", Literal::Number(2.0));

        assert_eq!(env.variables.get("a").unwrap(), &Literal::Number(1.0));
        assert_eq!(env.variables.get("b").unwrap(), &Literal::Number(2.0));
    }


    #[test]
    fn test_get() {
        let mut env = Environment::new(None);
        env.define("a", Literal::Number(1.0));
        env.define("b", Literal::Number(2.0));

        assert_eq!(env.get(&Token::new(Type::Identifier, "a".to_string(), None, 1)).unwrap(), Literal::Number(1.0));
        assert_eq!(env.get(&Token::new(Type::Identifier, "b".to_string(), None, 1)).unwrap(), Literal::Number(2.0));
    }

    #[test]
    fn test_get_undefined() {
        let mut env = Environment::new(None);
        env.define("a", Literal::Number(1.0));
        env.define("b", Literal::Number(2.0));

        let mut env2 = Environment::new(Some(Box::new(env)));
        env2.define("c", Literal::Number(3.0));
        env2.define("d", Literal::Number(4.0));

        assert!(env2.get(&Token::new(Type::Identifier, "e".to_string(), None, 1)).is_err());
    }

    #[test]
    fn test_get_enclosing() {
        let mut env = Environment::new(None);
        env.define("a", Literal::Number(1.0));
        env.define("b", Literal::Number(2.0));

        let mut env2 = Environment::new(Some(Box::new(env)));
        env2.define("c", Literal::Number(3.0));
        env2.define("d", Literal::Number(4.0));

        assert_eq!(env2.get(&Token::new(Type::Identifier, "a".to_string(), None, 1)).unwrap(), Literal::Number(1.0));
        assert_eq!(env2.get(&Token::new(Type::Identifier, "b".to_string(), None, 1)).unwrap(), Literal::Number(2.0));
        assert_eq!(env2.get(&Token::new(Type::Identifier, "c".to_string(), None, 1)).unwrap(), Literal::Number(3.0));
        assert_eq!(env2.get(&Token::new(Type::Identifier, "d".to_string(), None, 1)).unwrap(), Literal::Number(4.0));
    }

    #[test]
    fn test_get_shadowing() {
        let mut env = Environment::new(None);
        env.define("a", Literal::Number(1.0));
        env.define("b", Literal::Number(2.0));

        let mut env2 = Environment::new(Some(Box::new(env)));
        env2.define("a", Literal::Number(3.0));
        env2.define("b", Literal::Number(4.0));

        assert_eq!(env2.get(&Token::new(Type::Identifier, "a".to_string(), None, 1)).unwrap(), Literal::Number(3.0));
        assert_eq!(env2.get(&Token::new(Type::Identifier, "b".to_string(), None, 1)).unwrap(), Literal::Number(4.0));
    }

    #[test]
    fn test_assign() {
        let mut env = Environment::new(None);
        env.define("a", Literal::Number(1.0));
        env.define("b", Literal::Number(2.0));

        env.assign(&Token::new(Type::Identifier, "a".to_string(), None, 1), Literal::Number(3.0));
        env.assign(&Token::new(Type::Identifier, "b".to_string(), None, 1), Literal::Number(4.0));

        assert_eq!(env.variables.get("a").unwrap(), &Literal::Number(3.0));
        assert_eq!(env.variables.get("b").unwrap(), &Literal::Number(4.0));
    }

    #[test]
    fn test_assign_undefined() {
        let mut env = Environment::new(None);
        env.define("a", Literal::Number(1.0));
        env.define("b", Literal::Number(2.0));

        let mut env2 = Environment::new(Some(Box::new(env)));
        env2.define("c", Literal::Number(3.0));
        env2.define("d", Literal::Number(4.0));

        env2.assign(&Token::new(Type::Identifier, "e".to_string(), None, 1), Literal::Number(5.0));
    }

    #[test]
    fn test_assign_enclosing() {
        let mut env = Environment::new(None);
        env.define("a", Literal::Number(1.0));
        env.define("b", Literal::Number(2.0));

        let mut env2 = Environment::new(Some(Box::new(env)));
        env2.define("c", Literal::Number(3.0));
        env2.define("d", Literal::Number(4.0));

        env2.assign(&Token::new(Type::Identifier, "a".to_string(), None, 1), Literal::Number(5.0));
        env2.assign(&Token::new(Type::Identifier, "b".to_string(), None, 1), Literal::Number(6.0));
        env2.assign(&Token::new(Type::Identifier, "c".to_string(), None, 1), Literal::Number(7.0));
        env2.assign(&Token::new(Type::Identifier, "d".to_string(), None, 1), Literal::Number(8.0));

        assert_eq!(env2.get(&Token::new(Type::Identifier, "a".to_string(), None, 1)).unwrap(), Literal::Number(5.0));
        assert_eq!(env2.get(&Token::new(Type::Identifier, "b".to_string(), None, 1)).unwrap(), Literal::Number(6.0));
        assert_eq!(env2.get(&Token::new(Type::Identifier, "c".to_string(), None, 1)).unwrap(), Literal::Number(7.0));
        assert_eq!(env2.get(&Token::new(Type::Identifier, "d".to_string(), None, 1)).unwrap(), Literal::Number(8.0));
    }

    #[test]
    fn test_assign_shadowing() {
        let mut env = Environment::new(None);
        env.define("a", Literal::Number(1.0));
        env.define("b", Literal::Number(2.0));

        let mut env2 = Environment::new(Some(Box::new(env)));
        env2.define("a", Literal::Number(3.0));
        env2.define("b", Literal::Number(4.0));

        env2.assign(&Token::new(Type::Identifier, "a".to_string(), None, 1), Literal::Number(5.0));
        env2.assign(&Token::new(Type::Identifier, "b".to_string(), None, 1), Literal::Number(6.0));

        assert_eq!(env2.get(&Token::new(Type::Identifier, "a".to_string(), None, 1)).unwrap(), Literal::Number(5.0));
        assert_eq!(env2.get(&Token::new(Type::Identifier, "b".to_string(), None, 1)).unwrap(), Literal::Number(6.0));
    }
}

