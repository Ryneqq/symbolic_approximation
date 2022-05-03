use itertools::Itertools;
use std::iter::DoubleEndedIterator;
use std::ops::Range;

const SKIP: u128 = 8 << 125;

pub struct PrimeStore {
    primes: Vec<u128>
}

impl PrimeStore {
    pub fn new(n: impl Into<u128>) -> Self {
        let n = n.into();
        let batch = (std::mem::size_of::<u128>() * 8usize) as u128;
        let n = n / 2 / batch;

        let mut primes = (0..n).map(|_|  u128::max_value()).collect::<Vec<u128>>();

        Self { primes }
    }

    pub fn idx(&self, n: impl Into<u128>) -> (usize, usize) {
        // It has to estimates where should number be put and then iterate over skips until one is lesser than the number
        unimplemented!() // TODO
        // probably some newton? as for zero places search
    }

    pub fn number(&self, (vec_idx, postion): (usize, usize)) -> u128 {
        // its easier in theory, you have to find last skip and sum it with distance to it

        unimplemented!() // TODO
    }

    pub fn calculate(mut self) -> Self {
        let batch = (std::mem::size_of::<u128>() * 8usize) as u128;
        let n = self.primes.len() as u128 * 2 * batch;

        let mut step = 3;
        let mut skip = 0;

        while step + skip < n {
            for i in (skip..n).step_by(step).skip(1) {
                let idx = i / batch;
                let bit_place = i % batch;
                let not_prime_mask = 1 << bit_place;

                self.primes[i] ^= not_prime_mask;
            }

            if let Some((next_idx, _)) = primes.iter()
                .enumerate()
                .skip(skip + 1)
                .find(|(_idx, prime)| **prime) {
                    skip = next_idx;
                    step = next_idx * 2 + 3;
            } else {
                break
            }
        }

        Self{ primes }
    }
}


#[derive(Debug)]
struct Primes {
    primes: Vec<bool> // TODO: Vec<u16>
}

// When Vec<uN> you can store gaps as a number
// When uN startst with 3 bits set as one it means it is a gap of primes
// With that in mind you have N - 3 * N - 1 bits to safe a number so for u8 bit int
// you can skip 2^5 * 7
// ! Unfortunately, we will not be able to easily use ind to determine what prime we have
// BUT maybe we could do the Btree

// ! Unless i count all the existed skips till that point then to reverse number from index i have to find last skips * 2 + 3 + len_from_skips * 2 + 3
// I have to have some proof how big gap can be between 2 primes
// On the other hand how to convert prime into index

// I could in theory do have how many numbers we had til the time of skip
// 3 5 20 23
//     ^ skipped it will be marked as 111xn where x is whatever bit and n means how many bits we have for saving a number
// so bigger is better in that case so Vec<u128> would make the most sense

impl Primes {
    fn new(n: usize) -> Self {
        let n = n / 2;
        let mut step = 3;
        let mut skip = 0;
        let mut primes = (0..n).map(|_| true).collect::<Vec<bool>>();

        while step + skip < n {
            for i in (skip..n).step_by(step).skip(1) {
                primes[i] = false;
            }

            if let Some((next_idx, _)) = primes.iter()
                .enumerate()
                .skip(skip + 1)
                .find(|(_idx, prime)| **prime) {
                    skip = next_idx;
                    step = next_idx * 2 + 3;
            } else {
                break
            }
        }

        Self{ primes }
    }

    fn into_number(idx: usize) -> usize {
        idx * 2 + 3
    }

    fn prime(idx: usize) -> f64 {
        Self::into_number(idx) as f64
    }

    fn idx(prime: f64) -> usize {
        ((prime - 3f64) / 2f64) as usize
    }

    fn iter_primes<'a>(&'a self) -> impl DoubleEndedIterator<Item = usize> + 'a {
        self.primes
            .iter()
            .enumerate()
            .filter_map(|( i, p )| if *p { Some(Self::into_number(i)) } else { None })
    }


    // fn into_inner(self) -> Vec<bool> {
    //     self.primes
    // }

    // fn last(self, n: usize) -> Vec<usize> {
    //     self.primes
    //         .into_iter()
    //         .enumerate()
    //         .rev()
    //         .filter(|(_, prim)| *prim)
    //         .map(|(idx, _)| idx * 2 + 3)
    //         .take(n)
    //         .collect()
    // }

    // // TODO: instead create `next_prime()` & `previous_prime`
    // fn position(&self, x: f64) -> usize {
    //     self.primes.iter()
    //         .enumerate()
    //         .position(|(p, _)| prime(p) as f64 >= x)
    //         .expect("No such prime range")
    // }

    fn next_prime(&self, x: f64) -> f64 {
        let idx = Self::idx(x) + 1;

        self.primes.iter()
            .enumerate()
            .skip(idx)
            .filter(|(_, prime)| **prime)
            .next()
            .map(|(idx, _)| Self::prime(idx))
            .expect(&format!("Not found for x: `{}`", x))
    }

    // fn next_prime_idx(&self, x: f64) -> Option<usize> {
    //     let idx = Self::idx(x) + 1;

    //     self.primes.iter()
    //         .enumerate()
    //         .skip(idx)
    //         .filter(|(_, prime)| **prime)
    //         .next()
    //         .map(|(idx, _)| idx)
    // }

    fn previous_prime(&self, x: f64) -> f64 {
        let idx = Self::idx(x) + 1;

        self.primes.iter()
            .enumerate()
            .take(idx)
            .rev()
            .filter(|(_, prime)| **prime)
            .next()
            .map(|(idx, _)| Self::prime(idx))
            .unwrap()
    }

    // fn sign_fun(&self, x: f64) -> f64 {
    //     let idx = Self::idx(x) + 1;
    //     let primes = self.primes.iter()
    //         .take(idx)
    //         .filter(|p| **p)
    //         .count();

    //     if primes % 2 == 0 {
    //         -1f64
    //     } else {
    //         1f64
    //     }

    // }

    // fn into_fun<'a>(&'a self) -> impl FnMut(f64) -> f64 + 'a {
    //     let mut start = 3f64;
    //     let mut end = 5f64;
    //     let mut sign = 1f64;

    //     move |x| {
    //         if x < start || x >= end {
    //             start = self.previous_prime(x);
    //             end = self.next_prime(x);
    //             sign = self.sign_fun(x)
    //         }

    //         let mid = (start + end) / 2f64;
    //         let y = end - mid;
    //         let a = y / ((mid - start) * (mid - end));

    //         // a * (x - start) * (x - end) * sign
    //         (a * (x - start) * (x - end) * sign).powf(3f64) * 20f64
    //         // (a * (x - start) * (x - end)).powf(2f64) * sign * 1000f64
    //         // (a * (x - start) * (x - end)).powf(1f64) * sign * 100000f64
    //     }
    // }

    // fn iter_fun<'a>(&'a self, mut range: Range<usize>, precision: f64) -> impl Iterator<Item = (f64, f64)> + 'a {
    //     let mut fun = self.into_fun();
    //     range.end = (range.end as f64 * precision) as usize;

    //     range.map(move |x| {
    //         let x = x as f64 / precision;
    //         let y = fun(x);

    //         (x, y)
    //     })
    // }
}