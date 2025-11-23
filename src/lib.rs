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
mod convenience_traits;
pub mod gens;
mod internal;
mod runner;
pub mod shrinks;

#[cfg(test)]
mod testing;

use std::ops::RangeInclusive;

// Re-export details from some modules for easier access.
pub use config::*;
pub use convenience_traits::*;

/// Main entry point for writing property based tests using the monkey-test
/// tool.
///
/// # Example
/// ```should_panic
/// use monkey_test::*;
///
/// monkey_test()
///   .with_generator(gens::u8::any())
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

/// A property is something that should hold, for all given examples.
pub type Property<E> = fn(E) -> bool;

/// Type alias for size range used when generating examples. It is an inclusive
/// range so it can encompass all values including usize::MAX.
pub type ExampleSize = RangeInclusive<usize>;

/// Type alias for a randomization seed used when generating random examples in
/// a generator.
pub type Seed = u64;

impl<E> core::fmt::Debug for dyn Gen<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(std::any::type_name::<Self>())
    }
}

/// The generator trait, for producing example values to test in a property.
pub trait Gen<E: Clone + 'static>: CloneGen<E> {
    /// Produce a example iterator from the generator, given a randomization
    /// seed and (if applicable) size of examples to produce.
    fn examples(&self, seed: Seed, size: ExampleSize) -> BoxIter<E>;

    /// Returns a predefined shrinker, or a empty shrinker if no suitable
    /// exists.
    ///
    /// This enables distributing a default shrinker with given generator,
    /// reducing the need to explicitly configure a shrinker at place of use.
    ///
    /// When implementing a [Gen], you can return a empty [shrinks::none]
    /// shrinker, if that makes the implementation easier, but when you will not
    /// get any shrinking functionality applied to failing example.
    fn shrinker(&self) -> BoxShrink<E>;

    /// Bind another shrinker to generator. See [gens::other_shrinker].
    fn with_shrinker(&self, other_shrink: BoxShrink<E>) -> BoxGen<E> {
        gens::other_shrinker(self.clone_box(), other_shrink)
    }

    /// Concatenate together two generators. See [gens::chain].
    fn chain(&self, other_gen: BoxGen<E>) -> BoxGen<E> {
        gens::chain(self.clone_box(), other_gen)
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
