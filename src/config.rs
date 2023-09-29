use std::marker::PhantomData;

pub use crate::runner::MonkeyResult;
use crate::{shrink::NoShrink, Gen, Shrink};

/// Configuration for executing monkey tests.
#[derive(Clone)]
pub struct Conf {
    /// See [Conf::with_seed].
    pub example_count: u32,
    /// See [Conf::with_seed].
    pub seed: u64,
}

/// Configuration for executing monkey tests, including the generator.
pub struct ConfAndGen<E, G>
where
    G: Gen<E>,
{
    phantom: PhantomData<E>,
    /// The configuration to use.
    pub conf: Conf,
    /// See [Conf::with_generator].
    pub gen: G,
}

/// Configuration for executing monkey tests, including the generator and an
/// explicit shrinker.
pub struct ConfGenAndShrink<E, G, S>
where
    G: Gen<E>,
    S: Shrink<E>,
{
    phantom: PhantomData<E>,
    /// The configuration to use.
    pub conf: Conf,
    /// See [Conf::with_generator].
    pub gen: G,
    /// See [ConfAndGen::with_shrinker].
    pub shrinker: S,
}

impl Conf {
    /// Specify which single generator to use in test.
    pub fn with_generator<E, G>(&self, gen: G) -> ConfAndGen<E, G>
    where
        E: 'static,
        G: Gen<E>,
    {
        ConfAndGen::<E, G> {
            phantom: PhantomData,
            conf: self.clone(),
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

impl<E, G> ConfAndGen<E, G>
where
    E: std::fmt::Debug + Clone + 'static,
    G: Gen<E>,
{
    /// Check that the property holds for all generated example values.
    /// It returns a [`MonkeyResult`](MonkeyResult) to indicate success or failure.
    pub fn check_true<P>(&self, prop: P) -> MonkeyResult<E>
    where
        P: Fn(E) -> bool,
    {
        self.with_shrinker(NoShrink {}).check_true(prop)
    }

    /// Check that the property holds for all generated example values.
    /// It panics on failure.
    pub fn assert_true<P>(&self, prop: P)
    where
        P: Fn(E) -> bool,
    {
        self.with_shrinker(NoShrink {}).assert_true(prop)
    }

    /// Add/change which shriker to use if a failing example is found.
    pub fn with_shrinker<S2>(&self, shrink: S2) -> ConfGenAndShrink<E, G, S2>
    where
        S2: Shrink<E>,
    {
        ConfGenAndShrink::<E, G, S2> {
            phantom: PhantomData,
            shrinker: shrink,
            gen: self.gen.clone(),
            conf: self.conf.clone(),
        }
    }
}

impl<E, G, S> ConfGenAndShrink<E, G, S>
where
    E: std::fmt::Debug + Clone + 'static,
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
    pub fn with_shrinker<S2>(&self, shrink: S2) -> ConfGenAndShrink<E, G, S2>
    where
        S2: Shrink<E>,
    {
        ConfGenAndShrink::<E, G, S2> {
            phantom: PhantomData,
            shrinker: shrink,
            gen: self.gen.clone(),
            conf: self.conf.clone(),
        }
    }
}