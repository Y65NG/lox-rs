use crate::ast::{Expr, Stmt};
use crate::lexer::Token::{self, *};

use std::cell::Cell;

pub struct Parser {
    tokens: Vec<Token>,
    current: Cell<usize>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: Cell::new(0),
        }
    }

    pub fn parse(&self) -> Result<Vec<Stmt>, &'static str> {
        let mut statements = Vec::new();
        while let Some(t) = self.peek() {
            if let Eof = t {
                break;
            }
            statements.push(self.declaration()?);
        }
        Ok(statements)
    }

    // SECTION - Statements
    fn declaration(&self) -> Result<Stmt, &'static str> {
        match self.peek().expect("Current token is None") {
            Var => self.var_declaration(),
            _ => self.statement(),
        }
    }

    fn var_declaration(&self) -> Result<Stmt, &'static str> {
        self.advance();
        if let Some(Identifier(name)) = self.advance() {
            let mut initializer = None;
            if let Some(Equal) = self.peek() {
                self.advance();
                initializer = Some(self.expression()?);
            }
            if let Some(Semicolon) = self.advance() {
                return Ok(Stmt::Var {
                    name: Identifier(name.to_string()),
                    initializer,
                });
            } else {
                return Err("Expect ';' after variable declaration.");
            }
        } else {
            return Err("Expect variable name.");
        }
    }

    fn statement(&self) -> Result<Stmt, &'static str> {
        match self.peek().expect("Current token is None") {
            Print => self.print_statement(),
            _ => self.expr_statement(),
        }
    }

    fn print_statement(&self) -> Result<Stmt, &'static str> {
        self.advance();
        let value = self.expression()?;
        if let Some(Semicolon) = self.peek() {
            self.advance();
            Ok(Stmt::Print { expression: value })
        } else {
            Err("Expect ';' after value.")
        }
    }

    fn expr_statement(&self) -> Result<Stmt, &'static str> {
        let expr = self.expression()?;
        if let Some(Semicolon) = self.peek() {
            self.advance();
            Ok(Stmt::Expression { expression: expr })
        } else {
            Err("Expect ';' after expression.")
        }
    }

    // SECTION - Expressions
    fn expression(&self) -> Result<Expr, &'static str> {
        self.assignment()
    }

    fn assignment(&self) -> Result<Expr, &'static str> {
        let expr = self.equality()?;
        if let Some(Token::Equal) = self.peek() {
            self.advance();
            let value = self.assignment()?;

            if let Expr::Variable { ref name } = expr {
                return Ok(Expr::Assign { name: name.clone(), value: Box::new(value) })
            } else {
                return Err("Invalid assignment target.");
            }

        }
        Ok(expr)
    }

    fn equality(&self) -> Result<Expr, &'static str> {
        let mut expr = self.comparison()?;

        while let Some(operator) = match self.peek() {
            Some(&EqualEqual) | Some(&BangEqual) => self.advance(),
            _ => None,
        } {
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn comparison(&self) -> Result<Expr, &'static str> {
        let mut expr = self.term()?;

        while let Some(operator) = match self.peek() {
            Some(&Greater) | Some(&GreaterEqual) | Some(&Less) | Some(&LessEqual) => self.advance(),
            _ => None,
        } {
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn term(&self) -> Result<Expr, &'static str> {
        let mut expr = self.factor()?;

        while let Some(operator) = match self.peek() {
            Some(&Minus) | Some(&Plus) => self.advance(),
            _ => None,
        } {
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn factor(&self) -> Result<Expr, &'static str> {
        let mut expr = self.unary()?;

        while let Some(operator) = match self.peek() {
            Some(&Slash) | Some(&Star) => self.advance(),
            _ => None,
        } {
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator.clone(),
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn unary(&self) -> Result<Expr, &'static str> {
        match self.peek() {
            Some(&Bang) | Some(&Minus) => {
                let operator = self.advance().unwrap();
                let right = self.unary()?;
                return Ok(Expr::Unary {
                    operator: operator.clone(),
                    right: Box::new(right),
                });
            }
            _ => self.primary(),
        }
    }

    fn primary(&self) -> Result<Expr, &'static str> {
        let t = self.peek();
        match t {
            Some(False) => {
                self.advance();
                return Ok(Expr::Literal { value: False });
            }
            Some(True) => {
                self.advance();
                return Ok(Expr::Literal { value: True });
            }
            Some(Nil) => {
                self.advance();
                return Ok(Expr::Literal { value: Nil });
            }
            Some(Number(n)) => {
                self.advance();
                return Ok(Expr::Literal { value: Number(*n) });
            }
            Some(Str(ref s)) => {
                self.advance();
                return Ok(Expr::Literal {
                    value: Str(s.to_string()),
                });
            }
            Some(Identifier(name)) => {
                self.advance();
                Ok(Expr::Variable {
                    name: Identifier(name.to_string()),
                })
            }
            Some(LeftParen) => {
                self.advance();
                let expr = self.expression();
                if let Some(RightParen) = self.peek() {
                    self.advance();
                } else {
                    return Err("Expect ')' after expression.");
                }
                return Ok(Expr::Grouping {
                    expression: Box::new(expr?),
                });
            }
            _ => Err("Expect expression."),
        }
    }

    // SECTION - Helpers
    /// Discard tokens until the parser has found a statement boundary.
    fn synchronize(&self) {
        self.advance();

        while let Some(t) = self.peek() {
            if let &Eof = t {
                break;
            }
            if let Some(&Semicolon) = self.previous() {
                return;
            }
            match t {
                Class | Fun | Var | For | If | While | Print | Return => return,
                _ => self.advance(),
            };
        }
    }

    fn advance(&self) -> Option<&Token> {
        if self.current.get() >= self.tokens.len() {
            return None;
        }
        self.current.set(self.current.get() + 1);
        self.previous()
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current.get())
    }

    fn previous(&self) -> Option<&Token> {
        self.tokens.get(self.current.get() - 1)
    }
}
