mod as_value;
mod value;
mod fun;
mod var;
mod outcome;

use self::{as_value::*, value::*, fun::*, var::*, outcome::*};

fn main() {
    use self::Var::*;
    use self::Value::*;

    // let result = Element::Fun(Fun::Sin(Elements::new(Element::Const(1.0), Element::Const(2.0) + Element::Const(2.0) * Element::Const(2.0))));
    // let result = Element::add((Const(1.0), Element::from(X) + Const(2.0) + Const(2.0) * Const(2.0)));
    // let result = Const(1.0) + Var(X) + Const(2.0) + Const(2.0) * Const(2.0);
    let result = Const(1.0) + Value::pow((X, Y)) + Const(2.0) + Const(2.0) * Const(2.0);
    // let result = result.pre_calc(Some(2.0), Some(2.0), None);
    let result = result.outcome().bind(X, 2.0).bind(Y, 3.0).calculate();

    // let fun = |x, y| {

    // };

    println!("Works {:?}", result);
}
