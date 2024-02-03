use crate::BoxGen;
use crate::BoxIter;
use crate::BoxShrink;
use crate::Gen;

/// Generator wrapper that allows binding new shrinker to existing generator.
#[derive(Clone)]
pub struct ZipGen<E0, E1>
where
    E0: Clone,
    E1: Clone,
{
    generator0: BoxGen<E0>,
    generator1: BoxGen<E1>,
}

/// Combine two generators together element wise into generator of tuples.
///
/// ```rust
/// use monkey_test::*;
///
/// let bytes1: BoxGen<u8> = gen::u8::any();
/// let bytes2: BoxGen<u8> = gen::u8::any();
/// let chars1: BoxGen<char> = gen::pick_evenly(&['a', 'b', 'c', 'd']);
/// let chars2: BoxGen<char> = gen::pick_evenly(&['a', 'b', 'c', 'd']);
///
/// // Zip two generators to a tuple generator.
/// let tuples1: BoxGen<(u8, char)> = gen::zip(bytes1, chars1);
///
/// // Shorthand way to do the same thing.
/// let tuples2: BoxGen<(u8, char)> = bytes2.zip(chars2);
/// ```
pub fn zip<E0, E1>(g0: BoxGen<E0>, g1: BoxGen<E1>) -> BoxGen<(E0, E1)>
where
    E0: Clone + 'static,
    E1: Clone + 'static,
{
    Box::new(ZipGen::<E0, E1> {
        generator0: g0,
        generator1: g1,
    })
}

impl<E0, E1> Gen<(E0, E1)> for ZipGen<E0, E1>
where
    E0: Clone + 'static,
    E1: Clone + 'static,
{
    fn examples(&self, seed: u64) -> BoxIter<(E0, E1)> {
        let it1 = self.generator0.clone().examples(seed);
        let it2 = self.generator1.clone().examples(seed);

        Box::new(it1.zip(it2))
    }

    fn shrinker(&self) -> BoxShrink<(E0, E1)> {
        crate::shrink::zip(
            self.generator0.shrinker(),
            self.generator1.shrinker(),
        )
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
        let gen = super::zip(
            fixed::sequence::<u8>(&[1, 2, 3, 4]),
            // Empty input generator
            fixed::sequence::<u8>(&[]),
        );

        // Empty output generator
        assert_generator_is_empty(gen);
    }

    #[test]
    fn always_same_values_with_generators_ignoring_seed() {
        let gen = super::zip(
            fixed::sequence(&[1, 2]),
            fixed::sequence(&['a', 'b', 'c', 'd']),
        );

        let expected = even_distribution_of(&[(1, 'a'), (2, 'b')]);

        assert_generator_has_distribution_within_percent(gen, expected, 1.0)
    }

    #[test]
    fn even_distribution_with_generators_using_seed() {
        let gen = super::zip(
            crate::gen::i32::completely_random(1..4),
            crate::gen::pick_evenly(&['a', 'b', 'c']),
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

        assert_generator_has_distribution_within_percent(gen, expected, 1.5)
    }

    #[test]
    fn automatically_can_shrink_tuple_examples() {
        let shrinker =
            super::zip(crate::gen::u8::any(), crate::gen::u8::any()).shrinker();

        assert_shrinker_has_some_candidates_given(shrinker, (4, 0))
    }
}
