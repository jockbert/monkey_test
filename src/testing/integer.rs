use crate::BoxGen;
use num_traits::FromPrimitive;
use num_traits::PrimInt;
use std::cmp;

/// Pick large ammount of values from generator and assess if distribution
/// looks roughly even.
pub fn assert_even_distr<E>(
    generator_to_test: BoxGen<E>,
    expected_min: E,
    expected_max: E,
) where
    E: PrimInt + FromPrimitive + std::fmt::Debug + 'static,
{
    // Allowed deviation in percent from expected frequency.
    let max_error_percent = 15;
    let instances_per_value: usize = 2_000;
    let expected_range_limit: usize = 1_000_000;
    let max_occurrence_size: usize = 1000;

    let expected_range = (expected_max - expected_min + E::one())
        .to_usize()
        .unwrap_or(usize::MAX);

    // Pick so many elements such there should be roughly x instances of
    // each value.
    assert!(
        expected_range < expected_range_limit,
        "Expected range {expected_range} is too wide.
            Takes too long to test. \
            Choose a smaller range < {expected_range_limit} to test."
    );
    let total_count = expected_range * instances_per_value;

    // Count elements on entire range, or only on the low end of total range
    // if range is exta large.
    let occurrences_size: usize = cmp::min(expected_range, max_occurrence_size);
    let mut occurrences = vec![0usize; occurrences_size];

    let seed = crate::global_seed();
    let it = generator_to_test.examples(seed, 0..=10);
    for value in it.take(total_count) {
        // Do a bounds check
        if value < expected_min || value > expected_max {
            panic!(
                "{value:?} is out of bounds \
                    [{expected_min:?}, {expected_max:?}]"
            );
        }

        // Increment count if applicable.
        let index: usize = (value - expected_min).to_usize().expect(
            "difference between expected minimum and and generator value \
            should be less than usize max.",
        );
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
            let value: E = E::from_usize(index)
                .expect("index to fit inside the generic type E tested")
                + expected_min;
            panic!(
                "For value {value:?}, number of occurrences should be \
                    {instances_per_value}, but is {count} which deviates \
                    more than {max_error_percent}%. Seed={seed}"
            )
        }
    })
}
