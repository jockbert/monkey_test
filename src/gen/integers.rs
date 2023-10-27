//! Generic generators for integer type values.

use std::ops::Bound;
use std::ops::RangeBounds;

use min_max_traits::{Max, Min};
use num_traits::Num;
use rand::distributions::uniform::SampleUniform;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use crate::{shrink::NumShrink, Gen};

use super::OtherShrinkGen;
use super::{chain::ChainGen, fixed::SequenceGen};

pub(crate) type IntGen<E> =
    ChainGen<OtherShrinkGen<SequenceGen<E>, NumShrink>, UxGen<E>>;

/// Uniformly distributed range of values.
pub fn ranged<E, B>(bounds: B) -> IntGen<E>
where
    E: Num
        + Min
        + Max
        + SampleUniform
        + Copy
        + Clone
        + 'static
        + std::fmt::Debug,
    B: RangeBounds<E>,
{
    let min: E = match bounds.start_bound() {
        Bound::Included(x) => *x,
        Bound::Excluded(x) => *x + E::one(),
        Bound::Unbounded => E::MIN,
    };

    let max: E = match bounds.end_bound() {
        Bound::Included(x) => *x,
        Bound::Excluded(x) => *x - E::one(),
        Bound::Unbounded => E::MAX,
    };

    crate::gen::fixed::sequence(&[min, max])
        .with_shrinker(crate::shrink::number())
        .chain(&UxGen { min, max })
}

/// Generator of random usize values.
#[derive(Clone)]
pub struct UxGen<E> {
    min: E,
    max: E,
}

impl<E> Gen for UxGen<E>
where
    E: Num + SampleUniform + Copy + Clone + 'static,
{
    type Example = E;
    type Shrink = NumShrink;

    fn examples(&self, seed: u64) -> crate::SomeIter<E> {
        Box::new(UxIter::<E> {
            min: self.min,
            max: self.max,
            rng: rand_chacha::ChaCha8Rng::seed_from_u64(seed),
        })
    }

    fn shrinker(&self) -> Self::Shrink {
        NumShrink {}
    }
}

/// Iterator of random integer values.
pub struct UxIter<E> {
    min: E,
    max: E,
    rng: ChaCha8Rng,
}

impl<E> Iterator for UxIter<E>
where
    E: Clone + SampleUniform,
{
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        let distr = rand::distributions::Uniform::new_inclusive(
            self.min.clone(),
            self.max.clone(),
        );
        Some(self.rng.sample(distr))
    }
}

#[cfg(test)]
mod tests {
    use super::ranged;
    use crate::testing::assert_even_distr;
    use crate::testing::assert_first_fixed_then_random;
    use crate::*;

    /// Generator values should be evenly distributed within range.
    #[test]
    fn ranged_has_uniform_distribution() {
        assert_even_distr(ranged(..=100u64), 0, 100);
        assert_even_distr(ranged(10..=40), 10, 40);
        assert_even_distr(ranged(5..15), 5, 14);
    }

    /// Generator outputs the extreme values first.
    #[test]
    fn starts_width_min_and_max() {
        assert_first_fixed_then_random(ranged(4..=7), &[4, 7]);
        assert_first_fixed_then_random(ranged(200..300), &[200, 299]);

        assert_first_fixed_then_random(
            ranged(..),
            &[std::i64::MIN, std::i64::MAX],
        );
    }

    #[test]
    fn has_shrinker() {
        let gen = ranged(..10);
        let shrinker = gen.shrinker();
        let mut it = shrinker.candidates(123);
        assert!(it.next().is_some());
        assert!(it.next().is_some());
        assert!(it.next().is_some());
        assert!(it.next().is_some());
    }
}
