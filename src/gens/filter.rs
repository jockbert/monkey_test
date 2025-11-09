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

    crate::gens::from_fn(move |seed| {
        let pred = predicate.clone();
        let mut filter_streak = 0;

        original_gen.examples(seed).filter(move |example| {
            let verdict = pred(example);

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
        })
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
            gens::fixed::sequence(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13])
                .filter(|e| e % 2 == 0)
                .examples(1337),
            vec![2, 4, 6, 8, 10, 12],
            "only even numbers should be kept in the filtering",
        );
    }

    #[test]
    #[should_panic = "Too heavy filtering. Filtered out 100 examples in a row. \
       For test performance, please use more efficient way to generate \
       examples than heavy reliance on filtering."]
    fn should_panic_on_too_heavy_filtering() {
        let filtererd_generator = crate::gens::u128::any().filter(|_| false);

        // Trying to get an example should throw, since all examples are
        // filtered out.
        filtererd_generator.examples(1337).next();
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
            out without panicing"
        );
    }
}
