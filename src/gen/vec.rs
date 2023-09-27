//! Generators for vectors.

use std::marker::PhantomData;

use crate::Gen;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

/// Any vector filled with values from given generator
pub fn any<E: 'static + Clone, GE: Gen<E>>(inner_gen: GE) -> RandVecGen<E, GE> {
    RandVecGen::<E, GE> {
        phantom: PhantomData,
        inner_gen,
    }
}

/// Generator for random vectors.
#[derive(Clone)]
pub struct RandVecGen<E, GE>
where
    E: 'static + Clone,
    GE: Gen<E>,
{
    /// just saying that we care about the type A.
    phantom: PhantomData<E>,
    inner_gen: GE,
}

impl<E: 'static + Clone, GE: Gen<E>> crate::Gen<Vec<E>> for RandVecGen<E, GE> {
    fn iter(&self, seed: u64) -> crate::SomeIter<Vec<E>> {
        Box::new(RandVecIter::<E> {
            rng: rand_chacha::ChaCha8Rng::seed_from_u64(seed),
            inner_it: self.inner_gen.iter(seed),
        })
    }
}

/// Iterator of random vectors.
pub struct RandVecIter<E> {
    rng: ChaCha8Rng,
    inner_it: crate::SomeIter<E>,
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
