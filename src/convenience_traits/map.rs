//! Convenience traits for generator and shrinker combinators for mapping.

use crate::{gens, shrinks, BoxGen, BoxShrink, Gen, Shrink};

/// Not dyn compatible (a.k.a. object safe) trait for providing generator
/// mapping.
pub trait MapWithGen<E0>
where
    E0: Clone + 'static,
{
    /// See [gens::map].
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
        gens::map(self.clone_box(), map_fn, unmap_fn)
    }
}

/// Not dyn compatible (a.k.a. object safe) trait for providing shrinker
/// mapping.
pub trait MapWithShrink<E0>
where
    E0: Clone + 'static,
{
    /// See [shrinks::map].
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
        shrinks::map(self.clone_box(), map_fn, unmap_fn)
    }
}
