use crate::{Var, Gen};

pub enum Par {
    Con(f64),
    Var(Var),
    Fun(Gen)
}