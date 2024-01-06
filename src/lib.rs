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

/// Main entry point for writing property based tests using the monkey-test
/// tool.
///
/// # Example
/// ```should_panic
/// use monkey_test::*;
///
/// monkey_test()
///   .with_generator(gen::u8::any())
///   .assert_true(|x| x < 15);
/// ```
pub fn monkey_test() -> Conf {
    Conf::default()
}

/// A boxed iterator of example type `E`
pub type BoxIter<E> = Box<dyn Iterator<Item = E>>;

/// A boxed shrinker of example type `E`
pub type BoxShrink<E> = Box<dyn Shrink<E>>;

/// A boxed generator of example type `E`
pub type BoxGen<E> = Box<dyn Gen<E>>;

/// Trait that enables cloning a boxed generator.
#[doc(hidden)]
pub trait CloneGen<E> {
    fn clone_box(&self) -> BoxGen<E>;
}

impl<E: Clone + 'static, T> CloneGen<E> for T
where
    T: Gen<E> + Clone + 'static,
{
    fn clone_box(&self) -> BoxGen<E> {
        Box::new(self.clone())
    }
}

impl<E: Clone> Clone for BoxGen<E> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

/// Trait that enables cloning a boxed shrinker.
#[doc(hidden)]
pub trait CloneShrink<E> {
    fn clone_box(&self) -> BoxShrink<E>;
}

impl<E: Clone + 'static, T> CloneShrink<E> for T
where
    T: Shrink<E> + Clone + 'static,
{
    fn clone_box(&self) -> BoxShrink<E> {
        Box::new(self.clone())
    }
}

impl<E: Clone> Clone for BoxShrink<E> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

/// The generator trait, for producing example values to test in a property.
pub trait Gen<E: Clone + 'static>: CloneGen<E> {
    /// Produce a example iterator from the generator, given a randomization
    /// seed.
    fn examples(&self, seed: u64) -> BoxIter<E>;

    /// Returns a predefined shrinker, or a empty shrinker if no suitable
    /// exists.
    ///
    /// This enables distributing a default shrinker with given generator,
    /// reducing the need to explicitly configure a shrinker at place of use.
    ///
    /// When implementing a [Gen], you can return a empty [shrink::NoShrink]
    /// shrinker, if that makes the implementation easier, but when you will not
    /// get any shrinking functionality applied to failing example.
    fn shrinker(&self) -> BoxShrink<E>;

    /// Bind another shrinker to generator. See [gen::other_shrinker].
    fn with_shrinker(&self, other_shrink: BoxShrink<E>) -> BoxGen<E> {
        gen::other_shrinker(self.clone_box(), other_shrink)
    }

    /// Concatenate together two generators
    ///
    /// ```
    /// use monkey_test::gen::fixed::sequence;
    /// use monkey_test::*;
    ///
    /// let a = sequence::<u32>(&[1, 2]);
    /// let b = sequence::<u32>(&[3, 4]);
    /// let c = a.chain(b);
    /// let mut it = c.examples(77);
    ///
    /// assert_eq!(Some(1), it.next());
    /// assert_eq!(Some(2), it.next());
    /// assert_eq!(Some(3), it.next());
    /// assert_eq!(Some(4), it.next());
    /// assert_eq!(None, it.next());
    /// ```
    fn chain(&self, other_gen: BoxGen<E>) -> BoxGen<E> {
        Box::new(ChainGen::new(self.clone_box(), other_gen))
    }
}

/// The shrinker trait, for shrinking a failed example values into smaller ones.
/// What is determined as a smaller value can be subjective and is up to author
/// or tester to determine, but as a rule of thumb a smaller value should be
/// easier to interpret, when a property is proven wrong.
pub trait Shrink<E>: CloneShrink<E>
where
    E: Clone,
{
    /// Returns a series of smaller examples, given an original example.
    fn candidates(&self, original: E) -> BoxIter<E>;
}

// Doctest the readme file
#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
