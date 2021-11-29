use crate::{Fun, Var, Elements, Outcome};

use std::collections::HashMap;
use std::ops::{Add, Mul, Sub, Div};

#[derive(Debug)]
pub enum Element { // TODO: rename to Value
    Fun(Fun),
    Var(Var),
    Const(f32),
}

impl Element {
    pub fn x() -> Self {
        Self::Var(Var::X)
    }

    pub fn y() -> Self {
        Self::Var(Var::Y)
    }

    pub fn z() -> Self {
        Self::Var(Var::Z)
    }

    pub fn add(elements: impl Into<Elements>) -> Self {
        Self::Fun(Fun::Add(elements.into()))
    }

    pub fn mul(elements: impl Into<Elements>) -> Self {
        Self::Fun(Fun::Mul(elements.into()))
    }

    pub fn pow(elements: impl Into<Elements>) -> Self {
        Self::Fun(Fun::Pow(elements.into()))
    }

    pub fn sin(elements: impl Into<Elements>) -> Self {
        Self::Fun(Fun::Sin(elements.into()))
    }

    pub fn log(elements: impl Into<Elements>) -> Self {
        Self::Fun(Fun::Log(elements.into()))
    }

    // TODO: should return Element
    pub fn calc(&self, vars: &HashMap<Var, f32>) -> f32 {
        match self {
            Element::Const(con) => *con,
            Element::Var(var) => *vars.get(&var).unwrap(),
            Element::Fun(fun) => fun.calc(&vars),
        }
    }

    pub fn pre_calc(&self, x: Option<f32>, y: Option<f32>, z: Option<f32>) -> Element {
        match self {
            Element::Const(con) => Element::Const(*con),
            Element::Fun(fun) => fun.pre_calc(x, y, z),
            Element::Var(Var::X) => {
                if x.is_some() {
                    Element::Const(x.unwrap())
                } else {
                    Element::x()
                }
            }
            Element::Var(Var::Y) => {
                if y.is_some() {
                    Element::Const(y.unwrap())
                } else {
                    Element::y()
                }
            }
            Element::Var(Var::Z) => {
                if z.is_some() {
                    Element::Const(z.unwrap())
                } else {
                    Element::z()
                }
            }
        }
    }

    pub fn outcome(self) -> Outcome {
       self.into()
    }
}

impl Add for Element {
    type Output = Element;

    fn add(self, other: Self) -> Element {
        Element::add((self, other))
    }
}

impl Sub for Element {
    type Output = Element;

    fn sub(self, other: Self) -> Element {
        self + Element::Const(-1.0) * other
    }
}

impl Mul for Element {
    type Output = Element;

    fn mul(self, other: Self) -> Element {
        Element::mul((self, other))
    }
}

impl Div for Element {
    type Output = Element;

    fn div(self, other: Self) -> Element {
        self * Element::pow((other, Element::Const(-1.0)))
    }
}

impl From<Var> for Element {
    fn from(v: Var) -> Self {
        Element::Var(v)
    }
}

impl From<f32> for Element {
    fn from(f: f32) -> Self {
        Element::Const(f)
    }
}

impl<T: ElementTrait, O: ElementTrait> From<(T, O)> for Elements {
    fn from((one, other): (T, O)) -> Self {
        Self::new(one.element(), other.element())
    }
}

// TODO: Call it AsValue and move to new module
pub trait ElementTrait {
    fn element(self) -> Element;
}

impl ElementTrait for Element {
    fn element(self) -> Element {
        self
    }
}

impl ElementTrait for f32 {
    fn element(self) -> Element {
        self.into()
    }
}

impl ElementTrait for Var {
    fn element(self) -> Element {
        self.into()
    }
}


