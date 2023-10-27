//! Generators for vectors.

use std::marker::PhantomData;

use crate::shrink::vec::VecShrink;
use crate::Gen;
use crate::SomeIter;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

/// Any vector filled with values from given generator
pub fn any<E, GE>(inner_gen: GE) -> RandVecGen<E, GE>
where
    E: Clone + 'static,
    GE: Gen<E>,
{
    RandVecGen::<E, GE> {
        element_phantom: PhantomData,
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
    element_phantom: PhantomData<E>,
    inner_gen: GE,
}

impl<E, GE> Gen<Vec<E>> for RandVecGen<E, GE>
where
    E: Clone + 'static,
    GE: Gen<E>,
{
    type Shrink = VecShrink<E>;
    fn examples(&self, seed: u64) -> SomeIter<Vec<E>> {
        Box::new(RandVecIter::<E> {
            rng: rand_chacha::ChaCha8Rng::seed_from_u64(seed),
            inner_it: self.inner_gen.examples(seed),
        })
    }

    fn shrinker(&self) -> Self::Shrink {
        crate::shrink::vec::default()
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
