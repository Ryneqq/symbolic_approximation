use crate::{Fun, Var, FunValues, Outcome, AsValue};

use std::collections::HashMap;
use std::ops::{Add, Mul, Sub, Div};

#[derive(Debug)]
pub enum Value {
    Fun(Fun),
    Var(Var),
    Const(f32),
}

impl Value {
    pub fn add(elements: impl Into<FunValues>) -> Self {
        Self::Fun(Fun::Add(elements.into()))
    }

    pub fn mul(elements: impl Into<FunValues>) -> Self {
        Self::Fun(Fun::Mul(elements.into()))
    }

    pub fn pow(elements: impl Into<FunValues>) -> Self {
        Self::Fun(Fun::Pow(elements.into()))
    }

    pub fn sin(elements: impl Into<FunValues>) -> Self {
        Self::Fun(Fun::Sin(elements.into()))
    }

    pub fn log(elements: impl Into<FunValues>) -> Self {
        Self::Fun(Fun::Log(elements.into()))
    }

    // TODO: should return Value
    pub fn calc(&self, vars: &HashMap<Var, f32>) -> f32 {
        match self {
            Value::Const(con) => *con,
            Value::Var(var) => *vars.get(&var).unwrap(),
            Value::Fun(fun) => fun.calc(&vars),
        }
    }

    // TODO: remove
    // pub fn pre_calc(&self, x: Option<f32>, y: Option<f32>, z: Option<f32>) -> Value {
    //     match self {
    //         Value::Const(con) => Value::Const(*con),
    //         Value::Fun(fun) => fun.pre_calc(x, y, z),
    //         Value::Var(Var::X) => {
    //             if x.is_some() {
    //                 Value::Const(x.unwrap())
    //             } else {
    //                 Value::x()
    //             }
    //         }
    //         Value::Var(Var::Y) => {
    //             if y.is_some() {
    //                 Value::Const(y.unwrap())
    //             } else {
    //                 Value::y()
    //             }
    //         }
    //         Value::Var(Var::Z) => {
    //             if z.is_some() {
    //                 Value::Const(z.unwrap())
    //             } else {
    //                 Value::z()
    //             }
    //         }
    //     }
    // }

    pub fn outcome(self) -> Outcome {
       self.into()
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, other: Self) -> Value {
        Value::add((self, other))
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, other: Self) -> Value {
        self + Value::Const(-1.0) * other
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, other: Self) -> Value {
        Value::mul((self, other))
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, other: Self) -> Value {
        self * Value::pow((other, Value::Const(-1.0)))
    }
}

impl From<Var> for Value {
    fn from(v: Var) -> Self {
        Value::Var(v)
    }
}

impl From<f32> for Value {
    fn from(f: f32) -> Self {
        Value::Const(f)
    }
}

impl<T: AsValue, O: AsValue> From<(T, O)> for FunValues {
    fn from((one, other): (T, O)) -> Self {
        Self::new(one.element(), other.element())
    }
}
