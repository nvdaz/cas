use std::fmt::Display;

use super::expr::Expr;

pub struct Eq(Box<dyn Expr>, Box<dyn Expr>);

impl Display for Eq {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{} = {}", self.0, self.1)
    }
}
