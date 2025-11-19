use crate::BoxGen;

/// Convert a generator of type E0 to a generator of type E1.
///
/// This enables generating examples of type E1
/// by piggybacking on a generator of type E0,
/// produce examples of type E0 that is then mapped to type E1.
///
/// The unmapping function is used for reverse the mapping back to type E0 from
/// E1, enabling piggybacking of the associated shrinker of type
/// E0, to automatically also get shrinking of type E1. This requires that
/// there is an associated shrinker of type E0.
///
/// ```rust
/// use monkey_test::*;
///
/// let number_string_generator: BoxGen<String> = gens::map(
///     gens::i64::any(),
///     |i: i64| i.to_string(),
///     |s: String| s.parse().unwrap(),
/// );
///
/// let even_numbers_only_generator: BoxGen<u64> = gens::map(
///     gens::u64::ranged(..10_000),
///     |i: u64| i * 2,
///     |even: u64| even / 2,
/// );
///
/// // Shorthand way to do the same thing
/// let even_numbers_only_generator_2: BoxGen<u64> = gens::u64::ranged(..10_000)
///     .map(|i| i * 2, |e| e / 2);
/// ```
pub fn map<E0, E1>(
    gen0: BoxGen<E0>,
    map_fn: fn(E0) -> E1,
    unmap_fn: fn(E1) -> E0,
) -> BoxGen<E1>
where
    E0: Clone + 'static,
    E1: Clone + 'static,
{
    let shrinker = gen0.shrinker();

    crate::gens::from_fn(move |seed, size| {
        gen0.examples(seed, size).map(map_fn)
    })
    .with_shrinker(crate::shrinks::map(shrinker, map_fn, unmap_fn))
}

#[cfg(test)]
mod test {
    use crate::gens::fixed;
    use crate::testing::assert_generator_is_empty;
    use crate::testing::assert_shrinker_has_some_candidates_given;
    use crate::testing::distribution::assert_generator_has_distribution_within_percent;
    use crate::testing::distribution::even_distribution_of;

    #[test]
    fn empty_generators_can_not_build_anything() {
        let generator = super::map(
            fixed::sequence::<u8>(&[]),
            |i| i.to_string(),
            |s| s.parse().unwrap(),
        );

        // Empty output generator
        assert_generator_is_empty(generator);
    }

    #[test]
    fn always_same_values_with_generators_that_ignore_seed() {
        let generator = super::map(
            fixed::sequence(&[1, 2]),
            |i| i.to_string(),
            |s| s.parse().unwrap(),
        );

        let expected =
            even_distribution_of(&["1".to_string(), "2".to_string()]);

        assert_generator_has_distribution_within_percent(
            generator, expected, 1.0,
        )
    }

    #[test]
    fn even_distribution_with_generators_using_seed() {
        let generator = super::map(
            crate::gens::i32::completely_random(1..4),
            |i| i * 10,
            |i| i / 10,
        );

        let expected = even_distribution_of(&[10, 20, 30]);

        assert_generator_has_distribution_within_percent(
            generator, expected, 1.5,
        )
    }

    #[test]
    fn automatically_can_shrink_mapped_examples() {
        let shrinker = super::map(
            crate::gens::u8::any(),
            |i| i.to_string(),
            |s| s.parse().unwrap(),
        )
        .shrinker();

        assert_shrinker_has_some_candidates_given(shrinker, "4".to_string())
    }
}
