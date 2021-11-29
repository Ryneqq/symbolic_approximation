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

    pub fn calc(&self, vars: &HashMap<Var, f32>) -> Value {
        match self {
            Value::Const(con) => Value::Const(*con),
            Value::Var(var) => vars.get(&var).map(|con| Value::Const(*con)).unwrap_or(Value::Var(*var)),
            Value::Fun(fun) => fun.calc(&vars),
        }
    }

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
