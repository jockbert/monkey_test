//! Generators for vectors.

use crate::BoxGen;
use crate::BoxIter;
use crate::BoxShrink;
use crate::Gen;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

/// Any vector filled with values from given element generator
pub fn any<E: Clone + 'static>(element_gen: BoxGen<E>) -> BoxGen<Vec<E>> {
    Box::new(RandVecGen::<E> { element_gen })
}

/// Generator for random vectors.
#[derive(Clone)]
struct RandVecGen<E> {
    element_gen: BoxGen<E>,
}

impl<E: Clone + 'static> Gen<Vec<E>> for RandVecGen<E> {
    fn examples(&self, seed: u64) -> BoxIter<Vec<E>> {
        Box::new(RandVecIter::<E> {
            rng: rand_chacha::ChaCha8Rng::seed_from_u64(seed),
            element_it: self.element_gen.examples(seed),
        })
    }

    fn shrinker(&self) -> BoxShrink<Vec<E>> {
        crate::shrink::vec::default()
    }
}

/// Iterator of random vectors.
struct RandVecIter<E> {
    rng: ChaCha8Rng,
    element_it: crate::BoxIter<E>,
}

impl<E> Iterator for RandVecIter<E> {
    type Item = Vec<E>;

    fn next(&mut self) -> Option<Self::Item> {
        let f = self.rng.gen::<u16>();
        let length = f as u8;

        let mut res: Vec<E> = Vec::new();
        for _ in 0..length {
            res.push(self.element_it.next().expect(""))
        }
        Some(res)
    }
}
