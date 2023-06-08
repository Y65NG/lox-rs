use super::Expr;
use super::Stmt;

pub trait Visiter {
    type Expr;
    type Stmt;

    fn visit_expr(&self, expr: &Expr) -> Self::Expr;

    fn visit_stmt(&self, stmt: &Stmt) -> Self::Stmt;
}
