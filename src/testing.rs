use std::fmt::Debug;

pub mod distribution;
pub mod integer;

use crate::BoxGen;
use crate::BoxShrink;

/// Makes sure generator is empty
pub fn assert_generator_is_empty<E>(gen_to_check: BoxGen<E>)
where
    E: Clone + 'static,
{
    assert_eq!(gen_to_check.examples(1234).count(), 0);
}

/// Compares two iterators and makes sur elements are equal
pub fn assert_iter_eq<A, E>(actual: A, expected: E, because_message: &str)
where
    A: IntoIterator,
    E: IntoIterator,
    A::Item: Debug + PartialEq<E::Item>,
    E::Item: Debug,
{
    let safety_termination = 10_000;
    assert_eq!(
        actual
            .into_iter()
            .take(safety_termination)
            .collect::<Vec<_>>(),
        expected
            .into_iter()
            .take(safety_termination)
            .collect::<Vec<_>>(),
        "{}",
        because_message,
    )
}

/// Makes sure generator has a shrinker that can give a shrinked example, which
/// is not the same as the original example.
pub fn assert_generator_can_shrink<E>(
    generator: BoxGen<E>,
    example_to_shrink: E,
) where
    E: Clone + PartialEq + Debug + 'static,
{
    let shrinker = generator.shrinker();
    let candidate = shrinker.candidates(example_to_shrink.clone()).next();
    assert!(
        candidate.is_some(),
        "Expecting generator shrinker to return a candidate, given \
        example {:?}, but got none.",
        example_to_shrink
    );

    assert_ne!(
        candidate.unwrap(),
        example_to_shrink,
        "Expecting shrinking candidate to not equal given example {:?}, but \
        they are the same.",
        example_to_shrink
    );
}

/// Makes sure generator has a shrinker that cannot shrink.
pub fn assert_generator_cannot_shrink<E>(
    generator: BoxGen<E>,
    example_to_shrink: E,
) where
    E: Clone + PartialEq + Debug + 'static,
{
    let shrinker = generator.shrinker();
    let candidate = shrinker.candidates(example_to_shrink.clone()).next();
    assert!(
        candidate.is_none(),
        "Expecting generator shrinker to not return a candidate, given \
        example {:?}, but got {:?}.",
        example_to_shrink,
        candidate
    );
}

pub fn assert_shrinker_has_at_least_these_candidates<E>(
    shrinker: BoxShrink<E>,
    original: E,
    expected: &[E],
) where
    E: Clone + Debug + PartialEq,
{
    let mut left_to_expect = expected.to_vec();

    shrinker
        .candidates(original)
        .take(1_000)
        .for_each(|candidate| {
            left_to_expect
                .iter()
                .position(|expected| *expected == candidate)
                .map(|index| left_to_expect.remove(index));
        });

    assert!(
        left_to_expect.is_empty(),
        "Expecting shrinker to return all expected candidates, but never \
        got {:?}",
        left_to_expect
    )
}

/// Assert that shrinker has at least three candidates given original example.
pub fn assert_shrinker_has_some_candidates_given<E>(
    shrinker: BoxShrink<E>,
    original: E,
) where
    E: Clone + Debug,
{
    let length = shrinker.candidates(original.clone()).take(3).count();

    assert_eq!(
        length, 3,
        "Expecting shrinker to have at least 3 candidates \
        given original example {:?}, but only got candidate count {}.",
        original, length
    )
}
