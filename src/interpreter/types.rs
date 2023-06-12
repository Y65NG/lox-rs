use crate::ast::Stmt;
use crate::lexer::Token;

use super::env::Environment;
use super::{Interpreter, RuntimeError};
use std::fmt::{Debug, Display};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Type {
    Number(f64),
    String(String),
    Boolean(bool),
    Callable(Rc<dyn Callable>),
    // Class(Box<Class>),
    // Instance(Box<Instance>),
    Nil,
}

impl Type {
    pub fn is_true(&self) -> bool {
        match self {
            Self::Number(n) => n.is_normal(),
            Self::Boolean(b) => *b,
            Self::String(s) => !s.is_empty(),
            _ => false,
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Number(n) => write!(f, "{}", n),
            Type::String(s) => write!(f, "{}", s),
            Type::Boolean(b) => write!(f, "{}", b),
            Type::Callable(c) => write!(f, "{:?}", c),
            Type::Nil => write!(f, "nil"),
        }
    }
}

pub trait Callable: Debug + Display {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Type>) -> Result<Type, RuntimeError>;
}

#[derive(Debug)]
pub struct Function {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Box<Stmt>,
}

impl Callable for Function {
    fn arity(&self) -> usize {
        self.params.len()
    }

    fn call(&self, interpreter: &mut Interpreter, args: Vec<Type>) -> Result<Type, RuntimeError> {
        let local = Environment::new(Some(&interpreter.globals));
        for (i, arg) in args.into_iter().enumerate() {
            local.define(
                if let Token::Identifier(ref name) = self.params[i] {
                    name
                } else {
                    unreachable!()
                },
                arg,
            );
        }

        match interpreter.execute_block(
            if let Stmt::Block { ref statements } = *self.body {
                statements
            } else {
                unreachable!()
            },
            local,
        ) {
            Ok(()) => Ok(Type::Nil),
            Err(value) => match value {
                ReturnValue::Err(e) => Err(RuntimeError(self.name.clone(), e)),
                ReturnValue::Return(t) => Ok(t)
            },
        }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<fn {}>", self.name)
    }
}

pub enum ReturnValue {
    Err(String),
    Return(Type),
}

impl From<RuntimeError> for ReturnValue {
    fn from(value: RuntimeError) -> Self {
        ReturnValue::Err(value.1)
    }
}
