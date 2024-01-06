//! The `gen` module contains built in generators.

use crate::BoxGen;
use crate::BoxShrink;

pub(crate) mod chain;
pub mod fixed;
mod integers;
mod other_shrink;
pub mod vec;

/// Create new generator with other shrinker
///
/// ```rust
/// use monkey_test::*;
///
/// let gen = gen::u8::any();
/// assert!(gen.shrinker().candidates(123).next().is_some());
///
/// // let generator have other shrinker
/// let gen2 = gen::other_shrinker(gen, shrink::none());
/// assert!(gen2.shrinker().candidates(123).next().is_none());
///
/// // let generator have other shrinker again (alternative way)
/// let gen3 = gen2.with_shrinker(shrink::number());
/// assert!(gen3.shrinker().candidates(123).next().is_some());
/// ```
pub fn other_shrinker<E: Clone + 'static>(
    gen: BoxGen<E>,
    other_shrink: BoxShrink<E>,
) -> BoxGen<E> {
    other_shrink::OtherShrinkGen::new(gen, other_shrink)
}

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
