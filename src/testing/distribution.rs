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

// Create distribution with specified ratio for each of the given examples.
pub fn distribution_from_pairs<T: Clone + Ord>(
    t_ratio_pairs: &[(usize, T)],
) -> Distribution<T> {
    let mut result = Distribution::<T>::new();
    for (ratio, t) in t_ratio_pairs {
        result.insert(t.clone(), *ratio);
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
    let allowed_deviation_percent = 1.0;
    let actual = collect_distribution(actual_gen);

    let actual_total_count: usize = actual.values().sum();
    let expected_total_count: usize = expected.values().sum();

    // Make sure actual keys are expected
    for example in actual.keys() {
        if !expected.contains_key(example) {
            let key_count = actual.get(example).expect("Key should exist");
            let percent = format_percent(*key_count, actual_total_count);

            let formatted = format_distribution(&actual);
            panic!(
                "Unexpected generator example <{example:?}> with \
                 frequency {percent}.\n\
                 \n\
                 Got distribution:\n\
                 {formatted}"
            )
        }
    }

    // Make sure expected keys are actually returned
    for example in expected.keys() {
        if !actual.contains_key(example) {
            let key_count = expected.get(example).expect("Key should exist");
            let percent = format_percent(*key_count, expected_total_count);

            let formatted = format_distribution(&actual);
            panic!(
                "Generator never returned expected example <{example:?}>. \
                 Expected to have frequency {percent}.\n\
                 \n\
                 Got distribution:\n\
                 {formatted}"
            )
        }
    }

    // Make sure actual and expected frequencies match
    for example in actual.keys() {
        let actual_key_count = actual.get(example).expect("Key should exist");

        let expected_key_count =
            expected.get(example).expect("Key should exist");

        let actual_percent =
            calc_percent(*actual_key_count, actual_total_count);

        let expected_percent =
            calc_percent(*expected_key_count, expected_total_count);

        if (expected_percent - actual_percent).abs() > allowed_deviation_percent
        {
            let formatted = format_distribution(&actual);
            panic!(
                "Frequency of example <{example:?}> is expected to be \
                    {expected_percent:0.1}%, but actually is \
                    {actual_percent:0.1}%.\n\
                    \n\
                    Got distribution:\
                    {formatted}"
            )
        }
    }
}

fn calc_percent(this_count: usize, total_count: usize) -> f64 {
    this_count as f64 * 100.0 / total_count as f64
}

fn format_percent(this_count: usize, total_count: usize) -> String {
    let p = calc_percent(this_count, total_count);
    format! {"{p:0.1}%"}
}

fn format_distribution<T: Debug>(d: &Distribution<T>) -> String {
    let total_count = d.values().sum();

    d.iter().fold("{\n".to_string(), |result, (key, count)| {
        let p = format_percent(*count, total_count);
        result + &format! {"    {key:?}\t{p}\n"}
    }) + "}\n"
}

#[test]
#[should_panic(
    expected = "Unexpected generator example <10> with frequency 33.3%."
)]
fn assert_should_fail_on_unexpected_additional_generator_example() {
    let gen = crate::gen::fixed::in_loop(&[10, 11, 12]);
    let expected = even_distribution_of::<u8>(&[11, 12]);

    assert_generator_distribution_similar_to(gen, expected);
}

#[test]
#[should_panic = "Generator never returned expected example <13>. Expected \
    to have frequency 33.3%."]
fn assert_should_fail_on_missing_example_in_generator() {
    let gen = crate::gen::fixed::in_loop(&[11, 12]);
    let expected = even_distribution_of::<u8>(&[11, 12, 13]);

    assert_generator_distribution_similar_to(gen, expected);
}

#[test]
#[should_panic = "Frequency of example <11> is expected to be 75.0%, but \
    actually is 50.0%."]
fn assert_should_fail_on_frequency_missmatch() {
    let gen = crate::gen::fixed::in_loop(&[11, 12]);

    // Expected 11 as 75% and and 12 as 25% of the generator examples.
    let mut expected = Distribution::<u8>::new();
    expected.insert(11, 3);
    expected.insert(12, 1);

    assert_generator_distribution_similar_to(gen, expected);
}
