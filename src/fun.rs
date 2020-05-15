use crate::Par;

pub enum Fun {
    Add(Vec<Par>),
    Mul(Vec<Par>),
    Pow(Vec<Par>),
    Log(Vec<Par>) // first element is log base, unless its not effective then use ln
    // Other more specialistic functions
}
