use crate::internal::sample_target::Ratio;
use crate::internal::sample_target::SampleTarget;
use crate::BoxGen;
use rand::Rng;
use rand::SeedableRng;

/// Pick from given evenly distributed examples. Please note,
/// the generator has no associated shrinker.
pub fn pick_evenly<E>(examples: &[E]) -> BoxGen<E>
where
    E: Clone + 'static + core::fmt::Debug,
{
    pick_with_sample_target(SampleTarget::evenly(examples))
}

/// Pick one of given examples with frequencies by given ratios. Please note,
/// the generator has no associated shrinker.
///
/// Example, where first value is picked 10% (= 1/(1+4+5))
/// of the time, second value is picked 40% (= 4/(1+4+5)) of the time and
/// third valueis picked 50% (=5/(1+4+5)) of the time.
/// ```rust
/// use monkey_test::*;
///
/// let gen_ = gens::pick_with_ratio(&[(1, 'a'), (4, 'b'), (5, 'c')]);
/// ```
pub fn pick_with_ratio<E>(ratios_and_examples: &[(Ratio, E)]) -> BoxGen<E>
where
    E: Clone + 'static + core::fmt::Debug,
{
    pick_with_sample_target(SampleTarget::with_ratios(ratios_and_examples))
}

fn pick_with_sample_target<E>(sample_target: SampleTarget<E>) -> BoxGen<E>
where
    E: Clone + 'static + core::fmt::Debug,
{
    crate::gens::from_fn(move |seed, _size| {
        let high = sample_target.sample_domain_max();
        let distr = rand::distr::Uniform::new_inclusive(1usize, high).unwrap();
        let rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);
        let sample_target = sample_target.clone();

        rng.sample_iter(distr).flat_map(move |sample| {
            sample_target.target_from_sample(sample).cloned()
        })
    })
}

#[cfg(test)]
mod test {
    use crate::testing::distribution::assert_generator_has_distribution_within_percent;
    use crate::testing::distribution::distribution_from_pairs;
    use crate::testing::distribution::even_distribution_of;
    use crate::testing::distribution::single_value_distribution;

    #[test]
    #[should_panic(
        expected = "Given argument [] has no target value with non-zero ratio."
    )]
    fn pick_with_ratio_panics_on_missing_options() {
        super::pick_with_ratio::<u8>(&[]);
    }

    #[test]
    #[should_panic(expected = "Given argument [(0, 'x')] has no target value \
        with non-zero ratio.")]
    fn pick_with_ratio_panics_on_zero_ratio() {
        super::pick_with_ratio(&[(0, 'x')]);
    }

    #[test]
    fn pick_with_ratio_handles_single_option() {
        assert_generator_has_distribution_within_percent(
            super::pick_with_ratio(&[(255, 'x')]),
            single_value_distribution('x'),
            1.0,
        );
    }

    #[test]
    fn pick_with_ratio_follow_given_ratios() {
        assert_generator_has_distribution_within_percent(
            super::pick_with_ratio(&[(50, 'b'), (25, 'c'), (25, 'a')]),
            distribution_from_pairs(&[(1, 'a'), (2, 'b'), (1, 'c')]),
            1.0,
        );
    }

    #[test]
    fn pick_evenly_is_evenly_distributed() {
        assert_generator_has_distribution_within_percent(
            super::pick_evenly(&['b', 'c', 'a']),
            even_distribution_of(&['a', 'b', 'c']),
            1.0,
        );
    }
}
