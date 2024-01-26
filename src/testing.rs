use std::fmt::Debug;

pub mod distribution;
pub mod numbers;

use crate::BoxGen;

/// Makes sure generator is empty
pub fn assert_generator_is_empty<E>(gen_to_check: BoxGen<E>)
where
    E: Clone + 'static,
{
    assert_eq!(gen_to_check.examples(1234).count(), 0);
}

/// Makes sure generator has a shrinker that can give a shrinked example, which
/// is not the same as the original example.
pub fn assert_generator_can_shrink<E>(gen: BoxGen<E>, example_to_shrink: E)
where
    E: Clone + PartialEq + Debug + 'static,
{
    let shrinker = gen.shrinker();
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
pub fn assert_generator_cannot_shrink<E>(gen: BoxGen<E>, example_to_shrink: E)
where
    E: Clone + PartialEq + Debug + 'static,
{
    let shrinker = gen.shrinker();
    let candidate = shrinker.candidates(example_to_shrink.clone()).next();
    assert!(
        candidate.is_none(),
        "Expecting generator shrinker to not return a candidate, given \
        example {:?}, but got {:?}.",
        example_to_shrink,
        candidate
    );
}
