use rand::prelude::*;

struct Sequence<F> {
    cur: usize,
    max: usize,
    sum: f64,
    get_prb: F
}

impl<F> Sequence<F>
    where F: FnMut() -> f64
{
    pub fn new(len: usize, f: F) -> Self {
        Self {
            cur: 1,
            max: len,
            sum: 0f64,
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
            Some(1f64 - self.sum)
        } else {
            None
        }
    }
}

pub fn sequence(len: usize) -> impl Iterator<Item = f64> {
    Sequence::new(len, random(len))
}

fn random(mut len: usize) -> impl FnMut() -> f64 {
    len = len / 2;
    let mut prb = 1f64;
    let mut rng = rand::thread_rng();

    move || {
        let ret = rng.gen::<f64>() * (prb / len as f64);
        prb -= ret;
        if len > 1 {len -= 1};
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random() {
        assert_eq!(0f64, sequence(0).sum());
        assert_eq!(1f64, sequence(1).sum());
        assert_eq!(1f64, sequence(10).sum());
        assert_eq!(1f64, sequence(100).sum());

        assert_eq!(0, sequence(0).count());
        assert_eq!(1, sequence(1).count());
        assert_eq!(10, sequence(10).count());
        assert_eq!(100, sequence(100).count());
    }

    #[test]
    fn test_random1() {
        for val in sequence(5) {
            dbg!(val);
        }

        panic!("")
    }
}
