use crate::BoxGen;

/// Concatenate together two generators.
///
/// Second generator is used only when first generator is emptied.
/// If first generator is infinite, second generator will never be used.
///
/// ```
/// use monkey_test::gen::fixed::sequence;
/// use monkey_test::*;
///
/// let a = sequence::<u32>(&[1, 2]);
/// let b = sequence::<u32>(&[3, 4]);
/// let c = a.chain(b);
/// let mut it = c.examples(77);
///
/// assert_eq!(it.collect::<Vec<_>>(), vec![1, 2, 3, 4]);
/// ```
pub fn chain<E>(first_gen: BoxGen<E>, second_gen: BoxGen<E>) -> BoxGen<E>
where
    E: Clone + 'static,
{
    let shrinker = first_gen.shrinker();

    crate::gen::from_fn(move |seed| {
        first_gen.examples(seed).chain(second_gen.examples(seed))
    })
    .with_shrinker(shrinker)
}

#[cfg(test)]
mod test {
    use crate::{gen::fixed, testing::assert_iter_eq};

    #[test]
    fn empty_generators() {
        let gen = super::chain(
            fixed::sequence::<u8>(&[]),
            fixed::sequence::<u8>(&[]),
        );

        assert_iter_eq(
            gen.examples(1234),
            vec![],
            "empty generators has no examples to concatenate",
        );
    }

    #[test]
    fn some_elements_in_each_generator() {
        let gen = super::chain(
            fixed::sequence::<u8>(&[1, 2]),
            fixed::sequence::<u8>(&[3, 4]),
        );

        assert_iter_eq(
            gen.examples(1234),
            vec![1, 2, 3, 4],
            "given generators' examples are concatenated",
        );
    }
}
