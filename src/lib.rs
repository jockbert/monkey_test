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

/// A property is something that should hold, for all given examples.
pub type Property<E> = fn(E) -> bool;

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

impl<E> core::fmt::Debug for dyn Gen<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(std::any::type_name::<Self>())
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
    /// When implementing a [Gen], you can return a empty [shrink::none]
    /// shrinker, if that makes the implementation easier, but when you will not
    /// get any shrinking functionality applied to failing example.
    fn shrinker(&self) -> BoxShrink<E>;

    /// Bind another shrinker to generator. See [gen::other_shrinker].
    fn with_shrinker(&self, other_shrink: BoxShrink<E>) -> BoxGen<E> {
        gen::other_shrinker(self.clone_box(), other_shrink)
    }

    /// Concatenate together two generators. See [gen::chain].
    fn chain(&self, other_gen: BoxGen<E>) -> BoxGen<E> {
        gen::chain(self.clone_box(), other_gen)
    }
}

/// Non-object-safe trait for providing generator zipping.
///
/// The [ZipWithGen::zip] extension method cannot be implemented diectly on
/// [Gen] object trait, since generic method in respect to other type `E1`, does
/// not seem to be allowed on trait objects.
pub trait ZipWithGen<E0>
where
    E0: Clone + 'static,
{
    /// See [gen::zip].
    fn zip<E1>(&self, other_gen: BoxGen<E1>) -> BoxGen<(E0, E1)>
    where
        E1: Clone + 'static;

    /// Zip together 3 generators.
    fn zip_3<E1, E2>(
        &self,
        gen1: BoxGen<E1>,
        gen2: BoxGen<E2>,
    ) -> BoxGen<(E0, E1, E2)>
    where
        E1: Clone + 'static,
        E2: Clone + 'static;

    /// Zip together 4 generators.
    fn zip_4<E1, E2, E3>(
        &self,
        gen1: BoxGen<E1>,
        gen2: BoxGen<E2>,
        gen3: BoxGen<E3>,
    ) -> BoxGen<(E0, E1, E2, E3)>
    where
        E1: Clone + 'static,
        E2: Clone + 'static,
        E3: Clone + 'static;

    /// Zip together 5 generators.
    fn zip_5<E1, E2, E3, E4>(
        &self,
        gen1: BoxGen<E1>,
        gen2: BoxGen<E2>,
        gen3: BoxGen<E3>,
        gen4: BoxGen<E4>,
    ) -> BoxGen<(E0, E1, E2, E3, E4)>
    where
        E1: Clone + 'static,
        E2: Clone + 'static,
        E3: Clone + 'static,
        E4: Clone + 'static;

    /// Zip together 6 generators.
    fn zip_6<E1, E2, E3, E4, E5>(
        &self,
        gen1: BoxGen<E1>,
        gen2: BoxGen<E2>,
        gen3: BoxGen<E3>,
        gen4: BoxGen<E4>,
        gen5: BoxGen<E5>,
    ) -> BoxGen<(E0, E1, E2, E3, E4, E5)>
    where
        E1: Clone + 'static,
        E2: Clone + 'static,
        E3: Clone + 'static,
        E4: Clone + 'static,
        E5: Clone + 'static;
}

impl<E0: Clone + 'static> ZipWithGen<E0> for dyn Gen<E0> {
    fn zip<E1>(&self, other_gen: BoxGen<E1>) -> BoxGen<(E0, E1)>
    where
        E1: Clone + 'static,
    {
        gen::zip(self.clone_box(), other_gen)
    }

    fn zip_3<E1, E2>(
        &self,
        gen1: BoxGen<E1>,
        gen2: BoxGen<E2>,
    ) -> BoxGen<(E0, E1, E2)>
    where
        E1: Clone + 'static,
        E2: Clone + 'static,
    {
        gen::zip(gen::zip(self.clone_box(), gen1), gen2)
            .map(|((e0, e1), e2)| (e0, e1, e2), |(e0, e1, e2)| ((e0, e1), e2))
    }

    fn zip_4<E1, E2, E3>(
        &self,
        gen1: BoxGen<E1>,
        gen2: BoxGen<E2>,
        gen3: BoxGen<E3>,
    ) -> BoxGen<(E0, E1, E2, E3)>
    where
        E1: Clone + 'static,
        E2: Clone + 'static,
        E3: Clone + 'static,
    {
        gen::zip(gen::zip(self.clone_box(), gen1), gen::zip(gen2, gen3)).map(
            |((e0, e1), (e2, e3))| (e0, e1, e2, e3),
            |(e0, e1, e2, e3)| ((e0, e1), (e2, e3)),
        )
    }

    fn zip_5<E1, E2, E3, E4>(
        &self,
        gen1: BoxGen<E1>,
        gen2: BoxGen<E2>,
        gen3: BoxGen<E3>,
        gen4: BoxGen<E4>,
    ) -> BoxGen<(E0, E1, E2, E3, E4)>
    where
        E1: Clone + 'static,
        E2: Clone + 'static,
        E3: Clone + 'static,
        E4: Clone + 'static,
    {
        gen::zip(gen::zip(self.clone_box(), gen1), gen2.zip_3(gen3, gen4)).map(
            |((e0, e1), (e2, e3, e4))| (e0, e1, e2, e3, e4),
            |(e0, e1, e2, e3, e4)| ((e0, e1), (e2, e3, e4)),
        )
    }

    fn zip_6<E1, E2, E3, E4, E5>(
        &self,
        gen1: BoxGen<E1>,
        gen2: BoxGen<E2>,
        gen3: BoxGen<E3>,
        gen4: BoxGen<E4>,
        gen5: BoxGen<E5>,
    ) -> BoxGen<(E0, E1, E2, E3, E4, E5)>
    where
        E1: Clone + 'static,
        E2: Clone + 'static,
        E3: Clone + 'static,
        E4: Clone + 'static,
        E5: Clone + 'static,
    {
        gen::zip(self.zip_3(gen1, gen2), gen3.zip_3(gen4, gen5)).map(
            |((e0, e1, e2), (e3, e4, e5))| (e0, e1, e2, e3, e4, e5),
            |(e0, e1, e2, e3, e4, e5)| ((e0, e1, e2), (e3, e4, e5)),
        )
    }
}

/// Non-object-safe trait for providing generator mapping.
pub trait MapWithGen<E0>
where
    E0: Clone + 'static,
{
    /// See [gen::map].
    fn map<E1>(
        &self,
        map_fn: fn(E0) -> E1,
        unmap_fn: fn(E1) -> E0,
    ) -> BoxGen<E1>
    where
        E1: Clone + 'static;
}

impl<E0: Clone + 'static> MapWithGen<E0> for dyn Gen<E0> {
    fn map<E1>(
        &self,
        map_fn: fn(E0) -> E1,
        unmap_fn: fn(E1) -> E0,
    ) -> BoxGen<E1>
    where
        E1: Clone + 'static,
    {
        gen::map(self.clone_box(), map_fn, unmap_fn)
    }
}

/// Non-object-safe trait for providing example filtering in generator.
pub trait FilterWithGen<E>
where
    E: Clone + 'static,
{
    /// See [gen::filter].
    fn filter<P>(&self, predicate: P) -> BoxGen<E>
    where
        P: Fn(&E) -> bool + Clone + 'static;
}

impl<E: Clone + 'static> FilterWithGen<E> for dyn Gen<E> {
    fn filter<P>(&self, predicate: P) -> BoxGen<E>
    where
        P: Fn(&E) -> bool + Clone + 'static,
    {
        gen::filter(self.clone_box(), predicate)
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

/// Non-object-safe trait for providing shrinker zipping.
pub trait ZipWithShrink<E0>
where
    E0: Clone + 'static,
{
    /// See [shrink::zip].
    fn zip<E1>(&self, other_shrink: BoxShrink<E1>) -> BoxShrink<(E0, E1)>
    where
        E1: Clone + 'static;
}

impl<E0: Clone + 'static> ZipWithShrink<E0> for dyn Shrink<E0> {
    fn zip<E1>(&self, other_gen: BoxShrink<E1>) -> BoxShrink<(E0, E1)>
    where
        E1: Clone + 'static,
    {
        shrink::zip(self.clone_box(), other_gen)
    }
}

/// Non-object-safe trait for providing shrinker mapping.
pub trait MapWithShrink<E0>
where
    E0: Clone + 'static,
{
    /// See [shrink::map].
    fn map<E1>(
        &self,
        map_fn: fn(E0) -> E1,
        unmap_fn: fn(E1) -> E0,
    ) -> BoxShrink<E1>
    where
        E1: Clone + 'static;
}

impl<E0: Clone + 'static> MapWithShrink<E0> for dyn Shrink<E0> {
    fn map<E1>(
        &self,
        map_fn: fn(E0) -> E1,
        unmap_fn: fn(E1) -> E0,
    ) -> BoxShrink<E1>
    where
        E1: Clone + 'static,
    {
        shrink::map(self.clone_box(), map_fn, unmap_fn)
    }
}

/// Non-object-safe trait for providing shrinker filtering.
pub trait FilterWithShrink<E>
where
    E: Clone + 'static,
{
    /// See [shrink::filter].
    fn filter<P>(&self, predicate: P) -> BoxShrink<E>
    where
        P: Fn(&E) -> bool + Clone + 'static;
}

impl<E: Clone + 'static> FilterWithShrink<E> for dyn Shrink<E> {
    fn filter<P>(&self, predicate: P) -> BoxShrink<E>
    where
        P: Fn(&E) -> bool + Clone + 'static,
    {
        shrink::filter(self.clone_box(), predicate)
    }
}

// Doctest the readme file
#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
