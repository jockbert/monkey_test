use std::marker::PhantomData;

use crate::{Gen, Shrink, SomeIter};

/// Generator wrapper that allows binding new shrinker to existing generator.
#[derive(Clone)]
pub struct ChainGen<E, G1, G2>
where
    E: Clone,
    G1: Gen<E>,
    G2: Gen<E>,
{
    e_phantom: PhantomData<E>,
    generator1: G1,
    generator2: G2,
}

impl<E, G1, G2> ChainGen<E, G1, G2>
where
    E: Clone,
    G1: Gen<E>,
    G2: Gen<E>,
{
    /// Create a new generator with (other) shrinker
    pub fn new(g1: &G1, g2: &G2) -> ChainGen<E, G1, G2> {
        ChainGen::<E, G1, G2> {
            e_phantom: PhantomData,
            generator1: g1.clone(),
            generator2: g2.clone(),
        }
    }
}

impl<E, G1, G2> Gen<E> for ChainGen<E, G1, G2>
where
    E: Clone + 'static,
    G1: Gen<E>,
    G2: Gen<E>,
{
    type Shrink = G1::Shrink;

    fn examples(&self, seed: u64) -> SomeIter<E> {
        Box::new(
            self.generator1
                .examples(seed)
                .chain(self.generator2.examples(seed)),
        )
    }

    fn shrinker(&self) -> Self::Shrink {
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
            &fixed::sequence::<u8>(&[]),
            &fixed::sequence::<u8>(&[]),
        );
        let mut it = gen.examples(1234);
        assert_eq!(None, it.next())
    }

    #[test]
    fn some_elements_in_each_generator() {
        let gen = ChainGen::new(
            &fixed::sequence::<u8>(&[1, 2]),
            &fixed::sequence::<u8>(&[3, 4]),
        );
        let mut it = gen.examples(1234);
        assert_eq!(Some(1u8), it.next());
        assert_eq!(Some(2u8), it.next());
        assert_eq!(Some(3u8), it.next());
        assert_eq!(Some(4u8), it.next());
        assert_eq!(None, it.next());
    }
}
