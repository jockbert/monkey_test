use std::marker::PhantomData;

pub use crate::runner::MonkeyResult;
use crate::{shrink::NoShrink, Gen, Shrink};

/// Configuration for executing monkey tests.
pub struct Conf {
    /// See [Conf::with_seed].
    pub example_count: u32,
    /// See [Conf::with_seed].
    pub seed: u64,
}

/// Configuration for executing monkey tests, using a single generator.
pub struct ConfWithGen<E, G, S>
where
    G: Gen<E>,
    S: Shrink<E>,
{
    phantom: PhantomData<E>,
    /// See [Conf::with_example_count].
    pub example_count: u32,
    /// See [Conf::with_seed].
    pub seed: u64,
    /// See [Conf::with_generator].
    pub gen: G,
    /// See [ConfWithGen::with_shrinker].
    pub shrinker: Option<S>,
}

}

impl Conf {
    /// Specify which single generator to use in test.
    pub fn with_generator<E, G>(&self, gen: G) -> ConfWithGen<E, G, NoShrink>
    where
        E: 'static,
        G: Gen<E>,
    {
        ConfWithGen::<E, G, NoShrink> {
            phantom: PhantomData,
            example_count: self.example_count,
            seed: self.seed,
            shrinker: None,
            gen,
        }
    }

    /// Specify the number of examples to use in test. If not specified, the
    /// default number of examples are used. If the default number of examples
    /// are explicitly changed, it is set to 100.
    pub fn with_example_count(&self, example_count: u32) -> Conf {
        Conf {
            example_count,
            seed: self.seed,
        }
    }

    /// Specify which seed to use for random values. Specifying the seed is
    /// useful for reproducing a failing test run. Use this with caution, since
    /// using a seed hinders new test runs to use other examples than already
    /// used in earlier test runs.
    pub fn with_seed(&self, seed: u64) -> Conf {
        Conf {
            example_count: self.example_count,
            seed,
        }
    }
}

impl Default for Conf {
    /// Create new configuration with default values
    fn default() -> Self {
        Self {
            example_count: 100,
            seed: 0,
        }
    }
}

impl<E, G, S> ConfWithGen<E, G, S>
where
    E: std::fmt::Debug + Clone,
    G: Gen<E>,
    S: Shrink<E>,
{
    /// Check that the property holds for all generated example values.
    /// It returns a [`MonkeyResult`](MonkeyResult) to indicate success or failure.
    pub fn check_true<P>(&self, prop: P) -> MonkeyResult<E>
    where
        P: Fn(E) -> bool,
    {
        crate::runner::evaluate_property(self, prop)
    }

    /// Check that the property holds for all generated example values.
    /// It panics on failure.
    pub fn assert_true<P>(&self, prop: P)
    where
        P: Fn(E) -> bool,
    {
        if let MonkeyResult::MonkeyErr {
            minimum_failure,
            seed,
            ..
        } = self.check_true(prop)
        {
            panic!(
                "Monkey test property failed!\n\
                Counterexample: {:?}\n\
                Reproduction seed: {}\n",
                minimum_failure, seed
            )
        }
    }

    /// Add/change which shriker to use if a failing example is found.
    pub fn with_shrinker<S2>(&self, shrink: S2) -> ConfWithGen<E, G, S2>
    where
        S2: Shrink<E>,
    {
        ConfWithGen::<E, G, S2> {
            phantom: PhantomData,
            shrinker: Some(shrink),
            gen: self.gen.clone(),
            example_count: self.example_count,
            seed: self.seed,
        }
    }
}
