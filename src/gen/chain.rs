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
/// assert_eq!(Some(1), it.next());
/// assert_eq!(Some(2), it.next());
/// assert_eq!(Some(3), it.next());
/// assert_eq!(Some(4), it.next());
/// assert_eq!(None, it.next());
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
    use crate::gen::fixed;

    #[test]
    fn empty_generators() {
        let gen = super::chain(
            fixed::sequence::<u8>(&[]),
            fixed::sequence::<u8>(&[]),
        );
        let mut it = gen.examples(1234);
        assert_eq!(None, it.next())
    }

    #[test]
    fn some_elements_in_each_generator() {
        let gen = super::chain(
            fixed::sequence::<u8>(&[1, 2]),
            fixed::sequence::<u8>(&[3, 4]),
        );
        let mut it = gen.examples(1234);
        assert_eq!(Some(1u8), it.next());
        assert_eq!(Some(2u8), it.next());
        assert_eq!(Some(3u8), it.next());
        assert_eq!(Some(4u8), it.next());
        assert_eq!(None, it.next());
    }
}
