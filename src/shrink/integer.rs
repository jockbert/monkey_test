use crate::BoxShrink;
use num_traits::PrimInt;

/// Shrink integer types towards the value zero.
pub fn int_to_zero<E>() -> BoxShrink<E>
where
    E: PrimInt + 'static,
{
    crate::shrink::from_fn(move |original| {
        eager(original)
            .chain(decrement(original))
            .chain(as_positive(original))
    })
}

fn decrement<E>(original: E) -> impl Iterator<Item = E>
where
    E: PrimInt,
{
    // Values closes to original are covered by eager iterator
    let mut last = if original.is_zero() {
        original
    } else if original < E::zero() {
        original.add(E::one())
    } else {
        original.sub(E::one())
    };

    std::iter::from_fn(move || {
        // Candidate zero is already covered by eager iterator.
        if last.is_one() || last.is_zero() {
            None
        } else if last < E::zero() {
            last = last.add(E::one());
            Some(last)
        } else {
            last = last.sub(E::one());
            Some(last)
        }
    })
}

fn eager<E>(original: E) -> impl Iterator<Item = E>
where
    E: PrimInt,
{
    let mut decrement = original;
    let two = E::one().add(E::one());

    std::iter::from_fn(move || {
        if decrement.is_zero() {
            None
        } else {
            let result = original.sub(decrement);
            decrement = decrement.div(two);
            Some(result)
        }
    })
}

fn as_positive<E>(original: E) -> impl Iterator<Item = E>
where
    E: PrimInt,
{
    let mut o = original;
    std::iter::from_fn(move || {
        if o < E::zero() && E::zero().sub(E::max_value()) <= o {
            o = E::zero().sub(o);
            Some(o)
        } else {
            None
        }
    })
}

#[cfg(test)]
mod test {
    use crate::testing::assert_iter_eq;

    #[test]
    fn decrement_can_shrink_both_positive_and_negative_numbers() {
        assert_eq!(super::decrement(0).next(), None);
        assert_eq!(super::decrement(1).next(), None);
        assert_eq!(super::decrement(-1).next(), None);
        assert_eq!(super::decrement(i8::MAX).next(), Some(i8::MAX - 2));
        assert_eq!(super::decrement(i8::MIN).next(), Some(i8::MIN + 2));
    }

    #[test]
    fn eager_tries_iteratively_smaller_decrement_from_original() {
        assert_iter_eq(
            super::eager(16),
            vec![0, 8, 12, 14, 15],
            "positives don to zero",
        );
        assert_iter_eq(
            super::eager(-16),
            vec![0, -8, -12, -14, -15],
            "negatives up to zero",
        );
        assert_iter_eq(
            super::eager(17),
            vec![0, 9, 13, 15, 16],
            "not a multiple of 2",
        );
    }
}
