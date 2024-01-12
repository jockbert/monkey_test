//! The `gen` module contains built in generators.

mod chain;
pub mod fixed;
mod integers;
mod other_shrink;
mod pick;
mod sample_target;
pub mod vec;

pub use chain::chain;
pub use other_shrink::other_shrinker;
pub use pick::pick_evenly;
pub use pick::pick_with_ratio;
pub use sample_target::Ratio;
use sample_target::SampleTarget;

/// Macro to generate code for all integer type modules
macro_rules! integer_module {
    ($name:ident) => {
        /// Generators for values of module type.
        pub mod $name {
            use crate::BoxGen;
            use std::ops::RangeBounds;

            /// Uniformly distributed unbound range of value
            pub fn any() -> BoxGen<$name> {
                ranged(..)
            }

            /// Uniformly distributed limited range of values
            pub fn ranged<B>(bounds: B) -> BoxGen<$name>
            where
                B: RangeBounds<$name>,
            {
                super::integers::ranged(bounds)
            }
        }
    };
}

integer_module!(i8);
integer_module!(i16);
integer_module!(i32);
integer_module!(i64);
integer_module!(i128);
integer_module!(isize);
integer_module!(u8);
integer_module!(u16);
integer_module!(u32);
integer_module!(u64);
integer_module!(u128);
integer_module!(usize);
