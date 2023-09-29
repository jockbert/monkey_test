#![doc(issue_tracker_base_url = "https://github.com/jockbert/monkey_test/issues/")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/jockbert/monkey_test/main/assets/doc/logo-256.png"
)]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/jockbert/monkey_test/main/assets/doc/logo.ico"
)]
#![warn(missing_docs)]

//! A [property based testing (*PBT*)](https://en.wikipedia.org/wiki/Software_testing#Property_testing) tool like QuickCheck
//! [(Wikipedia)](https://en.wikipedia.org/wiki/QuickCheck)
//! [(github)](https://github.com/nick8325/quickcheck) and other deriviatives thereof.
//!
//! # PBT core concepts
//!
//! PBT is a complement to normal unit testing.
//!
//! A normal unit test uses a singel specific example input and verifies it
//! against a specific outcome.
//! ```
//! assert_eq!(1_f64.sqrt(), 1_f64);
//! assert_eq!(4_f64.sqrt(), 2_f64);
//! assert_eq!(9_f64.sqrt(), 3_f64);
//! assert_eq!(16_f64.sqrt(), 4_f64);
//! ```
//!
//! With PBT a property of your code is validated against an arbitrary number of
//! generated examples.
//! A propery is saying something more general about your code than a specific
//! example and outcome.
//! You often loose some specificity but can say something more general about
//! the code under test.
//! Further, using random examples in test can find aspects you missed when
//! manually choosing examples to test.
//!
//! ```
//! let randomExampleLargerThanOne: f64 = 9.0;
//!
//! assert!(randomExampleLargerThanOne.sqrt() < randomExampleLargerThanOne);
//! assert!(randomExampleLargerThanOne.sqrt() > 1.0);
//! ```
//!
//! So, what is the point of having the loose boundaries in the `sqrt`-properties
//! tested above? Is there any value in testing these properties?
//!
//! The answer is that usually when code goes wrong or has a bug, the
//! return value is not just a little bit off, but many times it is way off and
//! fail spectacularly, like returning a negative value or panicing.
//!
//! In short, combining general property based tests with some very specific
//! unit tests is a powerful testing technique.
//!
//! # Nomenclature
//! - *Generator*  - A source of random examples
//! - *Property* - Your parameterized test
//! - *Shrinker* - Generate smaller examples based on failing example, in order to simplify the failure
//!
//! # Some common classes of properties to use
//! How do you write a useful property that is valid for all generated examples?
//! One baby step is to try parameterize an already existing example based test.
//! As an inspiration, here follow some common classes of properties to test.
//!
//! ## No explosion
//! Just shoot examples at code under test and make sure there are no errors and
//! no panics.
//!
//! ## Simplification
//!
//!
//! ## Symmetry
//! Apply a function and its inverse and make sure you get back the same initial
//! value. Some examples of this is write and read back something, like save to
//! file system and load it again.
//! Another example is to generate some ground truth data, transform it to an
//! input format and make sure your business logic calculate an answer that
//! corresponds to the ground truth data.
//!
//! ## Idempotens
//! Applying the same function many times generate the same result.
//! ```
//! let example: i64 = -4;
//!
//! assert_eq!(
//!   example.abs(),
//!   example.abs().abs().abs());
//! ```
//!
//! ## Oracle
//! Compare the function result against other trusted means to get
//! the same result. Perhaps compare output to a model of one aspect,
//! analogous function, other existing implementation, unoptimized code or
//! legacy code. Perhaps compare the output of old and new code to enable
//! reckless refactoring.
//! ```
//! let example = -4;
//!
//! // Example of analogous function to get to the same result
//! let testedMethod = example + example;
//! let analogMethod = example * 2;
//! assert_eq!(testedMethod, analogMethod);
//! ```
//!
//! ## Induction
//! Show that some property holds for `P(0)` and that `P(n) + C = P(n+1)`.
//!
//! ```
//! let mut example = vec![-4,7,5,2];
//!
//! // Induction base case
//! let empty_vec: Vec<i32> = vec![];
//! assert_eq!{empty_vec.len(), 0};
//!
//! // Induction step
//! let len_original = example.len();
//! example.push(8);
//! let len_after_push = example.len();
//! assert_eq!(len_original + 1, len_after_push);
//! ```
//! ## Stateful testing
//! As one example, execute a series of commands against a stateful system, to
//! then verify some property of the system.
//!
//!
//! # Key design principles of the monkey test tool.
//! - *configurability and flexibility* - Leave a high degree of configurability
//!   and flexibility to the user by letting most details to be specified
//!   programatically. The aim is to have an declarative builder-style API like
//!   the Java library
//!   QuickTheories [(github)](https://github.com/quicktheories/QuickTheories).
//!
//! - *powerful shinking* - Good shrinkers is a really important aspect of a
//!   property based testing tool. Let say that the failing example is a vector
//!   of 1000 elements and only 3 of the elements in combination is the actual
//!   failure cause. You are then unlikely to find the 3-element combination,
//!   if the shrinking is not powerful enough.
//!
//! - *composability for complex test examples* - Basic type generators and
//!   shrinkers are provided out of the box.
//!   User should also be able to genereate and shrink more complex types, by
//!   composing together more primitive generators and shrinkers into more
//!   complex ones.
//!   The main inspiration here is the Scala library ScalaCheck
//!   [(homepage)](https://scalacheck.org/),
//!   which is fenomenal in this aspect, having the power to for example easily
//!   generate and shrink recursive data structures, by using composition.
//!
//! - *no macro magic* - In order to keep the tool simple, just avoid macros if
//!   same developer experience can be provided using normal Rust code.
//!   Macros-use is an complex escape hatch only to be used when normal syntax
//!   is insufficient.
//!

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
