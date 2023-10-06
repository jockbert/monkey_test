//! Generators for values of type usize.

use std::ops::{Bound, RangeBounds, RangeInclusive};

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::{
    shrink::{NoShrink, NumShrink},
    Gen,
};

use super::{chain::ChainGen, fixed::SequenceGen};

/// Uniformly distributed range of value
pub fn any() -> ChainGen<usize, SequenceGen<usize>, NoShrink<usize>, UsizeGen, NumShrink> {
    ranged(..)
}

/// Uniformly distributed range of values
pub fn ranged<B>(
    bounds: B,
) -> ChainGen<usize, SequenceGen<usize>, NoShrink<usize>, UsizeGen, NumShrink>
where
    B: RangeBounds<usize>,
{
    let min: usize = match bounds.start_bound() {
        Bound::Included(x) => *x,
        Bound::Excluded(x) => *x + 1,
        Bound::Unbounded => usize::MIN,
    };

    let max: usize = match bounds.end_bound() {
        Bound::Included(x) => *x,
        Bound::Excluded(x) => *x - 1,
        Bound::Unbounded => usize::MAX,
    };

    crate::gen::fixed::sequence(&[min, max]).chain(&UsizeGen { min, max })
}

/// Generator of random usize values.
#[derive(Clone)]
pub struct UsizeGen {
    min: usize,
    max: usize,
}

impl Gen<usize, NumShrink> for UsizeGen {
    fn examples(&self, seed: u64) -> crate::SomeIter<usize> {
        Box::new(UsizeIter {
            range: self.min..=self.max,
            rng: rand_chacha::ChaCha8Rng::seed_from_u64(seed),
        })
    }

    fn shrinker(&self) -> NumShrink {
        NumShrink {}
    }
}

/// Iterator of random integer values.
pub struct UsizeIter {
    range: RangeInclusive<usize>,
    rng: ChaCha8Rng,
}

impl Iterator for UsizeIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.rng.gen_range(self.range.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::any;
    use super::ranged;
    use crate::testing::assert_even_distr;
    use crate::testing::assert_first_fixed_then_random;

    /// Generator values should be evenly distributed within range.
    #[test]
    fn ranged_has_uniform_distribution() {
        assert_even_distr(ranged(..=100), 0, 100);
        assert_even_distr(ranged(10..=40), 10, 40);
        assert_even_distr(ranged(5..15), 5, 14);
    }

    /// Generator outputs the extreme values first.
    #[test]
    fn starts_width_min_and_max() {
        assert_first_fixed_then_random(ranged(4..=7), &[4usize, 7usize]);
        assert_first_fixed_then_random(ranged(200..300), &[200usize, 299usize]);
        assert_first_fixed_then_random(any(), &[usize::MIN, usize::MAX]);
    }
}
