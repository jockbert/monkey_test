#![doc(issue_tracker_base_url = "https://github.com/jockbert/monkey_test/issues/")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/jockbert/monkey_test/main/assets/doc/logo-256.png"
)]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/jockbert/monkey_test/main/assets/doc/logo.ico"
)]
#![warn(missing_docs)]
#![doc = include_str!("../DOCUMENTATION.md")]

mod config;
pub mod gen;
mod runner;
pub mod shrink;

// Re-export details from config-module
pub use config::*;
use gen::OtherShrinkGen;

/// Main entry point for writing property based tests using the monkey-test
/// tool.
///
/// # Example
/// ```should_panic
/// use monkey_test::*;
///
/// monkey_test()
///   .with_generator(gen::u8::any())
///   .assert_true(|x: u8| x < 15)
/// ```
pub fn monkey_test() -> Conf {
    Conf::default()
}

type SomeIter<E> = Box<dyn Iterator<Item = E>>;

/// The generator trait, for producing example values to test in a property.
pub trait Gen<E, S>: Clone
where
    E: Clone,
    S: Shrink<E>,
{
    /// Produce a example iterator from the generator, given a randomization
    /// seed.
    fn examples(&self, seed: u64) -> SomeIter<E>;

    /// Returns a predefined shrinker, or a empty shrinker if no suitable exists.
    ///
    /// This enables distributing a default shrinker with given generator,
    /// reducing the need to explicitly configure a shrinker at place of use.
    ///
    /// When implementing a [Gen], you can return a empty [shrink::NoShrink]
    /// shrinker, if that makes the implementation easier, but when you will not
    /// get any shrinking functionality applied to failing example.
    fn shrinker(&self) -> S;

    /// Bind another shrinker to generator.
    fn with_shrinker<S2>(&self, shrink: S2) -> OtherShrinkGen<E, Self, S, S2>
    where
        S2: Shrink<E>,
    {
        OtherShrinkGen::new(self, shrink)
    }
}

/// The shrinker trait, for shrinking a failed example values into smaller ones.
/// What is determined as a smaller value can be subjective and is up to author
/// or tester to determine, but as a rule of thumb a smaller value should be
/// easier to interpret, when a property is proven wrong.
pub trait Shrink<E>: Clone {
    /// Returns a series of smaller examples, given an original example.
    fn candidates(&self, original: E) -> SomeIter<E>;
}

// Doctest the readme file
#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
