//! The `shrink` module contains built in shrinkers.

mod bool;
pub mod fixed;
mod from_fn;
mod integer;
mod map;
mod no_shrink;
pub mod vec;
mod zip;

pub use bool::bool;
pub use bool::bool_to_true;
pub use from_fn::from_fn;
pub use from_fn::from_fn_boxed;
pub use integer::int_to_zero as int;
pub use map::map;
pub use no_shrink::none;
pub use zip::zip;
