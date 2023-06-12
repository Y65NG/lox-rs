use crate::interpreter::{types, Interpreter, Type, RuntimeError};
use std::{time::{SystemTime, UNIX_EPOCH}, fmt::Display};

#[derive(Debug)]
pub struct Clock {}

impl types::Callable for Clock {
    fn arity(&self) -> usize {
        0
    }
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Type>) -> Result<Type, RuntimeError> {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        Ok(Type::Number(time as f64))
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn>")
    }
}
