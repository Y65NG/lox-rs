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

// pub struct Function {
//     params: Vec<Token>,
//     body: Vec<Stmt>,
// }

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
