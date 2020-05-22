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
        let mut get_prb = random(3);

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

fn random(ele: usize) -> impl FnMut() -> f64 {
    let mut prb = 1f64;
    let mul = prb / (ele as f64 - 1f64);
    let mut rng = rand::thread_rng();

    move || {
        let ret = rng.gen::<f64>() * mul * prb;
        prb -= ret;
        ret
    }
}



// impl<I, F> Iterator for Update<I, F>
// where
//     I: Iterator,
//     F: FnMut(&mut I::Item),
// {
//     type Item = I::Item;

//     fn next(&mut self) -> Option<Self::Item> {
//         if let Some(mut v) = self.iter.next() {
//             (self.f)(&mut v);
//             Some(v)
//         } else {
//             None
//         }
//     }

struct Sequence<F> {
    cur: usize,
    max: usize,
    sum: f64,
    get_prb: F
}

impl<F> Sequence<F>
    where F: FnMut() -> f64
{
    pub fn new(ele: usize, f: F) -> Self {
        Self {
            cur: 0,
            max: ele,
            sum: 0.0,
            get_prb: f
        }
    }
}

impl<F> Iterator for Sequence<F>
    where
        F: FnMut() -> f64,
{
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur < self.max {
            let nxt = (self.get_prb)();
            self.sum += nxt;
            self.cur += 1;
            Some(nxt)
        } else if self.cur == self.max {
            self.cur += 1;
            Some(1.0 - self.sum)
        } else {
            None
        }
    }
}

pub fn seq(ele: usize) -> impl Iterator<Item = f64> {
    Sequence::new(ele, random(ele))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random() {
        println!("HEEERREEEA");
        let ele = 3;
        let mut get_prb = random(3);
        let mut sum = 0f64;
        dbg!("here");
        let mut seq = seq(ele);
        dbg!(seq.next());

        for num in seq {
            sum += dbg!(num);
        }
        dbg!(sum);
        panic!();

        // for _ in 0..ele {
        //     sum += dbg!(get_prb());
        // }

        // println!("sum: {}, last: {}", sum, 1f64 - sum);
        // assert!(sum > 1f64)
    }
}
