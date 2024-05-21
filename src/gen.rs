//! The `gen` module contains built in generators.

pub mod bool;
mod chain;
mod filter;
pub mod fixed;
mod float;
mod float_parts;
mod from_fn;
mod integer;
mod map;
mod mix;
mod other_shrink;
mod pick;
mod sample_target;
pub mod sized;
pub mod vec;
mod zip;

use crate::BoxGen;
pub use chain::chain;
pub use filter::filter;
pub use from_fn::from_fn;
pub use from_fn::from_fn_boxed;
pub use map::map;
pub use mix::mix_evenly;
pub use mix::mix_with_ratio;
pub use other_shrink::other_shrinker;
pub use pick::pick_evenly;
pub use pick::pick_with_ratio;
pub use sample_target::Ratio;
use sample_target::SampleTarget;
pub use zip::zip;

/// Standard way to generate seeds for random source.
pub fn seeds() -> BoxGen<u64> {
    crate::gen::u64::completely_random(..)
}

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
                super::integer::ranged(bounds)
            }

            /// Int generator with completely random distribution. This
            /// function has a long name, since `ranged` should be preferred.
            pub fn completely_random<B>(bounds: B) -> BoxGen<$name>
            where
                B: RangeBounds<$name>,
            {
                super::integer::completely_random(bounds)
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

/// Macro to generate code for all float type modules
macro_rules! float_module {
    ($name:ident) => {
        pub mod $name {
            //! Module with generators for floating point values. There are both
            //! modules [crate::gen::f64] and [crate::gen::f32] for each type
            //! respecive.
            //!
            //! There are some variants for generating floating point numbers. For
            //! truly  making sure your code works with any float value, use [any].
            //!
            //! Generator | -inf | negative finites | positive finites | inf | NaN
            //! ----------|:----:|:----------------:|:----------------:|:---:|:----
            //! [any]     |  ✓  |   ✓              |       ✓          |  ✓  |  ✓
            //! [number]  |  ✓  |   ✓              |       ✓          |  ✓  |
            //! [positive]|     |                   |       ✓         |  ✓   |
            //! [negative]|  ✓  |   ✓              |                  |      |
            //! [finite]  |     |   ✓              |       ✓          |      |
            //! [ranged]  |     |   ✓              |       ✓          |      |
            //! [completely_random] | |     ✓      |       ✓          |      |
            //! [zero_to_one]|  |                  |        ✓         |      |
            //!
            //!
            //! All generators but [completely_random] has some overwheight
            //! towards special values, like for example extreme values of given
            //! range, ±0, ±1, `±Inf` and `NaN`.

            use crate::BoxGen;
            use std::ops::RangeBounds;

            /// Generator that return any floating point value, including any
            /// finite number, NaN, Inf and -Inf.
            ///
            /// This generator has some overwheight to special border case
            /// values.
            pub fn any() -> BoxGen<$name> {
                super::float::any()
            }

            /// Generator that only return finite numbers, `-Inf` and
            /// `Inf`. In other words any float value besides `NaN`.
            ///
            /// This generator has some overwheight to special border case
            /// values.
            pub fn number() -> BoxGen<$name> {
                super::float::number()
            }

            /// Generator that only return numbers between 0 and
            /// `+Inf`.
            ///
            /// This generator has some overwheight to special border case
            /// values.
            pub fn positive() -> BoxGen<$name> {
                super::float::positive()
            }

            /// Generator that only return numbers between `-Inf`
            /// and -0.
            ///
            /// This generator has some overwheight to special border case
            /// values.
            pub fn negative() -> BoxGen<$name> {
                super::float::negative()
            }

            /// Generator that only return finite numbers between minimum
            /// and maximum finite number.
            ///
            /// This generator has some overwheight to special border case
            /// values.
            pub fn finite() -> BoxGen<$name> {
                super::float::finite()
            }

            /// Generator that only return finite numbers in the range from 0
            /// (inclusive) to 1 (exclusive).
            ///
            /// This generator has some overwheight to special border case
            /// values.
            pub fn zero_to_one() -> BoxGen<$name> {
                super::float::zero_to_one()
            }

            /// Generator that only return finite numbers in the given range.
            ///
            /// This generator has some overwheight to special border case
            /// values.
            ///
            /// It will throw if any of `±Inf` or `NaN` is part of the given
            /// range.
            pub fn ranged<B>(bounds: B) -> BoxGen<$name>
            where
                B: RangeBounds<$name>,
            {
                super::float::ranged(bounds)
            }

            /// Float generator with completely random distribution.
            /// This function has a longer name, since `ranged` should be
            /// preferred.
            ///
            /// It will throw if any of `±Inf` or `NaN` is part of the given
            /// range.
            pub fn completely_random<B>(bounds: B) -> BoxGen<$name>
            where
                B: RangeBounds<$name>,
            {
                super::float::completely_random_range(bounds)
            }
        }
    };
}

float_module!(f32);
float_module!(f64);
