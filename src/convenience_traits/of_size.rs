//! Convenience trait for setting specific size range on generator.

use std::ops::RangeBounds;

use crate::{gens, BoxGen, Gen};

/// Convenience trait that constrains the example sizes to be within the
/// given range.
///
/// This trait is not a dyn compatible (a.k.a. object safe) trait.
pub trait SizedGen<E>
where
    E: Clone + 'static,
{
    /// See [gens::of_size].
    fn of_size<R>(&self, range: R) -> BoxGen<E>
    where
        R: RangeBounds<usize> + Clone + 'static;
}

impl<E: Clone + 'static> SizedGen<E> for dyn Gen<E> {
    fn of_size<R>(&self, range: R) -> BoxGen<E>
    where
        R: RangeBounds<usize> + Clone + 'static,
    {
        gens::of_size(self.clone_box(), range)
    }
}
