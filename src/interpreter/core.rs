use super::{env::Environment, error::RuntimeError, types::*};
use crate::{
    ast::{Expr, Stmt, Visiter},
    lexer::Token,
};

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }
    pub fn interpret(&self, statements: &[Stmt]) {
        for stmt in statements {
            match self.visit_stmt(&stmt) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Panicked at {}", e.0);
                    eprintln!("{}", e.1);
                }
            }
        }
    }
}

impl Visiter for Interpreter {
    type Expr = Result<Type, RuntimeError>;
    type Stmt = Result<(), RuntimeError>;
    fn visit_expr(&self, expr: &Expr) -> Self::Expr {
        match expr {
            Expr::Literal { value } => match value {
                Token::Number(n) => Ok(Type::Number(*n)),
                Token::Str(s) => Ok(Type::String(s.to_string())),
                Token::True => Ok(Type::Boolean(true)),
                Token::False => Ok(Type::Boolean(false)),
                Token::Nil => Ok(Type::Nil),
                _ => Err(RuntimeError(value.clone(), "Unexpected token".to_string())),
            },
            Expr::Grouping { expression } => self.visit_expr(expression),
            Expr::Unary { operator, right } => {
                let right = self.visit_expr(right)?;
                match operator {
                    Token::Minus => {
                        if let Type::Number(n) = right {
                            Ok(Type::Number(-n))
                        } else {
                            Err(RuntimeError(
                                operator.clone(),
                                "Operand must be a number.".to_string(),
                            ))
                        }
                    }
                    Token::Bang => {
                        if let Type::Boolean(b) = right {
                            Ok(Type::Boolean(!b))
                        } else {
                            Err(RuntimeError(
                                operator.clone(),
                                "Operand must be a Boolean.".to_string(),
                            ))
                        }
                    }
                    _ => Err(RuntimeError(
                        operator.clone(),
                        "Unexpected token.".to_string(),
                    )),
                }
            }
            Expr::Variable { name } => Ok(self.environment.get(name)?),
            Expr::Assign { name, value } => {
                let value = self.visit_expr(&value)?;
                if let Err(e) = self.environment.assign(name.clone(), value.clone()) {
                    return Err(e);
                }
                Ok(value)
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let (left, right) = (self.visit_expr(left)?, self.visit_expr(right)?);
                match operator {
                    Token::Minus => {
                        if let (Type::Number(n1), Type::Number(n2)) = (left, right) {
                            Ok(Type::Number(n1 - n2))
                        } else {
                            Err(RuntimeError(
                                operator.clone(),
                                "Operand must be numbers.".to_string(),
                            ))
                        }
                    }
                    Token::Plus => match (left, right) {
                        (Type::Number(n1), Type::Number(n2)) => Ok(Type::Number(n1 + n2)),
                        (Type::String(s1), Type::String(s2)) => {
                            Ok(Type::String(format!("{}{}", s1, s2)))
                        }
                        _ => Err(RuntimeError(
                            operator.clone(),
                            "Operand must be both numbers or both strings.".to_string(),
                        )),
                    },
                    Token::Slash => {
                        if let (Type::Number(n1), Type::Number(n2)) = (left, right) {
                            Ok(Type::Number(n1 / n2))
                        } else {
                            Err(RuntimeError(
                                operator.clone(),
                                "Operand must be numbers.".to_string(),
                            ))
                        }
                    }
                    Token::Star => {
                        if let (Type::Number(n1), Type::Number(n2)) = (left, right) {
                            Ok(Type::Number(n1 * n2))
                        } else {
                            Err(RuntimeError(
                                operator.clone(),
                                "Operand must be numbers.".to_string(),
                            ))
                        }
                    }
                    Token::Greater => {
                        if let (Type::Number(n1), Type::Number(n2)) = (left, right) {
                            Ok(Type::Boolean(n1 > n2))
                        } else {
                            Err(RuntimeError(
                                operator.clone(),
                                "Operand must be numbers.".to_string(),
                            ))
                        }
                    }
                    Token::GreaterEqual => {
                        if let (Type::Number(n1), Type::Number(n2)) = (left, right) {
                            Ok(Type::Boolean(n1 >= n2))
                        } else {
                            Err(RuntimeError(
                                operator.clone(),
                                "Operand must be numbers.".to_string(),
                            ))
                        }
                    }
                    Token::Less => {
                        if let (Type::Number(n1), Type::Number(n2)) = (left, right) {
                            Ok(Type::Boolean(n1 < n2))
                        } else {
                            Err(RuntimeError(
                                operator.clone(),
                                "Operand must be numbers.".to_string(),
                            ))
                        }
                    }
                    Token::LessEqual => {
                        if let (Type::Number(n1), Type::Number(n2)) = (left, right) {
                            Ok(Type::Boolean(n1 <= n2))
                        } else {
                            Err(RuntimeError(
                                operator.clone(),
                                "Operand must be numbers.".to_string(),
                            ))
                        }
                    }
                    Token::BangEqual => match (left, right) {
                        (Type::Number(n1), Type::Number(n2)) => Ok(Type::Boolean(n1 != n2)),
                        (Type::String(s1), Type::String(s2)) => Ok(Type::Boolean(s1 != s2)),
                        (Type::Boolean(b1), Type::Boolean(b2)) => Ok(Type::Boolean(b1 != b2)),
                        (Type::Nil, Type::Nil) => Ok(Type::Boolean(false)),
                        _ => Ok(Type::Boolean(true)),
                    },
                    Token::EqualEqual => match (left, right) {
                        (Type::Number(n1), Type::Number(n2)) => Ok(Type::Boolean(n1 == n2)),
                        (Type::String(s1), Type::String(s2)) => Ok(Type::Boolean(s1 == s2)),
                        (Type::Boolean(b1), Type::Boolean(b2)) => Ok(Type::Boolean(b1 == b2)),
                        (Type::Nil, Type::Nil) => Ok(Type::Boolean(true)),
                        _ => Ok(Type::Boolean(false)),
                    },
                    _ => Err(RuntimeError(
                        operator.clone(),
                        "Unexpected token.".to_string(),
                    )),
                }
            }
            _ => Err(RuntimeError(Token::Eof, "Unexpected token.".to_string())),
        }
    }

    fn visit_stmt(&self, stmt: &Stmt) -> Self::Stmt {
        match stmt {
            Stmt::Expression { expression } => {
                self.visit_expr(expression);
                Ok(())
            }
            Stmt::Print { expression } => {
                // dbg!(expression);
                let value = self.visit_expr(expression)?;
                println!("{}", value);
                Ok(())
            }
            Stmt::Var { name, initializer } => {
                let mut value = Type::Nil;
                if let Some(initializer) = initializer {
                    value = self.visit_expr(initializer)?;
                }
                if let Token::Identifier(name) = name {
                    self.environment.define(name, value);
                }
                Ok(())
            }
            _ => Err(RuntimeError(Token::Eof, "Unexpected token.".to_string())),
        }
    }
}
