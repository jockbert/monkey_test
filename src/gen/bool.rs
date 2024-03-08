//! Generators for boolean type.

use crate::BoxGen;
use crate::BoxShrink;
use crate::Gen;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

/// Generator of boolean values where ratio can be scewed according to given
/// ratios.
pub fn with_ratio(ratio_false: u32, ratio_true: u32) -> BoxGen<bool> {
    Box::new(BoolGen {
        ratio_false,
        ratio_true,
    })
}

/// Uniformly distributed generator of `true` and `false`.
pub fn evenly() -> BoxGen<bool> {
    with_ratio(1, 1)
}

#[derive(Clone)]
struct BoolGen {
    ratio_false: u32,
    ratio_true: u32,
}

impl Gen<bool> for BoolGen {
    fn examples(&self, seed: u64) -> crate::BoxIter<bool> {
        Box::new(BoolIter {
            rng: rand_chacha::ChaCha8Rng::seed_from_u64(seed),
            ratio_true: self.ratio_true,
            ratio_false: self.ratio_false,
        })
    }

    fn shrinker(&self) -> BoxShrink<bool> {
        crate::shrink::bool()
    }
}

struct BoolIter {
    ratio_false: u32,
    ratio_true: u32,
    rng: ChaCha8Rng,
}

impl Iterator for BoolIter {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let numerator = self.ratio_true;
        let denominator = self.ratio_false + self.ratio_true;
        Some(self.rng.gen_ratio(numerator, denominator))
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::distribution::assert_generator_has_distribution_within_percent;
    use crate::testing::distribution::distribution_from_pairs;
    use crate::testing::distribution::even_distribution_of;

    #[test]
    fn any_has_uniform_distribution() {
        let bools = super::evenly();

        let expected = even_distribution_of(&[false, true]);

        assert_generator_has_distribution_within_percent(bools, expected, 1.0)
    }

    #[test]
    fn with_ratio_has_distribution_as_specified() {
        let bools = super::with_ratio(9, 1);

        let expected = distribution_from_pairs(&[(9, false), (1, true)]);

        assert_generator_has_distribution_within_percent(bools, expected, 1.0)
    }

    #[test]
    fn has_shrinker_that_shrinks_to_false() {
        let shrinker = super::evenly().shrinker();

        let mut candidates_of_true = shrinker.candidates(true);
        assert_eq!(candidates_of_true.next(), Some(false));
        assert_eq!(candidates_of_true.next(), None);

        let mut candidates_of_false = shrinker.candidates(false);
        assert_eq!(candidates_of_false.next(), None);
    }
}
