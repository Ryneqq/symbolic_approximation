mod element;
mod fun;
mod var;
mod outcome;

use self::{element::{ElementTrait, *}, fun::*, var::*, outcome::*};


// use itertools::Itertools;
// use std::collections::HashMap;
// use std::ops::Deref;
// use std::ops::{Add, Mul, Sub, Div};
// use std::str::FromStr;

// #[derive(Debug)]
// pub enum Fun {
//     Add(Elements),
//     Mul(Elements),
//     Pow(Elements),
//     Sin(Elements),
//     Log(Elements),
// }

// impl Fun {
//     pub fn calc(&self, vars: &HashMap<Var, f32>) -> f32 {
//         match self {
//             Fun::Add(elements) => {
//                 let (one, other) = elements.calc(vars);
//                 one + other
//             }
//             Fun::Mul(elements) => {
//                 let (one, other) = elements.calc(vars);
//                 one * other
//             }
//             Fun::Pow(elements) => {
//                 let (one, other) = elements.calc(vars);
//                 one.powf(other)
//             }
//             Fun::Sin(elements) => {
//                 let (one, other) = elements.calc(vars);
//                 one * other.sin()
//             }
//             Fun::Log(elements) => {
//                 let (one, other) = elements.calc(vars);
//                 one.log(other) // TODO: Log do not accept some values
//             }
//         }
//     }

//     pub fn pre_calc(&self, x: Option<f32>, y: Option<f32>, z: Option<f32>) -> Element {
//         use Element::Const;

//         match self {
//             Fun::Add(elements) => match elements.pre_calc(x, y, z) {
//                 (Const(one), Const(other)) => Element::Const(one + other),
//                 // TODO: Could add mechanism for switching elemnts to add
//                 // (one, Const(other)) => Element::Const(one + other),
//                 // (Const(one), other) => Element::Const(one + other),
//                 (one, other) => Element::add((one, other)),
//             },
//             Fun::Mul(elements) => match elements.pre_calc(x, y, z) {
//                 (Const(one), Const(other)) => Element::Const(one * other),
//                 (one, other) => Element::mul((one, other)),
//             },
//             Fun::Pow(elements) => match elements.pre_calc(x, y, z) {
//                 (Const(one), Const(other)) => Element::Const(one.powf(other)),
//                 (one, other) => Element::pow((one, other)),
//             },
//             Fun::Sin(elements) => match elements.pre_calc(x, y, z) {
//                 (Const(one), Const(other)) => Element::Const(one * other.sin()),
//                 (one, other) => Element::sin((one, other)),
//             },
//             Fun::Log(elements) => match elements.pre_calc(x, y, z) {
//                 (Const(one), Const(other)) => Element::Const(one.log(other)),
//                 (one, other) => Element::sin((one, other)),
//             },
//         }
//     }
// }

// /// ## Example
// /// (x, y) = 2 + x * 2 ^ y * sin(x) + log(2, x) // proposition
// /// |x, y| = 2 + x * 2 ^ y * sin(x) + log(2, x) // proposition
// /// skipping whatever braces
// impl FromStr for Fun {
//     type Err = ();

//     fn from_str(fun: &str) -> Result<Self, Self::Err> {
//         let fun = fun.split(" ").join(""); // remove all spaces;
//         let mut iter = fun.split("=");
//         let (vars, equation) = (
//             iter.next().expect("Missing Vars").trim(),
//             iter.next().expect("Missing Equation").trim(),
//         );
//         let vars = vars[1..vars.len()].split(",").map(|var| var.trim());

//         let extract_fun = |operator: &str| {}; // this is not the best way i have to match function to the sign
//                                                // i can add parsing to the `Fun` enum

//         // parse vars

//         unimplemented!()
//     }
// }

// #[derive(Debug)]
// pub struct Elements(Box<(Element, Element)>);

// impl Elements {
//     pub fn new(one: Element, other: Element) -> Self {
//         Elements(Box::new((one, other)))
//     }

//     pub fn calc(&self, vars: &HashMap<Var, f32>) -> (f32, f32) {
//         match self.0.deref() {
//             (ref one, ref other) => (one.calc(vars), other.calc(vars)),
//         }
//     }

//     pub fn pre_calc(&self, x: Option<f32>, y: Option<f32>, z: Option<f32>) -> (Element, Element) {
//         match self.0.deref() {
//             (ref one, ref other) => (one.pre_calc(x, y, z), other.pre_calc(x, y, z)),
//         }
//     }
// }

// impl<T: ElementTrait, O: ElementTrait> From<(T, O)> for Elements {
//     fn from((one, other): (T, O)) -> Self {
//         Self::new(one.element(), other.element())
//     }
// }


// #[derive(Debug)]
// pub enum Element {
//     Fun(Fun),
//     Var(Var),
//     Const(f32),
// }

// impl Element {
//     pub fn x() -> Self {
//         Self::Var(Var::X)
//     }

//     pub fn y() -> Self {
//         Self::Var(Var::Y)
//     }

//     pub fn z() -> Self {
//         Self::Var(Var::Z)
//     }

//     pub fn add(elements: impl Into<Elements>) -> Self {
//         Self::Fun(Fun::Add(elements.into()))
//     }

//     pub fn mul(elements: impl Into<Elements>) -> Self {
//         Self::Fun(Fun::Mul(elements.into()))
//     }

//     pub fn pow(elements: impl Into<Elements>) -> Self {
//         Self::Fun(Fun::Pow(elements.into()))
//     }

//     pub fn sin(elements: impl Into<Elements>) -> Self {
//         Self::Fun(Fun::Sin(elements.into()))
//     }

//     pub fn log(elements: impl Into<Elements>) -> Self {
//         Self::Fun(Fun::Log(elements.into()))
//     }

//     pub fn calc(&self, vars: &HashMap<Var, f32>) -> f32 {
//         match self {
//             Element::Const(con) => *con,
//             Element::Var(var) => *vars.get(&var).unwrap(),
//             Element::Fun(fun) => fun.calc(&vars),
//         }
//     }

//     pub fn pre_calc(&self, x: Option<f32>, y: Option<f32>, z: Option<f32>) -> Element {
//         match self {
//             Element::Const(con) => Element::Const(*con),
//             Element::Fun(fun) => fun.pre_calc(x, y, z),
//             Element::Var(Var::X) => {
//                 if x.is_some() {
//                     Element::Const(x.unwrap())
//                 } else {
//                     Element::x()
//                 }
//             }
//             Element::Var(Var::Y) => {
//                 if y.is_some() {
//                     Element::Const(y.unwrap())
//                 } else {
//                     Element::y()
//                 }
//             }
//             Element::Var(Var::Z) => {
//                 if z.is_some() {
//                     Element::Const(z.unwrap())
//                 } else {
//                     Element::z()
//                 }
//             }
//         }
//     }

//     pub fn outcome(self) -> Outcome {
//        self.into()
//     }
//     // TODO: Precalc could be implemented simple if variable is missing you can return other function
//     // Instead of f32 you can return Element which would be Element::Const(...) for calculated values
// }

// impl Add for Element {
//     type Output = Element;

//     fn add(self, other: Self) -> Element {
//         Element::add((self, other))
//     }
// }

// impl Sub for Element {
//     type Output = Element;

//     fn sub(self, other: Self) -> Element {
//         self + Element::Const(-1.0) * other
//     }
// }

// impl Mul for Element {
//     type Output = Element;

//     fn mul(self, other: Self) -> Element {
//         Element::mul((self, other))
//     }
// }

// impl Div for Element {
//     type Output = Element;

//     fn div(self, other: Self) -> Element {
//         self * Element::pow((other, Element::Const(-1.0)))
//     }
// }

// impl From<Var> for Element {
//     fn from(v: Var) -> Self {
//         Element::Var(v)
//     }
// }

// impl From<f32> for Element {
//     fn from(f: f32) -> Self {
//         Element::Const(f)
//     }
// }

// pub trait ElementTrait {
//     fn element(self) -> Element;
// }

// impl ElementTrait for Element {
//     fn element(self) -> Element {
//         self
//     }
// }


// #[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
// pub enum Var {
//     X,
//     Y,
//     Z,
// }

// impl ElementTrait for Var {
//     fn element(self) -> Element {
//         self.into()
//     }
// }

// impl ElementTrait for f32 {
//     fn element(self) -> Element {
//         self.into()
//     }
// }

// pub struct Outcome {
//     element: Element,
//     bindings: HashMap<Var, f32>
// }

// impl From<Element> for Outcome {
//     fn from(element: Element) -> Self {
//         Self { element, bindings: Default::default() }
//     }
// }

// impl Outcome {
//     fn bind(mut self, var: impl Into<Var>, value: f32) -> Self {
//         self.bindings.insert(var.into(), value);

//         self
//     }

//     fn calculate(&self) -> f32 {
//         self.element.calc(&self.bindings)
//     }
// }

fn main() {
    use self::Var::*;
    use self::Element::*;

    // let result = Element::Fun(Fun::Sin(Elements::new(Element::Const(1.0), Element::Const(2.0) + Element::Const(2.0) * Element::Const(2.0))));
    // let result = Element::add((Const(1.0), Element::from(X) + Const(2.0) + Const(2.0) * Const(2.0)));
    // let result = Const(1.0) + Var(X) + Const(2.0) + Const(2.0) * Const(2.0);
    let result = Const(1.0) + Element::pow((X, Y)) + Const(2.0) + Const(2.0) * Const(2.0);
    // let result = result.pre_calc(Some(2.0), Some(2.0), None);
    let result = result.outcome().bind(X, 2.0).bind(Y, 3.0).calculate();

    // let fun = |x, y| {

    // };

    println!("Works {:?}", result);
}
