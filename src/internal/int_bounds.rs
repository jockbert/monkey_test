//! Functions for handling range bounds.

use num_traits::PrimInt;
use std::ops::Bound;
use std::ops::RangeBounds;
use std::ops::RangeInclusive;

/// Get the inclusive start and end values as a tuple
pub fn to_inclusive_range_tuple<E, B>(bounds: &B) -> (E, E)
where
    E: PrimInt,
    B: RangeBounds<E>,
{
    (start(bounds), end(bounds))
}

/// Convert given int range bounds to an inclusive range.
pub fn to_inclusive_range<E, B>(bounds: &B) -> RangeInclusive<E>
where
    E: PrimInt,
    B: RangeBounds<E>,
{
    let (start, end) = to_inclusive_range_tuple(bounds);
    start..=end
}

/// Get the start value from a range bounds
fn start<E, B>(bounds: &B) -> E
where
    E: PrimInt,
    B: RangeBounds<E>,
{
    match bounds.start_bound() {
        Bound::Included(x) => *x,
        Bound::Excluded(x) => *x + E::one(),
        Bound::Unbounded => E::min_value(),
    }
}

/// Get the end value from a range bounds
fn end<E, B>(bounds: &B) -> E
where
    E: PrimInt,
    B: RangeBounds<E>,
{
    match bounds.end_bound() {
        Bound::Included(x) => *x,
        Bound::Excluded(x) => *x - E::one(),
        Bound::Unbounded => E::max_value(),
    }
}
