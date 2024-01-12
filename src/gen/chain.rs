use crate::{BoxGen, BoxIter, BoxShrink, Gen};

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
    Box::new(ChainGen::new(first_gen, second_gen))
}

/// Generator wrapper that allows binding new shrinker to existing generator.
#[derive(Clone)]
struct ChainGen<E>
where
    E: Clone,
{
    generator1: BoxGen<E>,
    generator2: BoxGen<E>,
}

impl<E> ChainGen<E>
where
    E: Clone,
{
    /// Create a new generator with (other) shrinker
    pub fn new(g1: BoxGen<E>, g2: BoxGen<E>) -> ChainGen<E> {
        ChainGen::<E> {
            generator1: g1,
            generator2: g2,
        }
    }
}

impl<E> Gen<E> for ChainGen<E>
where
    E: Clone + 'static,
{
    fn examples(&self, seed: u64) -> BoxIter<E> {
        Box::new(
            self.generator1
                .examples(seed)
                .chain(self.generator2.examples(seed)),
        )
    }

    fn shrinker(&self) -> BoxShrink<E> {
        self.generator1.shrinker().clone()
    }
}

#[cfg(test)]
mod test {
    use crate::{gen::fixed, Gen};

    use super::ChainGen;

    #[test]
    fn empty_generators() {
        let gen = ChainGen::new(
            fixed::sequence::<u8>(&[]),
            fixed::sequence::<u8>(&[]),
        );
        let mut it = gen.examples(1234);
        assert_eq!(None, it.next())
    }

    #[test]
    fn some_elements_in_each_generator() {
        let gen = ChainGen::new(
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
