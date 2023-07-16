//! Generators for vectors.

use std::marker::PhantomData;

use crate::Gen;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

/// Any vector filled with values from given generator
pub fn any<A: 'static + Clone, GA: Gen<A>>(inner_gen: GA) -> RandVecGen<A, GA> {
    RandVecGen::<A, GA> {
        phantom: PhantomData,
        inner_gen,
    }
}

/// Generator for random vectors.
#[derive(Clone)]
pub struct RandVecGen<A, GA>
where
    A: 'static + Clone,
    GA: Gen<A>,
{
    /// just saying that we care about the type A.
    phantom: PhantomData<A>,
    inner_gen: GA,
}

impl<A: 'static + Clone, GA: Gen<A>> crate::Gen<Vec<A>> for RandVecGen<A, GA> {
    fn iter(&self, seed: u64) -> crate::SomeIter<Vec<A>> {
        Box::new(RandVecIter::<A> {
            rng: rand_chacha::ChaCha8Rng::seed_from_u64(seed),
            inner_it: self.inner_gen.iter(seed),
        })
    }
}

/// Iterator of random vectors.
pub struct RandVecIter<A> {
    rng: ChaCha8Rng,
    inner_it: crate::SomeIter<A>,
}

impl<A> Iterator for RandVecIter<A> {
    type Item = Vec<A>;

    fn next(&mut self) -> Option<Self::Item> {
        let f = self.rng.gen::<u16>();
        let length = f as u8;

        let mut res: Vec<A> = Vec::new();
        for _ in 0..length {
            res.push(self.inner_it.next().expect(""))
        }
        Some(res)
    }
}
