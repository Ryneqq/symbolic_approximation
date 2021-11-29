use crate::{Value, Var};

pub trait AsValue {
    fn element(self) -> Value;
}

impl AsValue for Value {
    fn element(self) -> Value {
        self
    }
}

impl AsValue for f32 {
    fn element(self) -> Value {
        self.into()
    }
}

impl AsValue for Var {
    fn element(self) -> Value {
        self.into()
    }
}
