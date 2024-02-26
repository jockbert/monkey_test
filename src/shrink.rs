//! The `shrink` module contains built in shrinkers.

mod bool;
pub mod fixed;
mod map;
mod no_shrink;
mod num_shrink;
pub mod vec;
mod zip;

pub use bool::bool;
pub use bool::bool_to_true;
pub use map::map;
pub use no_shrink::none;
pub use num_shrink::to_zero as number;
pub use zip::zip;
