/// Result summary from evaluation of a property tested.
#[derive(Debug, PartialEq)]
pub enum MonkeyResult<E> {
    /// A successful monkey test result.
    MonkeyOk(),

    /// A failed monkey test result.
    MonkeyErr {
        /// The minimum example found that disproves the property. In case a
        /// shinker is provided, this is the shrunken failure example, possibly
        /// separate from original failure. In other cases the same as original
        /// failure.
        minimum_failure: E,

        /// The original (first found) example that disproves the property.
        original_failure: E,

        /// Other examples that also disproves the property. In case a shinker
        /// is provided, this vector is populated with non-minimum values found
        /// as part of the shrinking process of the original failure example.
        /// Some found failures may be exlided from list if many failure
        /// examples are found
        some_other_failures: Vec<E>,

        /// Successful example count tried before finding a failure.
        success_count: u64,

        /// Number of examples used when trying to shrink original failure
        /// example.
        shrink_count: u64,

        /// The seed used for generating the examples. Can be useful for
        /// reproducing the failed test run.
        seed: u64,
    },
}

impl<E> MonkeyResult<E> {
    /// Verify that the result is a failure and that the minimum failure equals
    /// given argument `expected_minimum_failure`.
    ///
    /// # Panics
    ///
    /// The function panics if there is not a failure result matching
    /// `expected_minimum_failue`.
    pub fn assert_minimum_failure(&self, expected_minimum_failure: E) -> &Self
    where
        E: Debug + PartialEq,
    {
        match self {
            MonkeyResult::MonkeyOk() => panic!(
                "Expecting property to fail for some example, but it never failed."),
            MonkeyResult::MonkeyErr { minimum_failure, .. } => assert!(
                expected_minimum_failure == *minimum_failure,
                "Expecting property to fail, which it did, but also \
                expecting minimum failure to be equal to {:?}, but got {:?}",
                expected_minimum_failure,
                minimum_failure,
            )
        };
        self
    }
}

use std::fmt::Debug;

use crate::config::*;

pub fn evaluate_property<E, P>(cg: &ConfAndGen<E>, prop: P) -> MonkeyResult<E>
where
    E: Clone + 'static,
    P: Fn(E) -> bool,
{
    let mut it = cg.gen.examples(cg.conf.seed);

    for i in 0..cg.conf.example_count {
        let example = it.next();

        let (e, success) = match example {
            Some(e) => (e.clone(), prop(e.clone())),
            None => panic!("To few examples. Only got {i}"),
        };

        if !success {
            let shrinked_values =
                do_shrink(prop, cg.gen.shrinker().candidates(e.clone()));

            // All but last shrinked value, up to a max limit
            let other_count = shrinked_values.len().min(100).max(1) as u64 - 1;
            let some_other_failures = shrinked_values
                .clone()
                .into_iter()
                .take(other_count as usize)
                .collect::<Vec<_>>();

            let minimum_failure =
                shrinked_values.last().cloned().unwrap_or(e.clone());

            return MonkeyResult::<E>::MonkeyErr {
                minimum_failure,
                original_failure: e,
                some_other_failures,
                success_count: i as u64,
                shrink_count: shrinked_values.len() as u64,
                seed: cg.conf.seed,
            };
        }
    }

    MonkeyResult::<E>::MonkeyOk()
}

fn do_shrink<E, P>(prop: P, it: Box<dyn Iterator<Item = E>>) -> Vec<E>
where
    E: Clone,
    P: Fn(E) -> bool,
{
    let mut shrinked_examples = vec![];

    for example in it.take(1000) {
        if !prop(example.clone()) {
            shrinked_examples.push(example);
        }
    }

    shrinked_examples
}
