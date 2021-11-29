use crate::{ Element, ElementTrait, Var };

use itertools::Itertools;
use std::collections::HashMap;
use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug)]
pub enum Fun {
    Add(Elements),
    Mul(Elements),
    Pow(Elements),
    Sin(Elements),
    Log(Elements),
}

impl Fun {
    pub fn calc(&self, vars: &HashMap<Var, f32>) -> f32 {
        match self {
            Fun::Add(elements) => {
                let (one, other) = elements.calc(vars);
                one + other
            }
            Fun::Mul(elements) => {
                let (one, other) = elements.calc(vars);
                one * other
            }
            Fun::Pow(elements) => {
                let (one, other) = elements.calc(vars);
                one.powf(other)
            }
            Fun::Sin(elements) => {
                let (one, other) = elements.calc(vars);
                one * other.sin()
            }
            Fun::Log(elements) => {
                let (one, other) = elements.calc(vars);
                one.log(other) // TODO: Log do not accept some values
            }
        }
    }

    pub fn pre_calc(&self, x: Option<f32>, y: Option<f32>, z: Option<f32>) -> Element {
        use Element::Const;

        match self {
            Fun::Add(elements) => match elements.pre_calc(x, y, z) {
                (Const(one), Const(other)) => Element::Const(one + other),
                // TODO: Could add mechanism for switching elemnts to add
                // (one, Const(other)) => Element::Const(one + other),
                // (Const(one), other) => Element::Const(one + other),
                (one, other) => Element::add((one, other)),
            },
            Fun::Mul(elements) => match elements.pre_calc(x, y, z) {
                (Const(one), Const(other)) => Element::Const(one * other),
                (one, other) => Element::mul((one, other)),
            },
            Fun::Pow(elements) => match elements.pre_calc(x, y, z) {
                (Const(one), Const(other)) => Element::Const(one.powf(other)),
                (one, other) => Element::pow((one, other)),
            },
            Fun::Sin(elements) => match elements.pre_calc(x, y, z) {
                (Const(one), Const(other)) => Element::Const(one * other.sin()),
                (one, other) => Element::sin((one, other)),
            },
            Fun::Log(elements) => match elements.pre_calc(x, y, z) {
                (Const(one), Const(other)) => Element::Const(one.log(other)),
                (one, other) => Element::sin((one, other)),
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
pub struct Elements(Box<(Element, Element)>); // TODO rename to FunValues

impl Elements {
    pub fn new(one: Element, other: Element) -> Self {
        Elements(Box::new((one, other)))
    }

    pub fn calc(&self, vars: &HashMap<Var, f32>) -> (f32, f32) {
        match self.0.deref() {
            (ref one, ref other) => (one.calc(vars), other.calc(vars)),
        }
    }

    pub fn pre_calc(&self, x: Option<f32>, y: Option<f32>, z: Option<f32>) -> (Element, Element) {
        match self.0.deref() {
            (ref one, ref other) => (one.pre_calc(x, y, z), other.pre_calc(x, y, z)),
        }
    }
}
