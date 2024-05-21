use crate::BoxShrink;

/// Creates a shrinker that uses the given predicate to determine
/// which shrink candidates that should be kept and not filtered out.
pub fn filter<E, P>(original_shrink: BoxShrink<E>, predicate: P) -> BoxShrink<E>
where
    E: Clone + 'static,
    P: Fn(&E) -> bool + Clone + 'static,
{
    crate::shrink::from_fn(move |original: E| {
        original_shrink
            .clone()
            .candidates(original.clone())
            .filter(predicate.clone())
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
}
