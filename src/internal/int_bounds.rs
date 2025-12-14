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
        Bound::Excluded(x) => (*x).saturating_add(E::one()),
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
        Bound::Excluded(x) => (*x).saturating_sub(E::one()),
        Bound::Unbounded => E::max_value(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_inclusive_range_tuple() {
        assert_eq!(to_inclusive_range_tuple(&(3..=7)), (3, 7));
        assert_eq!(to_inclusive_range_tuple(&(3..8)), (3, 7));
        assert_eq!(to_inclusive_range_tuple(&(..=5)), (i32::MIN, 5));
        assert_eq!(to_inclusive_range_tuple(&(4..)), (4, i32::MAX));
        assert_eq!(to_inclusive_range_tuple(&(..)), (i32::MIN, i32::MAX));
    }

    #[test]
    fn test_to_inclusive_range() {
        assert_eq!(to_inclusive_range(&(3..=7)), 3..=7);
        assert_eq!(to_inclusive_range(&(3..8)), 3..=7);
        assert_eq!(to_inclusive_range(&(..=5)), i32::MIN..=5);
        assert_eq!(to_inclusive_range(&(4..)), 4..=i32::MAX);
        assert_eq!(to_inclusive_range(&(..)), i32::MIN..=i32::MAX);
    }

    #[test]
    fn test_subtract_with_overflow_when_making_exclusive_min_end_inclusive() {
        assert_eq!(to_inclusive_range_tuple(&(..u8::MIN)), (u8::MIN, u8::MIN));
        assert_eq!(to_inclusive_range(&(..u8::MIN)), (u8::MIN..=u8::MIN));
    }

    /// Range implementations in std-lib does not have exclusive start, so
    /// making one up for the test below.
    struct RangeExclusive {
        start: u8,
        end: u8,
    }

    impl RangeBounds<u8> for RangeExclusive {
        fn start_bound(&self) -> Bound<&u8> {
            Bound::Excluded(&self.start)
        }

        fn end_bound(&self) -> Bound<&u8> {
            Bound::Excluded(&self.end)
        }
    }

    #[test]
    fn test_add_with_overflow_when_making_exclusive_max_start_inclusive() {
        let sanity_check = RangeExclusive { start: 5, end: 15 };
        assert_eq!(to_inclusive_range_tuple(&sanity_check), (6, 14));
        assert_eq!(to_inclusive_range(&sanity_check), (6..=14));

        let bad_range = RangeExclusive {
            start: u8::MAX,
            end: u8::MAX,
        };
        let almost_max = u8::MAX - 1;

        assert_eq!(to_inclusive_range_tuple(&bad_range), (u8::MAX, almost_max));
        assert_eq!(to_inclusive_range(&bad_range), (u8::MAX..=almost_max));
    }
}
