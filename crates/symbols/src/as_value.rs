use crate::{Value, Var};

pub trait AsValue {
    fn value(self) -> Value;
}

impl AsValue for Value {
    fn value(self) -> Value {
        self
    }
}

impl AsValue for f32 {
    fn value(self) -> Value {
        self.into()
    }
}

impl AsValue for Var {
    fn value(self) -> Value {
        self.into()
    }
}

pub trait AsValueMath: AsValue {} // Other way around to implement it for all without core types
