//! Convenience traits for generator and shrinker combinators.
use crate::{gens, shrinks, BoxGen, BoxShrink, Gen, Shrink};

mod map;
mod zip;

pub use map::*;
pub use zip::*;

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

/// Not dyn compatible (a.k.a. object safe) trait for providing example
/// filtering in generator.
pub trait FilterWithGen<E>
where
    E: Clone + 'static,
{
    /// See [gens::filter].
    fn filter<P>(&self, predicate: P) -> BoxGen<E>
    where
        P: Fn(&E) -> bool + Clone + 'static;
}

impl<E: Clone + 'static> FilterWithGen<E> for dyn Gen<E> {
    fn filter<P>(&self, predicate: P) -> BoxGen<E>
    where
        P: Fn(&E) -> bool + Clone + 'static,
    {
        gens::filter(self.clone_box(), predicate)
    }
}

/// Not dyn compatible (a.k.a. object safe) trait for providing shrinker
/// filtering.
pub trait FilterWithShrink<E>
where
    E: Clone + 'static,
{
    /// See [shrinks::filter].
    fn filter<P>(&self, predicate: P) -> BoxShrink<E>
    where
        P: Fn(&E) -> bool + Clone + 'static;
}

impl<E: Clone + 'static> FilterWithShrink<E> for dyn Shrink<E> {
    fn filter<P>(&self, predicate: P) -> BoxShrink<E>
    where
        P: Fn(&E) -> bool + Clone + 'static,
    {
        shrinks::filter(self.clone_box(), predicate)
    }
}
