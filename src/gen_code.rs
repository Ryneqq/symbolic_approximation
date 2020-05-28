use rand::prelude::*;

pub struct GenCode {
    par: ParGen,
    fun: FunGen,
    act: ActGen,
}

impl GenCode {
    pub fn new(par: ParGen, fun: FunGen, act: ActGen) -> Self {
        unimplemented!() // TODO
    }
}

pub struct ParGen {
    con: f64,
    var: f64,
    fun: f64,
}

impl ParGen {
    pub fn new(con: f64, var: f64, fun: f64) -> Self {
        Self { con, var, fun }
    }

    pub fn random() -> Self {
        // let mut gen = random_sequence(3);

        // let con = gen.next().unwrap();
        // let var = gen.next().unwrap();
        // let fun = gen.next().unwrap();

        // Self::new(con, var, fun)
        unimplemented!() // TODO
    }

}

pub struct FunGen {
    add: f64,
    mul: f64,
    pow: f64,
    log: f64,
}

pub struct ActGen {
    ele: i64, // Elemetns gathered at least two, more elements should be rare
    mutation: f64, // mutation rate
}

