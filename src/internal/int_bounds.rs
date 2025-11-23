//! Functions for handling range bounds.

use num_traits::PrimInt;
use std::ops::Bound;
use std::ops::RangeBounds;

/// Get the inclusive start and end values as a tuple
pub fn to_inclusive_range_tuple<E, B>(bounds: &B) -> (E, E)
where
    E: PrimInt,
    B: RangeBounds<E>,
{
    (start(bounds), end(bounds))
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
