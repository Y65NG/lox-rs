use crate::lexer::Token;

use super::native_functions::Clock;
use super::{types::Type, RuntimeError};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
pub struct Environment {
    env: Rc<RefCell<EnvNode>>,
}

impl Environment {
    pub fn new(enclosing: Option<&Environment>) -> Self {
        let enclosing_node = match enclosing {
            Some(env) => Some(Rc::clone(&env.env)),
            None => None,
        };
        Self {
            env: Rc::new(RefCell::new(EnvNode::new(enclosing_node))),
        }
    }

    pub fn global() -> Self {
        Self {
            env: Rc::new(RefCell::new(EnvNode::global())),
        }
    }

    pub fn set_enclosing(&mut self, enclosing: Environment) {
        self.env.borrow_mut().enclosing = Some(enclosing.env.clone());
    }

    pub fn assign(&self, name: Token, value: Type) -> Result<(), RuntimeError> {
        self.env.borrow_mut().assign(name, value)
    }

    pub fn define(&self, name: &str, value: Type) {
        self.env.borrow_mut().define(name, value)
    }

    pub fn get(&self, token: &Token) -> Result<Type, RuntimeError> {
        self.env.borrow().get(token)
    }
}

struct EnvNode {
    values: HashMap<String, Type>,
    enclosing: Option<Rc<RefCell<EnvNode>>>,
}

impl EnvNode {
    pub fn new(enclosing: Option<Rc<RefCell<EnvNode>>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing,
        }
    }

    pub fn global() -> Self {
        let mut global = EnvNode::new(None);
        let clock = Type::Callable(Rc::new(Clock {}));

        global.define("clock", clock);

        global
    }

    pub fn assign(&mut self, name: Token, value: Type) -> Result<(), RuntimeError> {
        if let Token::Identifier(ref var_name) = name {
            if self.values.contains_key(var_name) {
                self.values.insert(var_name.to_string(), value);
                return Ok(());
            } else if let Some(ref mut enclosing) = self.enclosing {
                return enclosing.borrow_mut().assign(name, value);
            } else {
                return Err(RuntimeError(
                    name.clone(),
                    format!("Undefined variable '{}'.", var_name),
                ));
            }
        }
        unreachable!()
    }

    pub fn define(&mut self, name: &str, value: Type) {
        self.values.insert(name.to_string(), value);
    }

    pub fn get(&self, token: &Token) -> Result<Type, RuntimeError> {
        if let Token::Identifier(name) = token {
            if let Some(result) = self.values.get(name) {
                return Ok(result.clone());
            } else if let Some(ref enclosing) = self.enclosing {
                return enclosing.borrow().get(token);
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
