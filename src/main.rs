use crate::cas::{
    expr::Expr,
    ops::Mul,
    terms::{Num, Var},
    trig::{Cot, Sec},
};

pub mod cas;

fn main() {
    let e = Mul(vec![
        Box::new(Var("x".to_string())),
        Box::new(Num(3)),
        Box::new(Cot(Box::new(Sec(Box::new(Var("x".to_string())))))),
    ]);

    println!("{}", e);
    println!("{}", e.differentiate());
}
