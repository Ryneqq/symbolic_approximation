use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::view::ContinuousView;
use plotlib::style::LineStyle;
use std::fmt;
use itertools::Itertools;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
enum Var {
    X, Y, Z
}

impl fmt::Display for Var {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Var::*;

        let var = match self {
            X => "x",
            Y => "y",
            Z => "z",
        };

        write!(f, "{}", var)
    }
}

enum Par {
    Con(f64),
    Var(Var),
    Fun(Gen)
}

// impl Par {
//     pub fn random(genetic: Code) -> Self {
//         Fun(Gen::new(Fun::random()))
//     }
// }

enum Fun {
    Add(Vec<Par>),
    Mul(Vec<Par>),
    Pow(Vec<Par>),
    Log(Vec<Par>) // first element is log base, unless its not effective then use ln
    // Other more specialistic functions
}

// impl Fun {
//     pub fn random(genetic: Code) -> Self {

//     }
// }

pub struct Gen {
    fun: Fun,
    // Genetic Code
    // gen: Code
}

impl Gen {
    pub fn new() -> Self {
        use self::Par::*;
        use self::Fun::*;

        // genetic code
        // let code = Code::random();
        // Code should contain information about random function
        Self {
            // fun: Fun::random(code)
            fun: Add(vec![Con(1.), Con(1.)])
        }
    }
}

pub struct ParGen {
    con: f64,
    var: f64,
    fun: f64,
}

pub struct FunGen {
    add: f64,
    mul: f64,
    pow: f64,
    log: f64,
}

pub struct ActGen {

}

pub struct GenCode {
    par: ParGen,
    fun: FunGen,
    act: ActGen,
}

fn main() {
    use self::Fun::*;
    use self::Par::*;
    use self::Var::*;




    // let s1: Plot = Plot::new(org)
    //     .line_style(LineStyle::new().colour("#0000ff"));
    // let s2: Plot = Plot::new(integral)
    //     .line_style(LineStyle::new().colour("#ff0000"));
    // let s3: Plot = Plot::new(integral_ideal)
    //     .line_style(LineStyle::new().colour("#00ff00"));
    // let v = ContinuousView::new()
    //     .add(s1)
    //     .add(s2)
    //     .add(s3)
    //     .x_range(-15., 15.);

    // Page::single(&v).save("integrals.svg").unwrap();
}
