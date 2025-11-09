use crate::BoxGen;

/// Combine two generators together element wise into generator of tuples.
///
/// ```rust
/// use monkey_test::*;
///
/// let bytes1: BoxGen<u8> = gens::u8::any();
/// let bytes2: BoxGen<u8> = gens::u8::any();
/// let chars1: BoxGen<char> = gens::pick_evenly(&['a', 'b', 'c', 'd']);
/// let chars2: BoxGen<char> = gens::pick_evenly(&['a', 'b', 'c', 'd']);
///
/// // Zip two generators to a tuple generator.
/// let tuples1: BoxGen<(u8, char)> = gens::zip(bytes1, chars1);
///
/// // Shorthand way to do the same thing.
/// let tuples2: BoxGen<(u8, char)> = bytes2.zip(chars2);
/// ```
pub fn zip<E0, E1>(g0: BoxGen<E0>, g1: BoxGen<E1>) -> BoxGen<(E0, E1)>
where
    E0: Clone + 'static,
    E1: Clone + 'static,
{
    let s0 = g0.shrinker();
    let s1 = g1.shrinker();

    crate::gens::from_fn(move |seed| {
        let mut seeds = crate::gens::seeds().examples(seed);
        let it1 = g0.clone().examples(seeds.next().expect("should have seed"));
        let it2 = g1.clone().examples(seeds.next().expect("should have seed"));
        it1.zip(it2)
    })
    .with_shrinker(crate::shrink::zip(s0, s1))
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
        let generator = super::zip(
            fixed::sequence::<u8>(&[1, 2, 3, 4]),
            // Empty input generator
            fixed::sequence::<u8>(&[]),
        );

        // Empty output generator
        assert_generator_is_empty(generator);
    }

    #[test]
    fn always_same_values_with_generators_ignoring_seed() {
        let generator = super::zip(
            fixed::sequence(&[1, 2]),
            fixed::sequence(&['a', 'b', 'c', 'd']),
        );

        let expected = even_distribution_of(&[(1, 'a'), (2, 'b')]);

        assert_generator_has_distribution_within_percent(
            generator, expected, 1.0,
        )
    }

    #[test]
    fn even_distribution_with_generators_using_seed() {
        let generator = super::zip(
            crate::gens::i32::completely_random(1..4),
            crate::gens::pick_evenly(&['a', 'b', 'c']),
        );

        let expected = even_distribution_of(&[
            (1, 'a'),
            (1, 'b'),
            (1, 'c'),
            (2, 'a'),
            (2, 'b'),
            (2, 'c'),
            (3, 'a'),
            (3, 'b'),
            (3, 'c'),
        ]);

        assert_generator_has_distribution_within_percent(
            generator, expected, 1.5,
        )
    }

    #[test]
    fn automatically_can_shrink_tuple_examples() {
        let shrinker =
            super::zip(crate::gens::u8::any(), crate::gens::u8::any())
                .shrinker();

        assert_shrinker_has_some_candidates_given(shrinker, (4, 0))
    }

    /// Make sure tuple do not just contain twin values, like (31, 31), in case
    /// same generator type is use for both parts of tuple.
    #[test]
    fn use_different_seeds_for_the_different_parts_of_tuple() {
        let same_gen = crate::gens::u8::any();
        let tuples = super::zip(same_gen.clone(), same_gen);
        assert! {tuples.examples(1234).take(100).any(|(a,b)| a!= b)}
    }
}
