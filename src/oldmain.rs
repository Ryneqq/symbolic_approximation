use std::ops::{Add, Mul};
use std::str::FromStr;
use std::ops::Deref;
use itertools::Itertools;

pub enum Fun {
    Add(Elements),
    Mul(Elements),
    Pow(Elements),
    Sin(Elements),
    Log(Elements),
}

impl Fun {
    pub fn calc(&self, x: Option<f32>, y: Option<f32>, z: Option<f32>) -> f32 {
        match self {
            Fun::Add(elements) => {
                let (one, other) = elements.calc(x, y, z);
                one + other
            }
            Fun::Mul(elements) => {
                let (one, other) = elements.calc(x, y, z);
                one * other
            }
            Fun::Pow(elements) => {
                let (one, other) = elements.calc(x, y, z);
                one.powf(other)
            }
            Fun::Sin(elements) => {
                let (one, other) = elements.calc(x, y, z);
                one * other.sin()
            }
            Fun::Log(elements) => {
                let (one, other) = elements.calc(x, y, z);
                one.log(other) // TODO: Log do not accept some values
            }
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

        let extract_fun = |operator: &str| {  }; // this is not the best way i have to match function to the sign
        // i can add parsing to the `Fun` enum



       // parse vars

       unimplemented!()

    }
}

pub struct Elements(Box<(Element, Element)>);

impl Elements {
    pub fn calc(&self, x: Option<f32>, y: Option<f32>, z: Option<f32>) -> (f32, f32) {
        match self.0.deref() {
            (ref one, ref other) => (one.calc(x, y, z), other.calc(x, y, z)),
        }
    }
}

pub enum Element {
    Fun(Fun),
    Var(Var),
    Const(f32),
}

impl Element {
   pub fn calc(&self, x: Option<f32>, y: Option<f32>, z: Option<f32>) -> f32 {
        match self {
            Element::Const(con) => *con,
            Element::Var(Var::X) if x.is_some() => x.unwrap(),
            Element::Var(Var::Y) if y.is_some() => y.unwrap(),
            Element::Var(Var::Z) if z.is_some() => z.unwrap(),
            Element::Fun(fun) => fun.calc(x, y, z),
            _ => panic!("Variable not defined: required variable is missing")
        }
    }
}

// from f32
// from Var
// from Fun

impl Add for Element {
    type Output = Element;

    fn add(self, other: Self) -> Element {
        Element::Fun(Fun::Add(Elements(Box::new((self, other)))))
    }
}

impl Mul for Element {
    type Output = Element;

    fn mul(self, other: Self) -> Element {
        Element::Fun(Fun::Mul(Elements(Box::new((self, other)))))
    }
}

pub enum Var {
    X,
    Y,
    Z,
}

fn main() {
    let result = Element::Const(2.0) + Element::Const(2.0) * Element::Const(2.0);
    let result = result.calc(Some(5.0), None, None);

    println!("Works {}", result);
}


// use plotlib::page::Page;
// use plotlib::repr::Plot;
// use plotlib::view::ContinuousView;
// use plotlib::style::LineStyle;
// use itertools::Itertools;
// mod var;
// mod par;
// mod fun;
// mod gen;
// mod gen_code;
// mod sequence;

// pub use var::Var;
// pub use par::Par;
// pub use fun::Fun;
// pub use gen::Gen;
// pub use gen_code::GenCode;

// fn main() {
//     use self::Fun::*;
//     use self::Par::*;
//     use self::Var::*;




//     // let s1: Plot = Plot::new(org)
//     //     .line_style(LineStyle::new().colour("#0000ff"));
//     // let s2: Plot = Plot::new(integral)
//     //     .line_style(LineStyle::new().colour("#ff0000"));
//     // let s3: Plot = Plot::new(integral_ideal)
//     //     .line_style(LineStyle::new().colour("#00ff00"));
//     // let v = ContinuousView::new()
//     //     .add(s1)
//     //     .add(s2)
//     //     .add(s3)
//     //     .x_range(-15., 15.);

//     // Page::single(&v).save("integrals.svg").unwrap();
// }