//! The `gen` module contains built in generators.

pub mod bool;
mod chain;
pub mod fixed;
mod integers;
mod map;
mod mix;
mod other_shrink;
mod pick;
mod sample_target;
pub mod sized;
pub mod vec;
mod zip;

pub use chain::chain;
pub use map::map;
pub use mix::mix_evenly;
pub use mix::mix_with_ratio;
pub use other_shrink::other_shrinker;
pub use pick::pick_evenly;
pub use pick::pick_with_ratio;
pub use sample_target::Ratio;
use sample_target::SampleTarget;
pub use zip::zip;

/// Macro to generate code for all integer type modules
macro_rules! integer_module {
    ($name:ident) => {
        /// Generators for values of module type.
        pub mod $name {
            use crate::BoxGen;
            use std::ops::RangeBounds;

            /// Roughly uniformly distributed unbound range of values, with
            /// some overwheight to extremes (min and max).
            pub fn any() -> BoxGen<$name> {
                ranged(..)
            }

            /// Roughly uniformly distributed range of values, with some
            /// overwheight to extremes (min and max) of given bounds.
            pub fn ranged<B>(bounds: B) -> BoxGen<$name>
            where
                B: RangeBounds<$name>,
            {
                super::integers::ranged(bounds)
            }

            /// Int generator with completely random distribution. This
            /// function has a long name, since `ranged` should be preferred.
            pub fn completely_random<B>(bounds: B) -> BoxGen<$name>
            where
                B: RangeBounds<$name>,
            {
                super::integers::completely_random(bounds)
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
