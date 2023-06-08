use crate::lexer::Token;

use super::{types::Type, RuntimeError};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Environment {
    values: Rc<RefCell<HashMap<String, Type>>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn assign(&self, name: Token, value: Type) -> Result<(), RuntimeError> {
        if let Token::Identifier(ref var_name) = name {
            if self.values.borrow().contains_key(var_name) {
                self.values.borrow_mut().insert(var_name.to_string(), value);
                return Ok(());
            } else {
                return Err(RuntimeError(
                    name.clone(),
                    format!("Undefined variable '{}'.", var_name),
                ));
            }
        }
        !unreachable!()
    }

    pub fn define(&self, name: &str, value: Type) {
        self.values.borrow_mut().insert(name.to_string(), value);
    }

    pub fn get(&self, token: &Token) -> Result<Type, RuntimeError> {
        if let Token::Identifier(name) = token {
            if let Some(result) = self.values.borrow().get(name) {
                return Ok(result.clone());
            } else {
                return Err(RuntimeError(
                    token.clone(),
                    format!("Undefined variable '{}'.", name.clone()),
                ));
            }
        } else {
            Err(RuntimeError(
                token.clone(),
                "Expect variable name.".to_string(),
            ))
        }
    }
}
