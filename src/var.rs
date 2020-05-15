use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum Var {
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
