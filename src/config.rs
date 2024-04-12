pub use crate::runner::MonkeyResult;
use crate::BoxGen;
use crate::BoxShrink;
use crate::Property;
use rand::RngCore;
use rand::SeedableRng;
use std::fmt::Write;
use std::sync::mpsc;

/// Configuration for executing monkey tests.
#[derive(Clone)]
pub struct Conf {
    /// See [Conf::with_seed].
    pub example_count: u32,
    /// See [Conf::with_seed].
    pub seed: u64,
}

/// Configuration for executing monkey tests, including the generator.
#[derive(Clone)]
pub struct ConfAndGen<E>
where
    E: Clone,
{
    /// The configuration to use.
    pub conf: Conf,
    /// See [Conf::with_generator].
    pub gen: BoxGen<E>,
    /// See [ConfAndGen::title].
    pub title: Option<String>,
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
            title: None,
        }
    }

    /// Specify the number of examples to use in test. If not specified, the
    /// default number of examples are used. If the default number of examples
    /// are explicitly changed, it is set to 100.
    pub fn with_example_count(&self, example_count: u32) -> Conf {
        Self {
            example_count,
            seed: self.seed,
        }
    }

    /// Specify which seed to use for random values. Specifying the seed is
    /// useful for reproducing a failing test run. Use this with caution, since
    /// using a seed hinders new test runs to use other examples than already
    /// used in earlier test runs.
    pub fn with_seed(&self, seed: u64) -> Conf {
        Self {
            example_count: self.example_count,
            seed,
        }
    }
}

/// The standard source to get randimization seed from.
pub fn seed_to_use() -> u64 {
    rand_chacha::ChaCha8Rng::from_entropy().next_u64()
}

impl Default for Conf {
    /// Create new configuration with default values
    fn default() -> Self {
        Self {
            example_count: 100,
            seed: seed_to_use(),
        }
    }
}

impl<E> ConfAndGen<E>
where
    E: Clone + 'static,
{
    /// Check that the property returns true for all generated example values.
    /// It returns a [`MonkeyResult`](MonkeyResult) to indicate success or
    /// failure.
    pub fn test_true(&self, prop: Property<E>) -> MonkeyResult<E> {
        crate::runner::evaluate_property(self, |example: E| {
            if prop(example) {
                Ok(())
            } else {
                Err("Expecting 'true' but got 'false'.".into())
            }
        })
    }

    /// This function is deprecated, due to name change, aligning names of
    /// different asserts and tests. Use [ConfAndGen::test_true] instead.
    #[deprecated = "Use ConfAndGen.test_true instead"]
    pub fn test_property(&self, prop: Property<E>) -> MonkeyResult<E> {
        self.test_true(prop)
    }

    /// Check that the property holds for all generated example values.
    /// It panics on failure.
    #[track_caller]
    pub fn assert_true(&self, prop: Property<E>) -> &ConfAndGen<E>
    where
        E: std::fmt::Debug,
    {
        panic_on_err(self.test_true(prop));
        self
    }

    /// Check that the property do not panic for any generated example values.
    /// It panics on failure.
    #[track_caller]
    pub fn assert_no_panic(&self, prop: fn(E) -> ()) -> &ConfAndGen<E>
    where
        E: std::fmt::Debug + std::panic::UnwindSafe,
    {
        panic_on_err(crate::runner::evaluate_property(self, |example| {
            test_no_panic(prop, example)
        }));
        self
    }

    /// Add/change which shriker to use when a failing example is found.
    pub fn with_shrinker(&self, shrink: BoxShrink<E>) -> ConfAndGen<E> {
        Self {
            gen: self.gen.with_shrinker(shrink),
            ..self.clone()
        }
    }

    /// Add or change title of all following asserts. The title is used for
    /// naming the failed property assert. The title is used on all following
    /// properties, until other title is set.
    pub fn title(&self, title: &str) -> ConfAndGen<E> {
        Self {
            title: Some(title.to_string()),
            ..self.clone()
        }
    }
}

fn panic_on_err<E>(result: MonkeyResult<E>)
where
    E: std::fmt::Debug,
{
    if let MonkeyResult::MonkeyErr {
        minimum_failure,
        seed,
        success_count,
        title,
        reason,
        some_other_failures,
        original_failure,
        ..
    } = result
    {
        let first_line = match title {
            Some(t) => format!("Monkey test property \"{t}\" failed!"),
            None => "Monkey test property failed!".into(),
        };

        let other_failures_text: String = some_other_failures.iter().fold(
            String::new(),
            |mut output, failure| {
                let _ = write!(output, "\n\t{failure:?}");
                output
            },
        );

        panic!(
            "{first_line}\n\
            Failure: {minimum_failure:?}\n\
            Reason: {reason}\n\
            \n\
            Reproduction seed: {seed}\n\
            Success count before failure: {success_count}\n\
            Other failures:\n\t{original_failure:?}{other_failures_text}\n",
        )
    }
}

fn test_no_panic<E>(prop: fn(E), example: E) -> Result<(), String>
where
    E: std::fmt::Debug + std::panic::UnwindSafe + Clone + 'static,
{
    let original_panic_hook = std::panic::take_hook();
    let (tx, rx) = mpsc::channel();

    // Set temporary panic hook catching original panic loaction and
    // muting stacktrace printouts to stdout. If not muted, there can be a panic
    // stacktrace for each failing example while shrinking a failure.
    std::panic::set_hook(Box::new(move |info| {
        if let Some(loc) = info.location() {
            let location_text =
                format! {"in file '{}' at line {}", loc.file(), loc.line()};
            tx.send(location_text).unwrap();
        }
    }));

    // Do a test with a single example
    let maybe_panic = std::panic::catch_unwind(|| prop(example));

    // Restore old original panic hook
    std::panic::set_hook(original_panic_hook);

    maybe_panic.map_err(|panic| {
        let message = panic_message::get_panic_message(&panic).unwrap_or("<?>");
        let location = rx.try_recv().unwrap_or("at unknown location".into());
        format! {"Expecting no panic, but got panic {message:?} {location}." }
    })
}
