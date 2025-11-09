use std::cmp::Ordering;

use crate::BoxShrink;
use num_traits::PrimInt;

/// Shrink integer types towards the value zero.
#[deprecated = "use int_in_range for better shrink behaviour, \
or int_to_zero for the old behaviour."]
pub fn int<E>() -> BoxShrink<E>
where
    E: PrimInt + std::fmt::Debug + 'static,
{
    int_in_range(E::min_value(), E::max_value())
}

/// Shrink integer types towards the value zero.
pub fn int_to_zero<E>() -> BoxShrink<E>
where
    E: PrimInt + std::fmt::Debug + 'static,
{
    int_in_range(E::min_value(), E::max_value())
}

/// Shrink integer types towards the value zero if in range, or other value
/// nearest zero within range of min and max.
pub fn int_in_range<E>(min: E, max: E) -> BoxShrink<E>
where
    E: PrimInt + std::fmt::Debug + 'static,
{
    crate::shrinks::from_fn(move |original| {
        eager(original, min, max).chain(decrement(original, min, max))
    })
}

fn decrement<E: PrimInt + std::fmt::Debug>(
    original: E,
    min: E,
    max: E,
) -> impl Iterator<Item = E> {
    check_example_is_in_range(original, min, max);
    let target = shrink_target(min, max);

    let mut last = original;

    std::iter::from_fn(move || match last.cmp(&target) {
        Ordering::Equal => None,
        Ordering::Less => {
            let maybe_positive = mirror_around(target, last);
            let maybe_negative =
                if last == original { None } else { Some(last) };

            last = last + E::one();

            let maybe_target = if last == target { Some(target) } else { None };

            Some(
                vec![maybe_negative, maybe_positive, maybe_target]
                    .into_iter()
                    .flatten(),
            )
        }
        Ordering::Greater => {
            last = last - E::one();

            if last == target {
                Some(vec![Some(last)].into_iter().flatten())
            } else {
                let maybe_negative = mirror_around(target, last);
                Some(vec![maybe_negative, Some(last)].into_iter().flatten())
            }
        }
    })
    .flatten()
    // Filter out examples out of range
    .filter(move |e| min <= *e && *e <= max)
}

/// Eagerly try shrink down original failing example using bisection. Tries
/// smaller an smaller decrements from original failure example down to
/// decrement of two. Decrement one unnessesarily genereates the same values as
/// the separate decrement iterator.
///
/// Test out both the positive and negative complement (mirror) of candidate.
/// Positive value is regarded as closer to target than negative value.
///
/// Shrink target is kept inside the given range from min to max. If min is
/// greater than zero, min is choosen. If max is lower than zero, max is
/// choosen. In all other cases zero is the target.
fn eager<E>(original: E, min: E, max: E) -> impl Iterator<Item = E>
where
    E: PrimInt + std::fmt::Debug,
{
    check_example_is_in_range(original, min, max);
    let target = shrink_target(min, max);

    let two = E::one() + E::one();
    let mut decrement = (original - target) / two;

    std::iter::from_fn(move || {
        let is_neg_one =
            decrement < E::zero() && (decrement + E::one()).is_zero();

        // No need to continue if decrement is small
        if decrement.is_zero() || is_neg_one || decrement.is_one() {
            None
        } else {
            let result = original - decrement;
            decrement = decrement / two;

            let maybe_mirror_result = mirror_around(target, result);
            maybe_mirror_result
                .map(|mirror| vec![result, mirror])
                .or(Some(vec![result]))
        }
    })
    .flatten()
    // Filter out examples out of range
    .filter(move |e| min <= *e && *e <= max)
}

/// Panics if example is out of range
fn check_example_is_in_range<E: PrimInt + std::fmt::Debug>(
    example: E,
    min: E,
    max: E,
) {
    if example < min || example > max {
        panic!("Given example {example:?} is not in range {min:?}..={max:?}.")
    }
}

/// Chooses shrink target based on given valid range.
///
/// By default zero is the shrink target, but this can be shifted by the given
/// valid range.
fn shrink_target<E: PrimInt>(range_min: E, range_max: E) -> E {
    if range_min > E::zero() {
        range_min
    } else if range_max < E::zero() {
        range_max
    } else {
        E::zero()
    }
}

/// Mirror a value `x` around the `mirror`, returning `2*mirror-x`, if possible
/// witout overflow.
fn mirror_around<E: PrimInt>(mirror: E, x: E) -> Option<E> {
    let diff = x.saturating_sub(mirror);
    let x_mirrored = mirror.saturating_sub(diff);
    let mirror_diff = mirror.saturating_sub(x_mirrored);
    // Only emitting mirror of we can calculate back to x, i.e. no saturation.
    if mirror.add(mirror_diff) == x {
        Some(x_mirrored)
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use num_traits::PrimInt;

    use crate::{testing::assert_iter_eq, BoxGen};

    #[test]
    fn candidates_are_always_within_min_max_range_unsigned() {
        use crate::*;

        let mmms = crate::gens::u8::any()
            .zip_3(crate::gens::u8::any(), crate::gens::u8::any());

        assert_within_range(mmms);
    }

    #[test]
    fn candidates_are_always_within_min_max_range_signed() {
        use crate::*;

        let mmms = crate::gens::i8::any()
            .zip_3(crate::gens::i8::any(), crate::gens::i8::any());

        assert_within_range(mmms);
    }

    fn assert_within_range<E>(g: BoxGen<(E, E, E)>)
    where
        E: PrimInt + std::fmt::Debug + std::panic::UnwindSafe + 'static,
    {
        crate::monkey_test()
            .with_example_count(1000)
            .with_generator(g)
            .assert_no_panic(|(a, b, c)| {
                let min = a.min(b).min(c);
                let max = a.max(b).max(c);
                let middle = if a != min && a != max {
                    a
                } else if b != min && b != max {
                    b
                } else {
                    c
                };

                super::int_in_range(min, max).candidates(middle).for_each(
                    |candidate| {
                        assert!(
                            candidate >= min,
                            "candidate {candidate:?} >= min {min:?}"
                        );
                        assert!(
                            candidate <= max,
                            "candidate {candidate:?} <= max {max:?}"
                        );
                    },
                );
            });
    }

    #[test]
    fn shrinker_can_shrink_from_one_to_zero() {
        assert_iter_eq(
            super::int_to_zero().candidates(1),
            vec![0],
            "can shrink from 1 to 0",
        )
    }
    #[test]
    fn shrinker_can_shrink_from_negative_one_to_zero_and_one() {
        assert_iter_eq(
            super::int_to_zero().candidates(-1),
            vec![1, 0],
            "can shrink from -1 to 1 and 0",
        )
    }

    #[test]
    fn decrement_can_shrink_both_positive_and_negative_numbers() {
        assert_iter_eq(
            super::decrement(-3, i64::MIN, i64::MAX),
            vec![3, -2, 2, -1, 1, 0],
            "decrement without restriction",
        );

        assert_iter_eq(
            super::decrement(-3, i64::MIN, -1),
            vec![-2, -1],
            "decrement to targer -1 with upper bound restriction -1",
        );

        assert_iter_eq(
            super::decrement(-3, i64::MIN, 1),
            vec![-2, -1, 1, 0],
            "decrement to target 0 with partial upper bound restriction 1",
        );

        assert_iter_eq(
            super::decrement(4, i64::MIN, i64::MAX),
            vec![-3, 3, -2, 2, -1, 1, 0],
            "decrement from 4 to target 0 without restiction",
        );
    }

    #[test]
    fn target_adjusts_with_range() {
        assert_eq!(super::shrink_target(-5, 9), 0);
        assert_eq!(super::shrink_target(-1, 9), 0);
        assert_eq!(super::shrink_target(-11, -9), -9);
        assert_eq!(super::shrink_target(-11, -2), -2);
        assert_eq!(super::shrink_target(1, 9), 1);
        assert_eq!(super::shrink_target(3, 9), 3);
    }

    #[test]
    fn eager_tries_iteratively_smaller_decrement_both_positive_and_negative() {
        assert_iter_eq(
            super::eager(16, i64::MIN, i64::MAX),
            vec![8, -8, 12, -12, 14, -14],
            "shrinks toward zero, interleave positive and negative canidates \
            if not restricted by given range",
        );
    }

    #[test]
    fn eager_tries_iteratively_smaller_decrement_with_positive_only_for_unsigned(
    ) {
        assert_iter_eq(
            super::eager(16, u64::MIN, u64::MAX),
            vec![8, 12, 14],
            "shrinks toward zero, with only positive canidates for unsigned \
            integer typed",
        );
    }

    #[test]
    fn eager_only_returns_candidates_within_range() {
        assert_iter_eq(
            super::eager(16, -5, i64::MAX),
            vec![8, 12, 14],
            "shrinks down toward zero, but only positives are in range",
        );

        assert_iter_eq(
            super::eager(17, 1, i64::MAX),
            vec![9, 13, 15],
            "shrink as in first but translated 1 to rateger 1 from 17, so \
            difference from original_failure to target is still 16",
        );

        assert_iter_eq(
            super::eager(-16, i64::MIN, 5),
            vec![-8, -12, -14],
            "same shrinking as in first assert, but only negative are in range.",
        );

        assert_iter_eq(
            super::eager(16, -12, i64::MAX),
            vec![8, -8, 12, -12, 14],
            "same shrinking as in first assert, but some (!) negative are in range.",
        );
    }

    /// Providing failing examples out of range is logically wrong and
    /// will panic. Assumptions and graceful degradation can be made but
    /// not sure it is the logically right thing to do. Better to be
    /// restrictive now and possibly open up API at a later time.
    #[test]
    #[should_panic = "Given example 1337 is not in range 0..=16."]
    fn eager_with_example_out_of_range_should_panic() {
        let _ = super::eager(1337, u64::MIN, 16);
    }
}
