//! A collection of generators suitable for sizing up collections of data.

use std::cmp::max;
use std::cmp::min;

use crate::BoxGen;
use crate::BoxIter;
use rand::distributions::Uniform;
use rand::Rng;
use rand::SeedableRng;

/// A progressively increasing usize generator, with some sort of reasobable
/// default values. For more details, see [progressively_increasing].
///
/// For now, the max size example value generated starts at really low size, its
/// max size increases with roughly 30% for each example and is limited to max
/// size 100_000.
pub fn default() -> BoxGen<usize> {
    progressively_increasing(0, 30, 100_000)
}

/// Produces a usize-generator that should be reasonably okay for generating the
/// size of collections.
///
/// Some type of testing might not handle large collections or do have
/// unacceptable performance when using big collections, so in those cases the
/// collection size need to be kept small.
///
/// Other code under test might handle large collections well, but large
/// collections still take more time to generate than small collections.
/// Therefore, in this generator, the maximum possible generated example size
/// value progressively grows when iterating over the examples, first trying
/// with small sizes before continouing with larger sizes.
///
/// All in all, this should hopefully ensure a good test coverage, first testing
/// with small sizes and later on with possibly larger ones too, hitting a good
/// balance between test coverage and performance.
pub fn progressively_increasing(
    start_size: usize,
    percent_increase: usize,
    max_size: usize,
) -> BoxGen<usize> {
    crate::gens::from_fn(move |seed| {
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);

        max_iterator(start_size, percent_increase, max_size)
            .map(move |max| rng.sample(Uniform::new_inclusive(0usize, max)))
    })
    .with_shrinker(crate::shrinks::int_to_zero())
}

fn max_iterator(
    start_size: usize,
    percent_increase: usize,
    max_size: usize,
) -> BoxIter<usize> {
    Box::new(std::iter::successors(
        Some(min(start_size, max_size)),
        move |last| {
            let inc = max(last * percent_increase / 100, 1);
            Some(min(max_size, last + inc))
        },
    ))
}

#[cfg(test)]
mod test {
    use assert_approx_eq::assert_approx_eq;

    use crate::testing::assert_iter_eq;

    #[test]
    pub fn max_iterator_should_clamp_at_max_size() {
        let max_size = 7;
        let extra_large = 7_000;

        assert_iter_eq(
            super::max_iterator(extra_large, extra_large, max_size).take(6),
            vec![max_size, max_size, max_size, max_size, max_size, max_size],
            "stay flat when at max size",
        );
    }

    #[test]
    pub fn max_iterator_should_grow_with_at_least_one() {
        let percent_increase = 10;

        assert_iter_eq(
            super::max_iterator(3, percent_increase, 1337).take(30),
            vec![
                // always increasing with at least 1, more than 10%
                3, 4, 5, 6, 7, 8, 9, 10,
                // 10% being trucated down to increment of 1
                11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
                // 10% being truncated down to increment of 2
                22, 24, 26, 28, 30,
                // 10% being truncated down to increment of 3
                33, 36, 39, 42,
                // 10% being truncated down to increment of 4
                46, 50,
                // 10% of 50 is a increase of 5 exactly ......fighting rust_fmt
                55,
            ],
            "max iterator should increase with largest value of percent, \
            but at least with 1 to get growth at all",
        );
    }

    #[test]
    pub fn max_iterator_should_normally_grow_with_percent_increase() {
        let percent_increase = 100;

        assert_iter_eq(
            super::max_iterator(16, percent_increase, 1337).take(10),
            vec![16, 32, 64, 128, 256, 512, 1024, 1337, 1337, 1337],
            "size should grow with increase 100% up to max value 1337",
        );
    }

    #[test]
    pub fn generator_should_never_exceed_max_size() {
        let max_size = 7;
        let extra_large = 7_000;

        let max_value_generated =
            super::progressively_increasing(extra_large, extra_large, max_size)
                .examples(1337)
                .take(1_000)
                .max();

        assert_eq!(max_value_generated, Some(max_size));
    }

    #[test]
    pub fn generator_should_behave_reasonably() {
        // If max increments linearly with one each example taken, from start
        // value 0, the aggregated sum of all generated values should end up
        // near n^2 / 4, where n is the number of samples taken.

        let always_increase_with_one = 0;

        let n = 10_000;

        let sum_of_examples = super::progressively_increasing(
            0,
            always_increase_with_one,
            usize::MAX,
        )
        .examples(1337)
        .take(n)
        .sum::<usize>() as f64;

        let expected_sum = n as f64 * n as f64 / 4.0;
        let diff = expected_sum * 0.005;

        assert_approx_eq!(sum_of_examples, expected_sum, diff);
    }
}
