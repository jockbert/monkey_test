use crate::BoxGen;
use num::Integer;
use std::cmp;

/// Assert that the first values from generator are the expected fixed
/// values and that value after that is at least somewhat random.
pub fn assert_first_fixed_then_random<E>(
    generator_to_test: BoxGen<E>,
    expected_fixed_values: &[E],
) where
    E: Integer + Clone + num::cast::AsPrimitive<usize> + std::fmt::Debug,
{
    let trial_count = 10;
    let mut first_randoms: Vec<E> = vec![];

    for seed in 0..trial_count {
        let mut it = generator_to_test.examples(seed);

        // Make sure that he first values of generators are the fixed ones.
        expected_fixed_values.iter().enumerate().for_each(
            |(index, expected_fixed_value)| {
                let actual = it.next().expect("generator should have some");
                assert_eq!(
                    *expected_fixed_value, actual,
                    "Generator value index {index} is expected to have \
                         the fixed value {expected_fixed_value:?}, \
                         but is {actual:?}."
                );
            },
        );

        // Collect first random value for each generator.
        first_randoms.push(it.next().expect("generator should have some"))
    }

    // At least some of the random values should differ.
    assert!(
        first_randoms.iter().any(|r| *r != first_randoms[0]),
        "At least some of the first random value after fixed values \
            should differ, but after {trial_count} trial got {first_randoms:?}"
    );
}

/// Pick large ammount of values from generator and assess if distribution
/// looks roughly even.
pub fn assert_even_distr<E>(
    generator_to_test: BoxGen<E>,
    expected_min: E,
    expected_max: E,
) where
    E: Integer + Clone + num::cast::AsPrimitive<usize> + std::fmt::Debug,
{
    let instances_per_value: usize = 10_000;
    let expected_range_limit: usize = 1_000_000;
    let max_error_percent = 15;

    // Count elements on entire range, or only on the low end of total range
    // if  range is exta large.
    let occurrences_size: usize =
        cmp::min(expected_max.sub(expected_min).as_() + 1, 1000usize);
    let mut occurrences = vec![0usize; occurrences_size];

    // Pick so many elements such there should be roughly x instances of
    // each value.
    let expected_range = (expected_max - expected_min).as_();
    assert!(
        expected_range < expected_range_limit,
        "Expected range {expected_range} is too wide.
            Takes too long to test. \
            Choose a smaller range < {expected_range_limit} to test."
    );
    let total_count = expected_range * instances_per_value;

    // TODO: use better seedö
    let it = generator_to_test.examples(1337);
    for value in it.take(total_count) {
        // Do a bounds check
        if value < expected_min || value > expected_max {
            panic!(
                "{value:?} is out of bounds \
                    [{expected_min:?}, {expected_max:?}]"
            );
        }

        // Increment count if applicable.
        let index: usize = (value - expected_min).as_();
        if index < occurrences_size {
            occurrences[index] += 1;
        }
    }

    // Verify distribution
    let max_error_count: usize = instances_per_value * max_error_percent / 100;

    occurrences.iter().enumerate().for_each(|(index, count)| {
        if count > &(instances_per_value + max_error_count)
            || count < &(instances_per_value - max_error_count)
        {
            let value = index + expected_min.as_();
            panic!(
                "For value {value}, number of occurrences should be \
                    {instances_per_value}, but is {count} which deviates \
                    more than {max_error_percent}%"
            )
        }
    })
}
