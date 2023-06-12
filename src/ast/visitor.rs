use super::Expr;
use super::Stmt;

pub trait Visiter {
    type Expr;
    type Stmt;

    fn visit_expr(&mut self, expr: &Expr) -> Self::Expr;

    fn visit_stmt(&mut self, stmt: &Stmt) -> Self::Stmt;
}
