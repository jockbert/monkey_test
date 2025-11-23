//! Generators for values of type `isize`.

// Please note! This module does not use the integer_module macro, since the
// underlying rand library does not support isize directly. For details, see:
// https://rust-random.github.io/book/update-0.9.html
//
// The code here should be similar to what the macro would have generated, but
// is piggybacking on the generators for type `i64` and just maps to type
// `isize`.

use crate::internal::int_bounds;
use crate::BoxGen;
use crate::*;
use core::panic;
use std::ops::{RangeBounds, RangeInclusive};

/// Roughly uniformly distributed unbound range of values, with
/// some overwheight to extremes (min and max).
pub fn any() -> BoxGen<isize> {
    ranged(..)
}

/// Roughly uniformly distributed range of values, with some
/// overwheight to extremes (min and max) of given bounds.
pub fn ranged<B>(bounds: B) -> BoxGen<isize>
where
    B: RangeBounds<isize>,
{
    assert_lib_supports_isize_bit_width();
    let i64_bounds = map_to_i64_bounds(&bounds);
    map_to_isize_gen(super::i64::ranged(i64_bounds))
}

/// Int generator with completely random distribution. This
/// function has a long name, since `ranged` should be preferred.
pub fn completely_random<B>(bounds: B) -> BoxGen<isize>
where
    B: RangeBounds<isize>,
{
    assert_lib_supports_isize_bit_width();
    let i64_bounds = map_to_i64_bounds(&bounds);
    map_to_isize_gen(super::i64::completely_random(i64_bounds))
}

/// Maps isize bounds to i64 bounds.
fn map_to_i64_bounds<B>(i_bounds: &B) -> RangeInclusive<i64>
where
    B: RangeBounds<isize>,
{
    let start: isize = int_bounds::start(i_bounds);
    let end: isize = int_bounds::end(i_bounds);
    (start as i64)..=(end as i64)
}

/// Maps `i64` generator to a `isize` generator.
fn map_to_isize_gen(gen: BoxGen<i64>) -> BoxGen<isize> {
    gen.map(|i| i as isize, |j| j as i64)
}

/// Ensure that the library supports the bit width of isize on this platform.
fn assert_lib_supports_isize_bit_width() {
    if isize::BITS > i64::BITS {
        panic!(
          "Generators for isize only support platforms where isize is at most \
          64 bits wide. \
          Please contact the library author for support for wider isize types.");
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::distribution::assert_generator_has_distribution_within_percent;
    use crate::testing::distribution::distribution_from_pairs;

    /// Generator values should be evenly distributed within range.
    #[test]
    fn random_inclusive_has_uniform_distribution() {
        assert_generator_has_distribution_within_percent(
            super::completely_random(-10isize..=10isize),
            distribution_from_pairs(&[
                (1, -10isize),
                (1, -9isize),
                (1, -8isize),
                (1, -7isize),
                (1, -6isize),
                (1, -5isize),
                (1, -4isize),
                (1, -3isize),
                (1, -2isize),
                (1, -1isize),
                (1, 0isize),
                (1, 1isize),
                (1, 2isize),
                (1, 3isize),
                (1, 4isize),
                (1, 5isize),
                (1, 6isize),
                (1, 7isize),
                (1, 8isize),
                (1, 9isize),
                (1, 10isize),
            ]),
            2.0, // 2% tolerance
        );
    }
}
