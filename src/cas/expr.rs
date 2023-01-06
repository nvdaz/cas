use dyn_clone::{clone_trait_object, DynClone};
use std::fmt::{Debug, Display};

pub trait Expr: Debug + Display + DynClone {
    fn differentiate(&self) -> Box<dyn Expr>;
}

clone_trait_object!(Expr);
