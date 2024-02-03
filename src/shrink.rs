//! The `shrink` module contains built in shrinkers.

mod no_shrink;
mod num_shrink;
pub mod vec;
mod zip;

pub use no_shrink::none;
pub use num_shrink::to_zero as number;
pub use zip::zip;
