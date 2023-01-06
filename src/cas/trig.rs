use std::fmt::{Display, Formatter, Result};

use super::{
    expr::Expr,
    ops::{Mul, Neg, Pow},
};

#[derive(Clone, Debug)]
pub struct Sin(pub Box<dyn Expr>);

impl Expr for Sin {
    fn differentiate(&self) -> Box<dyn Expr> {
        Box::new(Mul(vec![
            self.0.differentiate(),
            Box::new(Cos(self.0.clone())),
        ]))
    }
}

impl Display for Sin {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        write!(fmt, "sin({})", self.0)
    }
}

#[derive(Clone, Debug)]
pub struct Cos(pub Box<dyn Expr>);

impl Expr for Cos {
    fn differentiate(&self) -> Box<dyn Expr> {
        Box::new(Neg(Box::new(Mul(vec![
            self.0.differentiate(),
            Box::new(Sin(self.0.clone())),
        ]))))
    }
}

impl Display for Cos {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        write!(fmt, "cos({})", self.0)
    }
}

#[derive(Clone, Debug)]
pub struct Sec(pub Box<dyn Expr>);

impl Expr for Sec {
    fn differentiate(&self) -> Box<dyn Expr> {
        Box::new(Mul(vec![
            self.0.differentiate(),
            Box::new(Sec(self.0.clone())),
            Box::new(Tan(self.0.clone())),
        ]))
    }
}

impl Display for Sec {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        write!(fmt, "sec({})", self.0)
    }
}

#[derive(Clone, Debug)]
pub struct Csc(pub Box<dyn Expr>);

impl Expr for Csc {
    fn differentiate(&self) -> Box<dyn Expr> {
        Box::new(Neg(Box::new(Mul(vec![
            self.0.differentiate(),
            Box::new(Csc(self.0.clone())),
            Box::new(Cot(self.0.clone())),
        ]))))
    }
}

impl Display for Csc {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        write!(fmt, "csc({})", self.0)
    }
}

#[derive(Clone, Debug)]
pub struct Tan(pub Box<dyn Expr>);

impl Expr for Tan {
    fn differentiate(&self) -> Box<dyn Expr> {
        Box::new(Mul(vec![
            self.0.differentiate(),
            Box::new(Pow(Box::new(Sec(self.0.clone())), 2)),
        ]))
    }
}

impl Display for Tan {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        write!(fmt, "tan({})", self.0)
    }
}

#[derive(Clone, Debug)]
pub struct Cot(pub Box<dyn Expr>);

impl Expr for Cot {
    fn differentiate(&self) -> Box<dyn Expr> {
        Box::new(Neg(Box::new(Mul(vec![
            self.0.differentiate(),
            Box::new(Pow(Box::new(Csc(self.0.clone())), 2)),
        ]))))
    }
}

impl Display for Cot {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        write!(fmt, "cot({})", self.0)
    }
}
