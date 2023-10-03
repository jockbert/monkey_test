//! The `shrink` module contains built in shrinkers.

mod no_shrink;
mod num_shrink;
pub mod vec;

pub use no_shrink::NoShrink;
pub use num_shrink::NumShrink;

/// Shrink nothing
pub fn none<E: 'static>() -> NoShrink<E> {
    NoShrink::default()
}

/// Shrink number types to zero.
pub fn number() -> NumShrink {
    NumShrink {}
}
