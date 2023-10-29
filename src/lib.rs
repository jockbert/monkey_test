#![doc(
    issue_tracker_base_url = "https://github.com/jockbert/monkey_test/issues/"
)]
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

#[cfg(test)]
mod testing;

// Re-export details from config-module
pub use config::*;
use gen::chain::ChainGen;
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
pub trait Gen: Clone {
    /// The example type of the generator
    type Example: Clone;

    /// With generator associated shrinker.
    type Shrink: Shrink<Self::Example>;

    /// Produce a example iterator from the generator, given a randomization
    /// seed.
    fn examples(&self, seed: u64) -> SomeIter<Self::Example>;

    /// Returns a predefined shrinker, or a empty shrinker if no suitable
    /// exists.
    ///
    /// This enables distributing a default shrinker with given generator,
    /// reducing the need to explicitly configure a shrinker at place of use.
    ///
    /// When implementing a [Gen], you can return a empty [shrink::NoShrink]
    /// shrinker, if that makes the implementation easier, but when you will not
    /// get any shrinking functionality applied to failing example.
    fn shrinker(&self) -> Self::Shrink;

    /// Bind another shrinker to generator. See [gen::other_shrinker].
    fn with_shrinker<S2>(&self, shrink: S2) -> OtherShrinkGen<Self, S2>
    where
        S2: Shrink<Self::Example>,
    {
        gen::other_shrinker(self, shrink)
    }

    /// Concatenate together two generators
    ///
    /// ```
    /// use monkey_test::gen::fixed::sequence;
    /// use monkey_test::*;
    ///
    /// let a = sequence::<u32>(&[1, 2]);
    /// let b = sequence::<u32>(&[3, 4]);
    /// let c = a.chain(&b);
    /// let mut it = c.examples(77);
    ///
    /// assert_eq!(Some(1), it.next());
    /// assert_eq!(Some(2), it.next());
    /// assert_eq!(Some(3), it.next());
    /// assert_eq!(Some(4), it.next());
    /// assert_eq!(None, it.next());
    /// ```
    fn chain<G2>(&self, other_gen: &G2) -> ChainGen<Self, G2>
    where
        G2: Gen<Example = Self::Example>,
    {
        ChainGen::new(self, other_gen)
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
