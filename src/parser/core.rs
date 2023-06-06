use crate::ast::Expr;
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

    pub fn parse(&self) -> Result<Expr, &'static str> {
        self.expression()
    }

    // SECTION - Expressions
    fn expression(&self) -> Result<Expr, &'static str> {
        self.equality()
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
