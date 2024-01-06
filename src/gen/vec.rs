//! Generators for vectors.

use crate::BoxGen;
use crate::BoxIter;
use crate::BoxShrink;
use crate::Gen;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

/// Any vector filled with values from given generator
pub fn any<E: Clone + 'static>(inner_gen: BoxGen<E>) -> BoxGen<Vec<E>> {
    Box::new(RandVecGen::<E> { inner_gen })
}

/// Generator for random vectors.
#[derive(Clone)]
pub struct RandVecGen<E> {
    inner_gen: BoxGen<E>,
}

impl<E: Clone + 'static> Gen<Vec<E>> for RandVecGen<E> {
    fn examples(&self, seed: u64) -> BoxIter<Vec<E>> {
        Box::new(RandVecIter::<E> {
            rng: rand_chacha::ChaCha8Rng::seed_from_u64(seed),
            inner_it: self.inner_gen.examples(seed),
        })
    }

    fn shrinker(&self) -> BoxShrink<Vec<E>> {
        Box::new(crate::shrink::vec::default())
    }
}

/// Iterator of random vectors.
pub struct RandVecIter<E> {
    rng: ChaCha8Rng,
    inner_it: crate::BoxIter<E>,
}

impl<E> Iterator for RandVecIter<E> {
    type Item = Vec<E>;

    fn next(&mut self) -> Option<Self::Item> {
        let f = self.rng.gen::<u16>();
        let length = f as u8;

        let mut res: Vec<E> = Vec::new();
        for _ in 0..length {
            res.push(self.inner_it.next().expect(""))
        }
        Some(res)
    }
}
