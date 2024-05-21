use crate::BoxShrink;

/// Creates a shrinker that uses the given predicate to determine
/// which shrink candidates that should be kept and not filtered out.
pub fn filter<E, P>(original_shrink: BoxShrink<E>, predicate: P) -> BoxShrink<E>
where
    E: Clone + 'static,
    P: Fn(&E) -> bool + Clone + 'static,
{
    crate::shrink::from_fn(move |original: E| {
        let pred = predicate.clone();
        let mut filter_streak = 0;

        original_shrink.clone().candidates(original.clone()).filter(
            move |candidate| {
                let verdict = pred(candidate);

                filter_streak = if verdict { 0 } else { filter_streak + 1 };

                if filter_streak >= 100 {
                    panic!(
                        "Too heavy filtering. Filtered out 100 examples in a \
                        row. For test performance, please use more efficient \
                        way to generate examples than heavy reliance on \
                        filtering."
                    )
                }

                verdict
            },
        )
    })
}

#[cfg(test)]
mod test {
    use crate::testing::assert_iter_eq;
    use crate::*;

    #[test]
    fn should_be_able_to_filter_using_predicate() {
        let shrinker = crate::shrink::fixed::sequence(&[
            11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1,
        ])
        .filter(|e| e % 2 == 0);

        assert_iter_eq(
            shrinker.candidates(1337),
            vec![10, 8, 6, 4, 2],
            "only even candidates should be returned",
        )
    }

    #[test]
    #[should_panic = "Too heavy filtering. Filtered out 100 examples in a row. \
       For test performance, please use more efficient way to generate \
       examples than heavy reliance on filtering."]
    fn should_panic_on_too_heavy_filtering() {
        let filtererd_shrinker =
            crate::shrink::int_to_zero::<u8>().filter(|&e| e == 234);

        // Trying to get a shrinked candidate should throw, since all
        // candidates are filtered out.
        filtererd_shrinker.candidates(200).next();
    }

    #[test]
    fn should_not_panic_on_repeaded_filtering() {
        let filtererd_shrinker =
            crate::shrink::int_to_zero::<u16>().filter(|&e| e % 2 == 0);

        // Trying to get a shrinked candidate should throw, since all
        // candidates are filtered out.
        assert!(
            filtererd_shrinker.candidates(2000).count() > 1000,
            "More candidates that the limit for to heavy filtering is filtered \
            out without panicing");
    }
}
