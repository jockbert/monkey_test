use crate::BoxGen;
use crate::BoxIter;
use crate::BoxShrink;
use crate::Gen;

/// Generator wrapper that allows binding new shrinker to existing generator.
#[derive(Clone)]
pub struct MapGen<E0, E1>
where
    E0: Clone,
    E1: Clone,
{
    gen0: BoxGen<E0>,
    map_fn: fn(E0) -> E1,
    unmap_fn: fn(E1) -> E0,
}

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
/// let number_string_generator: BoxGen<String> = gen::map(
///     gen::i64::any(),
///     |i: i64| i.to_string(),
///     |s: String| s.parse().unwrap(),
/// );
///
/// let even_numbers_only_generator: BoxGen<u64> = gen::map(
///     gen::u64::ranged(..10_000),
///     |i: u64| i * 2,
///     |even: u64| even / 2,
/// );
///
/// // Shorthand way to do the same thing
/// let even_numbers_only_generator_2: BoxGen<u64> = gen::u64::ranged(..10_000)
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
    Box::new(MapGen::<E0, E1> {
        gen0,
        map_fn,
        unmap_fn,
    })
}

impl<E0, E1> Gen<E1> for MapGen<E0, E1>
where
    E0: Clone + 'static,
    E1: Clone + 'static,
{
    fn examples(&self, seed: u64) -> BoxIter<E1> {
        let it = self.gen0.clone().examples(seed).map(self.map_fn);
        Box::new(it)
    }

    fn shrinker(&self) -> BoxShrink<E1> {
        crate::shrink::map(self.gen0.shrinker(), self.map_fn, self.unmap_fn)
    }
}

#[cfg(test)]
mod test {
    use crate::gen::fixed;
    use crate::testing::assert_generator_is_empty;
    use crate::testing::assert_shrinker_has_some_candidates_given;
    use crate::testing::distribution::assert_generator_has_distribution_within_percent;
    use crate::testing::distribution::even_distribution_of;

    #[test]
    fn empty_generators_can_not_build_anything() {
        let gen = super::map(
            fixed::sequence::<u8>(&[]),
            |i| i.to_string(),
            |s| s.parse().unwrap(),
        );

        // Empty output generator
        assert_generator_is_empty(gen);
    }

    #[test]
    fn always_same_values_with_generators_that_ignore_seed() {
        let gen = super::map(
            fixed::sequence(&[1, 2]),
            |i| i.to_string(),
            |s| s.parse().unwrap(),
        );

        let expected =
            even_distribution_of(&["1".to_string(), "2".to_string()]);

        assert_generator_has_distribution_within_percent(gen, expected, 1.0)
    }

    #[test]
    fn even_distribution_with_generators_using_seed() {
        let gen = super::map(
            crate::gen::i32::completely_random(1..4),
            |i| i * 10,
            |i| i / 10,
        );

        let expected = even_distribution_of(&[10, 20, 30]);

        assert_generator_has_distribution_within_percent(gen, expected, 1.5)
    }

    #[test]
    fn automatically_can_shrink_mapped_examples() {
        let shrinker = super::map(
            crate::gen::u8::any(),
            |i| i.to_string(),
            |s| s.parse().unwrap(),
        )
        .shrinker();

        assert_shrinker_has_some_candidates_given(shrinker, "4".to_string())
    }
}
