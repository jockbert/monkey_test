//! Generators for vectors.

use std::marker::PhantomData;

use crate::shrink::vec::VecShrink;
use crate::Gen;
use crate::Shrink;
use crate::SomeIter;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

/// Any vector filled with values from given generator
pub fn any<E, SE, GE: Gen<E, SE>>(inner_gen: GE) -> RandVecGen<E, GE, SE>
where
    E: Clone + 'static,
    SE: Shrink<E>,
{
    RandVecGen::<E, GE, SE> {
        element_phantom: PhantomData,
        shrinker_phantom: PhantomData,
        inner_gen,
    }
}

/// Generator for random vectors.
#[derive(Clone)]
pub struct RandVecGen<E, GE, SE>
where
    E: 'static + Clone,
    SE: Shrink<E>,
    GE: Gen<E, SE>,
{
    /// just saying that we care about the type A.
    element_phantom: PhantomData<E>,
    shrinker_phantom: PhantomData<SE>,
    inner_gen: GE,
}

impl<E, SE, GE> Gen<Vec<E>, VecShrink<E>> for RandVecGen<E, GE, SE>
where
    E: Clone + 'static,
    SE: Shrink<E>,
    GE: Gen<E, SE>,
{
    fn examples(&self, seed: u64) -> SomeIter<Vec<E>> {
        Box::new(RandVecIter::<E> {
            rng: rand_chacha::ChaCha8Rng::seed_from_u64(seed),
            inner_it: self.inner_gen.examples(seed),
        })
    }

    fn shrinker(&self) -> VecShrink<E> {
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
