//! The `shrinks` module contains built in shrinkers.

mod bool;
mod filter;
pub mod fixed;
mod float;
mod from_fn;
mod integer;
mod map;
mod no_shrink;
pub mod vec;
mod zip;

pub use bool::bool;
pub use bool::bool_to_true;
pub use filter::filter;
pub use float::float;
pub use from_fn::from_fn;
pub use from_fn::from_fn_boxed;

#[allow(deprecated)]
pub use integer::int;
pub use integer::int_in_range;
pub use integer::int_to_zero;

pub use map::map;
pub use no_shrink::none;
pub use zip::zip;
