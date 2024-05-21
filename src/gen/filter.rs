use crate::*;

/// Creates a generator that uses the given predicate to determine
/// which examples should be kept and not filtered out.
pub fn filter<E, P>(original_gen: BoxGen<E>, predicate: P) -> BoxGen<E>
where
    E: Clone + 'static,
    P: Fn(&E) -> bool + Clone + 'static,
{
    let original_shrinker = original_gen.shrinker();
    let filtered_shrinker = original_shrinker.filter(predicate.clone());

    crate::gen::from_fn(move |seed| {
        let p = predicate.clone();
        original_gen.examples(seed).filter(move |e| p(e))
    })
    .with_shrinker(filtered_shrinker)
}

#[cfg(test)]
mod test {
    use self::testing::assert_iter_eq;
    use crate::*;

    #[test]
    fn filter_out_evens() {
        assert_iter_eq(
            gen::fixed::sequence(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13])
                .filter(|e| e % 2 == 0)
                .examples(1337),
            vec![2, 4, 6, 8, 10, 12],
            "only even numbers should be kept in the filtering",
        );
    }
}
