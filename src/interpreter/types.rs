use crate::lexer::Token::{self, *};

#[derive(Debug, Clone)]
pub enum Type {
    Number(f64),
    String(String),
    Boolean(bool),
    // Function(Box<Function>),
    // Class(Box<Class>),
    // Instance(Box<Instance>),
    Nil,
}

impl Type {
    pub fn is_true(&self) -> bool {
        match self {
            Self::Number(n) => {
                n.is_normal()
            }
            Self::Boolean(b) => *b,
            Self::String(s) => !s.is_empty(),
            _ => false
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Number(n) => write!(f, "{}", n),
            Type::String(s) => write!(f, "{}", s),
            Type::Boolean(b) => write!(f, "{}", b),
            Type::Nil => write!(f, "nil"),
        }
    }
}
