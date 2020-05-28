use crate::Par;
use variant_count::VariantCount;
use num_enum::IntoPrimitive;

#[derive(IntoPrimitive, VariantCount)]
#[repr(u8)]
pub enum Fun {
    Add,
    Mul,
    Pow,
    Log
}

pub struct Function {
    function: Fun,
    params: Vec<Par>
}

pub struct Genetics {
    entity: Function,
    code: Vec<f64>,
}


