use crate::gen::OtherShrinkGen;
pub use crate::runner::MonkeyResult;
use crate::Gen;
use crate::Shrink;

/// Configuration for executing monkey tests.
#[derive(Clone)]
pub struct Conf {
    /// See [Conf::with_seed].
    pub example_count: u32,
    /// See [Conf::with_seed].
    pub seed: u64,
}

/// Configuration for executing monkey tests, including the generator.
pub struct ConfAndGen<G>
where
    G: Gen,
{
    /// The configuration to use.
    pub conf: Conf,
    /// See [Conf::with_generator].
    pub gen: G,
}

impl Conf {
    /// Specify which single generator to use in test.
    pub fn with_generator<G>(&self, gen: G) -> ConfAndGen<G>
    where
        G: Gen,
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

impl<G> ConfAndGen<G>
where
    G: Gen,
{
    fn new(conf: Conf, gen: G) -> ConfAndGen<G> {
        ConfAndGen { gen, conf }
    }

    /// Check that the property holds for all generated example values.
    /// It returns a [`MonkeyResult`](MonkeyResult) to indicate success or
    /// failure.
    pub fn check_true<P>(&self, prop: P) -> MonkeyResult<G::Example>
    where
        P: Fn(G::Example) -> bool,
    {
        crate::runner::evaluate_property(self, prop)
    }

    /// Check that the property holds for all generated example values.
    /// It panics on failure.
    pub fn assert_true<P>(&self, prop: P) -> &ConfAndGen<G>
    where
        P: Fn(G::Example) -> bool,
        G::Example: std::fmt::Debug,
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
        self
    }

    /// Add/change which shriker to use if a failing example is found.
    pub fn with_shrinker<S2>(
        &self,
        shrink: S2,
    ) -> ConfAndGen<OtherShrinkGen<G, S2>>
    where
        S2: Shrink<G::Example>,
    {
        ConfAndGen::new(self.conf.clone(), self.gen.with_shrinker(shrink))
    }
}
