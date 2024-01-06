//! The `shrink` module contains built in shrinkers.

mod no_shrink;
mod num_shrink;
pub mod vec;

pub use no_shrink::NoShrink;
pub use num_shrink::NumShrink;
use num_traits::Num;

use crate::BoxShrink;

/// Shrink nothing
pub fn none<E>() -> BoxShrink<E>
where
    E: Clone + 'static,
{
    Box::<NoShrink<E>>::default()
}

/// Shrink number types to zero.
pub fn number<E>() -> BoxShrink<E>
where
    E: Num + Copy + std::cmp::PartialOrd + 'static,
{
    Box::new(NumShrink {})
}
