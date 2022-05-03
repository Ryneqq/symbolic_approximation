use symbols::{Var, Value};

pub struct Phenotype {
    code: Value
}
// enum Bit {
//     Const(f64),
//     Var(Var),
//     Operator(Operator),
// }

// enum Operator {
//     Add,
//     Mul,
//     Pow,
//     Sin,
//     Log,
// }

fn main() {
    use self::Var::*;
    use self::Value::*;

    // let result = Const(1.0) + Value::pow((X, Y)) + Const(2.0) + Const(2.0) * Const(2.0);
    let result = Const(2.0) * Const(2.0) + Const(2.0);
    let result = result.outcome().bind(X, 2.0).bind(Y, 3.0).calculate();

    println!("Works {:?}", result);
}
