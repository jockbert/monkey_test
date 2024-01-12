pub use crate::runner::MonkeyResult;
use crate::BoxGen;
use crate::BoxShrink;

/// Configuration for executing monkey tests.
#[derive(Clone)]
pub struct Conf {
    /// See [Conf::with_seed].
    pub example_count: u32,
    /// See [Conf::with_seed].
    pub seed: u64,
}

/// Configuration for executing monkey tests, including the generator.
pub struct ConfAndGen<E>
where
    E: Clone,
{
    /// The configuration to use.
    pub conf: Conf,
    /// See [Conf::with_generator].
    pub gen: BoxGen<E>,
}

impl Conf {
    /// Specify which single generator to use in test.
    pub fn with_generator<E>(&self, gen: BoxGen<E>) -> ConfAndGen<E>
    where
        E: Clone,
    {
        ConfAndGen {
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

impl<E> ConfAndGen<E>
where
    E: Clone + 'static,
{
    fn new(conf: Conf, gen: BoxGen<E>) -> ConfAndGen<E> {
        ConfAndGen { gen, conf }
    }

    /// Check that the property holds for all generated example values.
    /// It returns a [`MonkeyResult`](MonkeyResult) to indicate success or
    /// failure.
    pub fn check_true<P>(&self, prop: P) -> MonkeyResult<E>
    where
        P: Fn(E) -> bool,
    {
        crate::runner::evaluate_property(self, prop)
    }

    /// Check that the property holds for all generated example values.
    /// It panics on failure.
    pub fn assert_true<P>(&self, prop: P) -> &ConfAndGen<E>
    where
        P: Fn(E) -> bool,
        E: std::fmt::Debug,
    {
        if let MonkeyResult::MonkeyErr {
            minimum_failure,
            seed,
            success_count,
            ..
        } = self.check_true(prop)
        {
            panic!(
                "Monkey test property failed!\n\
                Counterexample: {:?}\n\
                Reproduction seed: {}\n\
                Success count before failure: {}",
                minimum_failure, seed, success_count
            )
        }
        self
    }

    /// Add/change which shriker to use when a failing example is found.
    pub fn with_shrinker(&self, shrink: BoxShrink<E>) -> ConfAndGen<E> {
        ConfAndGen::new(self.conf.clone(), self.gen.with_shrinker(shrink))
    }
}
