use std::fmt::{Display, Result};

use super::expr::Expr;

#[derive(Clone, Debug)]
pub struct Num(pub i128);

impl Expr for Num {
    fn differentiate(&self) -> Box<dyn Expr + 'static> {
        Box::new(Num(0))
    }
}

impl Display for Num {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result {
        write!(fmt, "{}", self.0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Var(pub String);

impl Expr for Var {
    fn differentiate(&self) -> Box<dyn Expr + 'static> {
        Box::new(Num(1))
    }
}

impl Display for Var {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result {
        write!(fmt, "{}", self.0)
    }
}
