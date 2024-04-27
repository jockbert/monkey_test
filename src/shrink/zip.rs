use crate::BoxShrink;

/// Combine two shrinkers together element wise into shrinker of tuples.
///
/// ```rust
/// use monkey_test::*;
///
/// let alfa1: BoxShrink<u8> = shrink::int::<u8>();
/// let beta1: BoxShrink<i64> = shrink::int::<i64>();
///
/// let alfa2: BoxShrink<u8> = shrink::int::<u8>();
/// let beta2: BoxShrink<i64> = shrink::int::<i64>();
///
///
/// // Zip two shrinkers to a tuple shrinker.
/// let tuples1: BoxShrink<(u8, i64)> = shrink::zip(alfa1, beta1);
///
/// // Shorthand way to do the same thing.
/// let tuples2: BoxShrink<(u8, i64)> = alfa2.zip(beta2);
/// ```
pub fn zip<E0, E1>(
    shrink0: BoxShrink<E0>,
    shrink1: BoxShrink<E1>,
) -> BoxShrink<(E0, E1)>
where
    E0: Clone + 'static,
    E1: Clone + 'static,
{
    crate::shrink::from_fn(move |original: (E0, E1)| {
        let o0 = original.0.clone();
        let o1 = original.1.clone();

        let it_left = shrink0
            .candidates(original.0.clone())
            .map(move |item0| (item0, o1.clone()));

        let it_right = shrink1
            .candidates(original.1.clone())
            .map(move |item1| (o0.clone(), item1));

        let it_both = shrink0
            .candidates(original.0.clone())
            .zip(shrink1.candidates(original.1.clone()));

        it_left.chain(it_right).chain(it_both)
    })
}

#[cfg(test)]
mod test {
    use crate::shrink::int_to_zero;
    use crate::shrink::none;
    use crate::testing::assert_shrinker_has_at_least_these_candidates;
    use crate::BoxShrink;

    #[test]
    fn no_shrinking_if_no_element_shrinkers() {
        let shrink: BoxShrink<(u8, char)> =
            super::zip(none::<u8>(), none::<char>());

        let actual_length = shrink.candidates((100, 'x')).take(1000).count();

        assert_eq!(actual_length, 0)
    }

    /// The combination of candidates are not complete, but is good enough as
    /// initial behaviour.
    #[test]
    fn returns_permutations_of_inner_candidates() {
        let shrink: BoxShrink<(u8, u8)> =
            super::zip(int_to_zero(), int_to_zero());

        assert_shrinker_has_at_least_these_candidates(
            shrink,
            (4, 4),
            &[
                (4, 3),
                (4, 2),
                (4, 1),
                (4, 0),
                (3, 4),
                (2, 4),
                (1, 4),
                (0, 4),
                (3, 3),
                (2, 2),
                (1, 1),
                (0, 0),
            ],
        );
    }
}
