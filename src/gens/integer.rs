//! Generic generators for integer type values.

use crate::BoxGen;
use num_traits::PrimInt;
use rand::distr::uniform::SampleUniform;
use rand::Rng;
use rand::SeedableRng;
use std::ops::RangeBounds;

use crate::internal::int_bounds;

/// Roughly uniformly distributed range of values, with some overwheight to
/// extremes of given bounds. That is, bounds min and max and additionally the
/// value zero, if in range of given bounds.
pub fn ranged<E, B>(bounds: B) -> BoxGen<E>
where
    E: PrimInt + SampleUniform + std::fmt::Debug + 'static,
    B: RangeBounds<E>,
{
    let (min, max) = int_bounds::to_inclusive_range_tuple(&bounds);
    let mut extreme_values = vec![min, max];

    if min < E::zero() && E::zero() < max {
        extreme_values.push(E::zero());
    }

    let extremes = crate::gens::pick_evenly(&extreme_values);
    let randoms = completely_random(bounds);

    crate::gens::mix_with_ratio(&[(96, randoms), (6, extremes)])
}

/// Int generator with completely random distribution. This function has a long
/// name, since `ranged` should be preferred.
pub fn completely_random<E, B>(bounds: B) -> BoxGen<E>
where
    E: PrimInt + SampleUniform + std::fmt::Debug + 'static,
    B: RangeBounds<E>,
{
    let (min, max) = int_bounds::to_inclusive_range_tuple(&bounds);

    crate::gens::from_fn(move |seed, _size| {
        let distr = rand::distr::Uniform::new_inclusive(min, max)
            .expect("distribution from bounds");
        rand_chacha::ChaCha8Rng::seed_from_u64(seed).sample_iter(distr)
    })
    .with_shrinker(crate::shrinks::int_in_range(min, max))
}

#[cfg(test)]
mod tests {
    use crate::testing::assert_generator_can_shrink;
    use crate::testing::distribution::assert_generator_has_distribution_within_percent;
    use crate::testing::distribution::distribution_from_pairs;
    use crate::testing::integer::assert_even_distr;

    /// Generator values should be evenly distributed within range.
    #[test]
    fn random_inclusive_has_uniform_distribution() {
        assert_even_distr(super::completely_random(..=100u64), 0, 100);
        assert_even_distr(super::completely_random(5..=5), 5, 5);
        assert_even_distr(super::completely_random(5..=6), 5, 6);
        assert_even_distr(super::completely_random(5..=7), 5, 7);
    }

    /// Generator outputs the extreme values a little bit more often.
    /// It is especially important in large ranges, to explicitly test the
    /// extreme endpoints and not only random values. Otherwise, with a hundered
    /// examples tested, the extreme values might never come up.
    #[test]
    fn assert_two_percent_higher_frequency_of_min_and_max() {
        let ranged = super::ranged(2..=6);

        let expected = distribution_from_pairs(&[
            (21, 2),
            (19, 3),
            (19, 4),
            (19, 5),
            (21, 6),
        ]);

        assert_generator_has_distribution_within_percent(ranged, expected, 1.0)
    }

    #[test]
    fn has_shrinker() {
        assert_generator_can_shrink(super::ranged(..1000), 123);
    }
}
