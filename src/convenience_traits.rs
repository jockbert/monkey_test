//! Convenience traits for generator and shrinker combinators.
use crate::{BoxGen, BoxShrink, Gen, Shrink};

mod filter;
mod map;
mod zip;

pub use filter::*;
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
