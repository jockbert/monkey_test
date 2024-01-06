use crate::BoxGen;
use std::fmt::Debug;

/// A collection of examples and their frequencies, a.k.a occurence count.
pub type Distribution<T> = std::collections::BTreeMap<T, usize>;

/// Create distribution with single example having 100% frequency.
pub fn single_value_distribution<T: Clone + Ord>(t: T) -> Distribution<T> {
    even_distribution_of::<T>(&[t])
}

/// Create distribution with all examples having the same even frequency.
pub fn even_distribution_of<T: Clone + Ord>(ts: &[T]) -> Distribution<T> {
    let mut result = Distribution::<T>::new();
    for t in ts {
        result.insert(t.clone(), 1);
    }
    result
}

/// Build a distribution by consuming a lot of values from given genereator.
fn collect_distribution<E>(gen_to_check: BoxGen<E>) -> Distribution<E>
where
    E: Clone + Ord + 'static,
{
    let mut result = Distribution::<E>::new();
    for example in gen_to_check.examples(1234).take(10_000) {
        let count = result.get(&example).map_or(1, |n| n + 1);
        result.insert(example, count);
    }

    result
}

/// Assert that the given generator have a distribution similar the the expected
/// distibution.
///
/// The examples returned from generator should be the same as in the expected
/// distribution, no more and no less. Additionaly, frequencies for the
/// different examples from the generator should, in percent, be approximately
/// the same as in the expected distribution.
pub fn assert_generator_distribution_similar_to<E>(
    actual_gen: BoxGen<E>,
    expected: Distribution<E>,
) where
    E: Clone + Ord + Debug + 'static,
{
    let allowed_deviation_percent = 2.0;
    let actual = collect_distribution(actual_gen);

    let actual_total_count: usize = actual.values().sum();
    let expected_total_count: usize = expected.values().sum();

    // Make sure actual keys are expected
    for example in actual.keys() {
        if !expected.contains_key(example) {
            let key_count = actual.get(example).expect("Key should exist");
            let percent = *key_count as f64 * 100.0 / actual_total_count as f64;
            panic!(
                "Unexpected generator example <{example:?}> with \
                 frequency {percent:0.0}%."
            )
        }
    }

    // Make sure expected keys are actually returned
    for example in expected.keys() {
        if !actual.contains_key(example) {
            let key_count = expected.get(example).expect("Key should exist");
            let percent =
                *key_count as f64 * 100.0 / expected_total_count as f64;
            panic!(
                "Generator never returned expected example <{example:?}>. \
                 Expected to have frequency {percent:0.0}%."
            )
        }
    }

    // Make sure actual and expected frequencies match
    for example in actual.keys() {
        let actual_key_count = actual.get(example).expect("Key should exist");

        let expected_key_count =
            expected.get(example).expect("Key should exist");

        let actual_percent =
            *actual_key_count as f64 * 100.0 / actual_total_count as f64;

        let expected_percent =
            *expected_key_count as f64 * 100.0 / expected_total_count as f64;

        if (expected_percent - actual_percent).abs()
            > (allowed_deviation_percent / 100.0)
        {
            panic!(
                "Frequency of example <{example:?}> is expected to be \
                    {expected_percent:0.0}%, but actually is \
                    {actual_percent:0.0}%."
            )
        }
    }
}

#[test]
#[should_panic(
    expected = "Unexpected generator example <10> with frequency 33%."
)]
fn assert_should_fail_on_unexpected_additional_generator_example() {
    let gen = crate::gen::u8::ranged(10..=12);
    let expected = even_distribution_of::<u8>(&[11, 12]);

    assert_generator_distribution_similar_to(gen, expected);
}

#[test]
#[should_panic(
    expected = "Generator never returned expected example <13>. Expected \
    to have frequency 33%."
)]
fn assert_should_fail_on_missing_example_in_generator() {
    let gen = crate::gen::u8::ranged(11..=12);
    let expected = even_distribution_of::<u8>(&[11, 12, 13]);

    assert_generator_distribution_similar_to(gen, expected);
}

#[test]
#[should_panic(
    expected = "Frequency of example <11> is expected to be 75%, but \
    actually is 50%."
)]
fn assert_should_fail_on_frequency_missmatch() {
    let gen = crate::gen::u8::ranged(11..=12);

    // Expected 11 as 75% and and 12 as 25% of the generator examples.
    let mut expected = Distribution::<u8>::new();
    expected.insert(11, 3);
    expected.insert(12, 1);

    assert_generator_distribution_similar_to(gen, expected);
}
