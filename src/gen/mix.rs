use crate::gen::Ratio;
use crate::gen::SampleTarget;
use crate::BoxGen;
use crate::BoxIter;
use crate::BoxShrink;
use crate::Gen;
use rand::Rng;
use rand::SeedableRng;

/// Mix values from given generators evenly.
///
/// Example, where a mixed and non-uniform distribution of `u8` values ensuring
/// extremes (`u8` min and max) are tested with 33% chance:
/// ```rust
/// use monkey_test::*;
///
/// let low = gen::u8::ranged(1..20);
/// let all = gen::u8::any();
/// let extremes = gen::pick_evenly(&[0u8, 255u8]);
///
/// let mixed = gen::mix_evenly(&[low, all, extremes]);
/// ```
pub fn mix_evenly<E>(generators: &[BoxGen<E>]) -> BoxGen<E>
where
    E: Clone + 'static + core::fmt::Debug,
{
    Box::new(MixGen {
        sample_target: SampleTarget::evenly(generators),
    })
}

/// Mix values from given generators in given ratios.
///
/// Example, where a mixed and non-uniform distribution of `u8` values ensuring
/// extremes (`u8` min and max) are tested with a 10% *=1/(4+5+1)* chance:
/// ```rust
/// use monkey_test::*;
///
/// let low = gen::u8::ranged(1..20);
/// let all = gen::u8::any();
/// let extremes = gen::pick_evenly(&[0u8, 255u8]);
///
/// let mixed = gen::mix_with_ratio(&[(4, low), (5, all), (1, extremes)]);
/// ```
pub fn mix_with_ratio<E>(ratios_and_gens: &[(Ratio, BoxGen<E>)]) -> BoxGen<E>
where
    E: Clone + 'static + core::fmt::Debug,
{
    Box::new(MixGen {
        sample_target: SampleTarget::with_ratios(ratios_and_gens),
    })
}

/// Generator for a given set of examples to pick from.
#[derive(Clone)]
struct MixGen<E> {
    sample_target: crate::gen::sample_target::SampleTarget<BoxGen<E>>,
}

impl<E> Gen<E> for MixGen<E>
where
    E: Clone + 'static + core::fmt::Debug,
{
    fn examples(&self, seed: u64) -> BoxIter<E> {
        let high = self.sample_target.sample_domain_max();
        let distr = rand::distributions::Uniform::new_inclusive(1usize, high);
        let rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);

        let mut sample_iterators =
            self.sample_target.clone().map(|gen| gen.examples(seed));

        let gen_iter = rng
            .sample_iter(distr)
            .map(move |sample| {
                sample_iterators
                    .target_from_sample_mut(sample)
                    .map(|it| it.next())
                    .unwrap()
            })
            // Some of the internal generators being empty (returning None) is
            // the end criteria for the whole mixing generator.
            .take_while(|opt| opt.is_some())
            .flatten();

        Box::new(gen_iter)
    }

    fn shrinker(&self) -> BoxShrink<E> {
        crate::shrink::none()
    }
}

#[cfg(test)]
mod test {
    use crate::testing::distribution::{
        assert_generator_distribution_similar_to, distribution_from_pairs,
    };

    /// If implementation is wrong, generator is cloned every time and only the
    /// first state or element is returned every time. This test makes sure all
    /// states or examples in generator are returned until no more is to return.
    #[test]
    fn all_values_from_generator_are_returned() {
        // Mixing with a single generator should use values from that generator
        // every time.
        let mixer = super::mix_with_ratio(&[(
            42,
            crate::gen::fixed::sequence(&[1, 2, 3, 4, 5, 6]),
        )]);

        let mut iter = mixer.examples(1337);
        assert!(iter.next() == Some(1));
        assert!(iter.next() == Some(2));
        assert!(iter.next() == Some(3));
        assert!(iter.next() == Some(4));
        assert!(iter.next() == Some(5));
        assert!(iter.next() == Some(6));
        assert!(iter.next().is_none());
    }

    #[test]
    fn mixer_be_used_more_than_once() {
        let mixer = super::mix_with_ratio(&[(
            42,
            crate::gen::fixed::sequence(&['a', 'b', 'c']),
        )]);

        let mut iter = mixer.examples(1337);
        assert!(iter.next() == Some('a'));
        assert!(iter.next() == Some('b'));
        assert!(iter.next() == Some('c'));
        assert!(iter.next().is_none());
        assert!(iter.next().is_none());
        assert!(iter.next().is_none());

        iter = mixer.examples(1337);
        assert!(iter.next() == Some('a'));
        assert!(iter.next() == Some('b'));
        assert!(iter.next() == Some('c'));
        assert!(iter.next().is_none());
        assert!(iter.next().is_none());
        assert!(iter.next().is_none());

        assert!(mixer.examples(1337).next() == Some('a'));
        assert!(mixer.examples(1337).next() == Some('a'));
        assert!(mixer.examples(1337).next() == Some('a'));
    }

    /// A and B will be 4 times more frequent than 1, 2, 3, and 4, both because
    /// first generators should be mixed in twice as often and because
    /// there are half the number ov values to choose from in that generator.
    #[test]
    fn values_are_distributed_according_to_ratio() {
        let mixer = super::mix_with_ratio(&[
            (64, crate::gen::pick_evenly(&['A', 'B'])),
            (32, crate::gen::pick_evenly(&['1', '2', '3', '4'])),
        ]);

        assert_generator_distribution_similar_to(
            mixer,
            distribution_from_pairs(&[
                (4, 'A'),
                (4, 'B'),
                (1, '1'),
                (1, '2'),
                (1, '3'),
                (1, '4'),
            ]),
        )
    }

    /// A and B will be 2 times more frequent than 1, 2, 3, and 4, just because
    /// there are half the number ov values to choose from in first generator.
    #[test]
    fn mix_evenly_has_uniform_distribution() {
        let mixer = super::mix_evenly(&[
            crate::gen::pick_evenly(&['A', 'B']),
            crate::gen::pick_evenly(&['1', '2', '3', '4']),
        ]);

        assert_generator_distribution_similar_to(
            mixer,
            distribution_from_pairs(&[
                (2, 'A'),
                (2, 'B'),
                (1, '1'),
                (1, '2'),
                (1, '3'),
                (1, '4'),
            ]),
        )
    }
}
