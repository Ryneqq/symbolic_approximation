use crate::{ Value, Var };

use itertools::Itertools;
use std::collections::HashMap;
use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug)]
pub enum Fun {
    Add(FunValues),
    Mul(FunValues),
    Pow(FunValues),
    Sin(FunValues),
    Log(FunValues),
}

impl Fun {
    pub fn calc(&self, vars: &HashMap<Var, f32>) -> Value {
        use Value::Const;

        match self {
            Fun::Add(fun_values) => match fun_values.calc(vars) {
                (Const(one), Const(other)) => Value::Const(one + other),
                // TODO: Could add mechanism for switching elemnts to add
                // TODO2 dont remember an idea
                // (one, Const(other)) => Element::Const(one + other),
                // (Const(one), other) => Element::Const(one + other),
                (one, other) => Value::add((one, other)),
            },
            Fun::Mul(fun_values) => match fun_values.calc(vars) {
                (Const(one), Const(other)) => Value::Const(one * other),
                (one, other) => Value::mul((one, other)),
            },
            Fun::Pow(fun_values) => match fun_values.calc(vars) {
                (Const(one), Const(other)) => Value::Const(one.powf(other)),
                (one, other) => Value::pow((one, other)),
            },
            Fun::Sin(fun_values) => match fun_values.calc(vars) {
                (Const(one), Const(other)) => Value::Const(one * other.sin()),
                (one, other) => Value::sin((one, other)),
            },
            Fun::Log(fun_values) => match fun_values.calc(vars) {
                (Const(one), Const(other)) => Value::Const(one.log(other)),
                (one, other) => Value::sin((one, other)),
            },
        }
    }
}

/// ## Example
/// (x, y) = 2 + x * 2 ^ y * sin(x) + log(2, x) // proposition
/// |x, y| = 2 + x * 2 ^ y * sin(x) + log(2, x) // proposition
/// skipping whatever braces
impl FromStr for Fun {
    type Err = ();

    fn from_str(fun: &str) -> Result<Self, Self::Err> {
        let fun = fun.split(" ").join(""); // remove all spaces;
        let mut iter = fun.split("=");
        let (vars, equation) = (
            iter.next().expect("Missing Vars").trim(),
            iter.next().expect("Missing Equation").trim(),
        );
        let vars = vars[1..vars.len()].split(",").map(|var| var.trim());

        let extract_fun = |operator: &str| {}; // this is not the best way i have to match function to the sign
                                               // i can add parsing to the `Fun` enum

        // parse vars

        unimplemented!()
    }
}

#[derive(Debug)]
pub struct FunValues(Box<(Value, Value)>);

impl FunValues {
    pub fn new(one: Value, other: Value) -> Self {
        FunValues(Box::new((one, other)))
    }

    pub fn calc(&self, vars: &HashMap<Var, f32>) -> (Value, Value) {
        match self.0.deref() {
            (ref one, ref other) => (one.calc(vars), other.calc(vars)),
        }
    }
}
