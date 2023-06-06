use super::Expr;

pub trait Visiter {
    type Output;

    fn visit_expr(&self, expr: &Expr) -> Self::Output;
}
