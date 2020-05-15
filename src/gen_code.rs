use rand::prelude::*;

pub struct GenCode {
    par: ParGen,
    fun: FunGen,
    act: ActGen,
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
        let mut get_prb = random();

        let con = get_prb();
        let var = get_prb();
        let fun = 1f64 - con - var;

        Self::new(con, var, fun)
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

fn random() -> impl FnMut() -> f64 {
    let mul = 0.5f64;
    let mut prb = 1f64;
    let mut rng = rand::thread_rng();

    move || {
        let ret = rng.gen::<f64>() * mul * prb;
        prb -= ret;
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random() {
        let mut get_prb = random();
        let mut sum = 0f64;

        for _ in 0..100 {
            sum += get_prb();
        }

        assert!(sum < 1f64)
    }
}