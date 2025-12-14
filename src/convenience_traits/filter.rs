//! Convenience traits for generator and shrinker combinators for filtering.

use crate::{gens, shrinks, BoxGen, BoxShrink, Gen, Shrink};

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
