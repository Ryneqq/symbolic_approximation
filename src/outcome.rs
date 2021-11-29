use crate::{Element, Var};
use std::collections::HashMap;

pub struct Outcome {
    element: Element,
    bindings: HashMap<Var, f32>
}

impl From<Element> for Outcome {
    fn from(element: Element) -> Self {
        Self { element, bindings: Default::default() }
    }
}

impl Outcome {
    pub fn bind(mut self, var: impl Into<Var>, value: f32) -> Self {
        self.bindings.insert(var.into(), value);

        self
    }

    pub fn calculate(&self) -> f32 {
        self.element.calc(&self.bindings)
    }
}