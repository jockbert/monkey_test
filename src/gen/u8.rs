//! Generators for values of type u8.

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::shrink::NumDecrementShrink;

/// Any u8 value
pub fn any() -> RandU8Gen {
    RandU8Gen {}
}

/// Generator of random u8 values.
#[derive(Clone)]
pub struct RandU8Gen {}

impl crate::Gen<u8, NumDecrementShrink> for RandU8Gen {
    fn examples(&self, seed: u64) -> crate::SomeIter<u8> {
        Box::new(RandU8Iter {
            rng: rand_chacha::ChaCha8Rng::seed_from_u64(seed),
        })
    }

    fn shrinker(&self) -> NumDecrementShrink {
        NumDecrementShrink {}
    }
}

/// Iterator of random u8 values.
pub struct RandU8Iter {
    rng: ChaCha8Rng,
}

impl Iterator for RandU8Iter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let f = self.rng.gen::<u16>();
        let h = f as u8;
        Some(h)
    }
}
