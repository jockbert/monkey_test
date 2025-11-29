use crate::BoxShrink;

/// Convert a shrinker of type E0 to a shrinker of type E1.
///
/// This enables shrinking of examples of type E1
/// by piggybacking on a shrinker of type E0,
/// unmapping original example from E1 to E0,
/// produce smaller examples of type E0
/// that is finally mapped back to type E1 again.
///
/// ```rust
/// use monkey_test::*;
///
/// let number_string_shrinker: BoxShrink<String> = shrinks::map(
///     shrinks::int_to_zero::<i64>(),
///     |i: i64| i.to_string(),
///     |s: String| s.parse().unwrap(),
/// );
///
/// let even_numbers_only_shrinker: BoxShrink<i64> = shrinks::map(
///     shrinks::int_to_zero::<i64>(),
///     |i:i64| i * 2,
///     |even: i64| even / 2,
/// );
///
/// // Shorthand way to do the same thing
/// let even_numbers_only_shrinker_2: BoxShrink<i64> =
///     shrinks::int_to_zero::<i64>()
///         .map(|i: i64| i * 2, |even: i64| even / 2);
/// ```
pub fn map<E0, E1>(
    shrink0: BoxShrink<E0>,
    map_fn: fn(E0) -> E1,
    unmap_fn: fn(E1) -> E0,
) -> BoxShrink<E1>
where
    E0: Clone + 'static,
    E1: Clone + 'static,
{
    crate::shrinks::from_fn(move |original: E1| {
        let o1 = original.clone();
        let o0 = (unmap_fn)(o1);
        shrink0.candidates(o0).map(map_fn)
    })
}

#[cfg(test)]
mod test {
    use crate::shrinks::int_to_zero;
    use crate::shrinks::none;
    use crate::testing::assert_shrinker_has_at_least_these_candidates;
    use crate::BoxShrink;

    #[test]
    fn no_shrinking_if_no_element_shrinkers() {
        // The backing shrinker is a "no-op" shrinker.
        let shrink: BoxShrink<String> = super::map(
            none::<u8>(),
            |i| i.to_string(),
            |s: String| s.parse().unwrap(),
        );

        let actual_length = shrink.candidates("100".into()).take(1000).count();

        assert_eq!(actual_length, 0)
    }

    #[test]
    fn returns_some_other_stringified_numbers() {
        let shrink: BoxShrink<String> = super::map(
            int_to_zero::<i64>(),
            |i| i.to_string(),
            |s: String| s.parse().unwrap(),
        );

        assert_shrinker_has_at_least_these_candidates(
            shrink,
            "100".into(),
            &[
                "99".into(),
                "98".into(),
                "50".into(),
                "1".into(),
                "0".into(),
            ],
        );
    }
}
