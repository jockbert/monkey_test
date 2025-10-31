use crate::BoxShrink;
use crate::ConfAndGen;
use std::fmt::Debug;

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
        /// Some found failures may be exluded from list if many failure
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

        /// Optional title of the failed property.
        title: Option<String>,

        /// Reason for the failed propert. This reason is from the
        /// [minimum failure](self::MonkeyResult#variant.MonkeyErr.field.minimum_failure)
        /// example. Other failures can have other reasons not shown here.
        reason: String,
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

pub fn evaluate_property<E, P>(cg: &ConfAndGen<E>, prop: P) -> MonkeyResult<E>
where
    E: Clone + 'static,
    P: Fn(E) -> Result<(), String>,
{
    let mut it = cg.gen.examples(cg.conf.seed);

    for i in 0..cg.conf.example_count {
        let example = it.next();

        let (first_example, maybe_first_reason) = match example {
            Some(e) => (e.clone(), prop(e.clone())),
            None => panic!("To few examples. Only got {i}"),
        };

        if let Err(first_reason) = maybe_first_reason {
            let shrinked_values =
                do_shrink(prop, first_example.clone(), cg.gen.shrinker());

            // All but last shrinked value, up to a max limit
            let other_count = shrinked_values.len().clamp(1, 100) as u64 - 1;
            let some_other_failures = shrinked_values
                .clone()
                .into_iter()
                .map(|(example, _)| example)
                .take(other_count as usize)
                .collect::<Vec<_>>();

            let (minimum_failure, minimum_reason) = shrinked_values
                .last()
                .cloned()
                .unwrap_or((first_example.clone(), first_reason));

            return MonkeyResult::<E>::MonkeyErr {
                minimum_failure,
                original_failure: first_example,
                some_other_failures,
                success_count: i as u64,
                shrink_count: shrinked_values.len() as u64,
                seed: cg.conf.seed,
                title: cg.title.clone(),
                reason: minimum_reason,
            };
        }
    }

    MonkeyResult::<E>::MonkeyOk()
}

fn do_shrink<E, P>(
    prop: P,
    original_failure: E,
    shrinker: BoxShrink<E>,
) -> Vec<(E, String)>
where
    E: Clone,
    P: Fn(E) -> Result<(), String>,
{
    let shrink_effort = 10_000;
    let mut shrinked_examples = vec![];
    let mut candidates = shrinker.candidates(original_failure);

    for _ in 0..shrink_effort {
        match candidates.next() {
            Some(candidate) => {
                let maybe_failure_reason = prop(candidate.clone());
                if let Err(reason) = maybe_failure_reason {
                    shrinked_examples.push((candidate.clone(), reason));
                    candidates = shrinker.candidates(candidate);
                }
            }
            None => return shrinked_examples,
        }
    }

    shrinked_examples
}
