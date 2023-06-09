use super::{
    env::Environment,
    error::RuntimeError,
    types::{self, *},
};
use crate::{
    ast::{Expr, Stmt, Visiter},
    lexer::Token,
};
use std::rc::Rc;

pub struct Interpreter {
    pub globals: Environment,
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Environment::global();
        let environment = globals.clone();
        Self {
            globals,
            environment,
        }
    }
    pub fn interpret(&mut self, statements: &[Stmt]) -> Result<(), String> {
        for stmt in statements {
            match self.visit_stmt(&stmt) {
                Ok(_) => {}
                Err(ReturnValue::Err(e)) => {
                    // eprintln!("Panicked at {}", e.0);
                    return Err(e);
                }
                Err(ReturnValue::Return(_)) => {
                    return Err("Only functions can return values.".to_string());
                }
            }
        }
        Ok(())
    }

    pub fn execute_block(
        &mut self,
        block: &[Stmt],
        mut environment: Environment,
    ) -> Result<(), ReturnValue> {
        std::mem::swap(&mut self.environment, &mut environment);
        for stmt in block {
            match self.visit_stmt(stmt) {
                Ok(()) => Ok(()),
                Err(value) => match value {
                    ReturnValue::Err(e) => Err(ReturnValue::Err(e)),
                    ReturnValue::Return(t) => {
                        std::mem::swap(&mut self.environment, &mut environment);
                        return Err(ReturnValue::Return(t));
                    }
                },
            };
        }
        std::mem::swap(&mut self.environment, &mut environment);
        Ok(())
    }
}

impl Visiter for Interpreter {
    type Expr = Result<Type, RuntimeError>;
    type Stmt = Result<(), ReturnValue>;
    fn visit_expr(&mut self, expr: &Expr) -> Self::Expr {
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
            Expr::Logical {
                left,
                operator,
                right,
            } => {
                let left = self.visit_expr(left)?;
                match operator {
                    Token::Or => {
                        if left.is_true() {
                            return Ok(left);
                        }
                    }
                    Token::And => {
                        if !left.is_true() {
                            return Ok(left);
                        }
                    }
                    _ => unreachable!(),
                }
                Ok(self.visit_expr(right)?)
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
                    Token::Mod => {
                        if let (Type::Number(n1), Type::Number(n2)) = (left, right) {
                            Ok(Type::Number(n1 % n2))
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
            Expr::Call {
                callee,
                paren,
                arguments,
            } => {
                let callee = self.visit_expr(callee)?;
                let mut args = Vec::new();
                for arg in arguments {
                    args.push(self.visit_expr(arg)?);
                }
                match callee {
                    Type::Callable(func) => func.call(self, args),
                    _ => Err(RuntimeError(
                        paren.clone(),
                        "Can only call functions and classes.".to_string(),
                    )),
                }
            }
            _ => Err(RuntimeError(Token::Eof, "Unexpected token.".to_string())),
        }
    }

    fn visit_stmt(&mut self, stmt: &Stmt) -> Self::Stmt {
        match stmt {
            Stmt::Expression { expression } => {
                self.visit_expr(expression)?;
                Ok(())
            }
            Stmt::Function { name, params, body } => {
                let function = types::Function {
                    name: name.clone(),
                    params: params.clone(),
                    body: body.clone(),
                };
                self.environment.define(
                    if let Token::Identifier(ref n) = name {
                        n
                    } else {
                        unreachable!()
                    },
                    Type::Callable(Rc::new(function)),
                );
                Ok(())
            }
            Stmt::Print { expression } => {
                // dbg!(expression);
                let value = self.visit_expr(expression)?;
                match value {
                    Type::Nil => (),
                    _ => println!("{}", value),
                }
                Ok(())
            }
            Stmt::Return { keyword, value } => {
                if let Some(value) = value {
                    Err(ReturnValue::Return(self.visit_expr(value)?))
                } else {
                    Err(ReturnValue::Return(Type::Nil))
                }
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
            Stmt::For {
                initializer,
                condition,
                body,
            } => {
                let mut statements: Vec<Stmt> = Vec::new();
                statements.push(*initializer.clone());
                let while_loop = Stmt::While {
                    condition: condition.clone(),
                    body: body.clone(),
                };
                statements.push(while_loop);
                self.visit_stmt(&Stmt::Block { statements })?;
                Ok(())
            }
            Stmt::While { condition, body } => {
                while self.visit_expr(condition)?.is_true() {
                    self.visit_stmt(body)?;
                }
                Ok(())
            }
            Stmt::Block { statements } => {
                let mut local = Environment::new(Some(&self.environment));
                self.execute_block(statements, local)?;
                Ok(())
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                if let Ok(condition) = self.visit_expr(condition) {
                    match condition {
                        Type::Boolean(true) => {
                            self.visit_stmt(&then_branch)?;
                        }
                        Type::Boolean(false) => {
                            if let Some(else_branch) = else_branch {
                                self.visit_stmt(else_branch)?;
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                Ok(())
            }
            _ => Err(ReturnValue::Err("Unexpected token.".to_string())),
        }
    }
}
