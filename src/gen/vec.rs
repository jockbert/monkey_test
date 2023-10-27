//! Generators for vectors.

use crate::shrink::vec::VecShrink;
use crate::Gen;
use crate::SomeIter;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

/// Any vector filled with values from given generator
pub fn any<G2>(inner_gen: G2) -> RandVecGen<G2>
where
    G2: Gen,
{
    RandVecGen::<G2> { inner_gen }
}

/// Generator for random vectors.
#[derive(Clone)]
pub struct RandVecGen<InnerGen>
where
    InnerGen: Gen,
{
    inner_gen: InnerGen,
}

impl<G2> Gen for RandVecGen<G2>
where
    G2: Gen + 'static,
{
    type Example = Vec<G2::Example>;
    type Shrink = VecShrink<G2::Example>;

    fn examples(&self, seed: u64) -> SomeIter<Self::Example> {
        Box::new(RandVecIter::<G2::Example> {
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
