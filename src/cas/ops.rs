use super::{expr::Expr, terms::Num};
use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Debug)]
pub struct Sum(pub Vec<Box<dyn Expr>>);

impl Expr for Sum {
    fn differentiate(&self) -> Box<dyn Expr> {
        Box::new(Sum(self.0.iter().map(|e| e.differentiate()).collect()))
    }
}

impl Display for Sum {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        write!(
            fmt,
            "({})",
            self.0
                .iter()
                .map(|e| format!("{}", e))
                .collect::<Vec<String>>()
                .join(" + ")
        )
    }
}

#[derive(Clone, Debug)]
pub struct Mul(pub Vec<Box<dyn Expr>>);

impl Expr for Mul {
    fn differentiate(&self) -> Box<dyn Expr> {
        let mut sum: Vec<Box<dyn Expr>> = vec![];

        for (i, e) in self.0.iter().enumerate() {
            let mut mul: Vec<Box<dyn Expr>> = vec![];
            mul.push(e.differentiate());
            for (j, b) in self.0.iter().enumerate() {
                if i != j {
                    mul.push(b.clone());
                }
            }

            sum.push(Box::new(Mul(mul)));
        }

        Box::new(Sum(sum))
    }
}

impl Display for Mul {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        write!(
            fmt,
            "({})",
            self.0
                .iter()
                .map(|e| format!("{}", e))
                .collect::<Vec<String>>()
                .join(" * ")
        )
    }
}

#[derive(Clone, Debug)]
pub struct Neg(pub Box<dyn Expr>);

impl Expr for Neg {
    fn differentiate(&self) -> Box<dyn Expr> {
        todo!()
    }
}

impl Display for Neg {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        write!(fmt, "-({})", self.0)
    }
}

#[derive(Clone, Debug)]
pub struct Pow(pub Box<dyn Expr>, pub i128);

impl Expr for Pow {
    fn differentiate(&self) -> Box<dyn Expr> {
        Box::new(Mul(vec![
            Box::new(Num(self.1)),
            self.0.differentiate(),
            Box::new(Pow(self.0.clone(), self.1 - 1)),
        ]))
    }
}

impl Display for Pow {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        write!(fmt, "({})^({})", self.0, self.1)
    }
}
