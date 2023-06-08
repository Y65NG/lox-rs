use crate::ast::*;
use crate::lexer::Token;

pub struct AstPrinter;

impl AstPrinter {
    fn parenthesize(&self, operator: &str, exprs: &[&Expr]) -> String {
        let mut s = String::new();
        s.push_str("(");
        s.push_str(operator);
        for expr in exprs {
            s.push_str(" ");
            s.push_str(&self.visit_expr(*expr).as_str());
        }
        s.push_str(")");
        s
    }
}

impl Visiter for AstPrinter {
    type Expr = String;
    fn visit_expr(&self, expr: &Expr) -> Self::Expr {
        match *expr {
            Expr::Assign {
                ref name,
                ref value,
            } => self.parenthesize(&name.to_string(), &[&value]),
            Expr::Binary {
                ref left,
                ref operator,
                ref right,
            } => self.parenthesize(&operator.to_string(), &[&left, &right]),

            Expr::Grouping { ref expression } => self.parenthesize("group", &[&expression]),
            Expr::Literal { ref value } => match value {
                &Token::Nil => "nil".to_string(),
                &Token::Number(n) => n.to_string(),
                &Token::Str(ref s) => s.clone(),
                _ => value.to_string(),
            },
            Expr::Unary {
                ref operator,
                ref right,
            } => self.parenthesize(&operator.to_string(), &[&right]),
            Expr::Variable { ref name } => name.to_string(),
            _ => String::new(),
        }
    }
}

#[test]
fn test() {
    let expr = Expr::Binary {
        left: Box::new(Expr::Unary {
            operator: Token::Minus,
            right: Box::new(Expr::Literal {
                value: Token::Number(123.0),
            }),
        }),
        operator: Token::Star,
        right: Box::new(Expr::Grouping {
            expression: Box::new(Expr::Literal {
                value: Token::Number(48.0),
            }),
        }),
    };
    let printer = AstPrinter {};
    println!("{}", printer.visit_expr(&expr));
}
