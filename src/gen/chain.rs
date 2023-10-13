use std::marker::PhantomData;

use crate::{Gen, Shrink, SomeIter};

/// Generator wrapper that allows binding new shrinker to existing generator.
#[derive(Clone)]
pub struct ChainGen<E, G1, S1, G2, S2>
where
    E: Clone,
    G1: Gen<E, S1>,
    S1: Shrink<E>,
    G2: Gen<E, S2>,
    S2: Shrink<E>,
{
    e_phantom: PhantomData<E>,
    s1_phantom: PhantomData<S1>,
    s2_phantom: PhantomData<S2>,
    generator1: G1,
    generator2: G2,
}

impl<E, G1, S1, G2, S2> ChainGen<E, G1, S1, G2, S2>
where
    E: Clone,
    G1: Gen<E, S1>,
    S1: Shrink<E>,
    G2: Gen<E, S2>,
    S2: Shrink<E>,
{
    /// Create a new generator with (other) shrinker
    pub fn new(g1: &G1, g2: &G2) -> ChainGen<E, G1, S1, G2, S2> {
        ChainGen::<E, G1, S1, G2, S2> {
            e_phantom: PhantomData,
            s1_phantom: PhantomData,
            s2_phantom: PhantomData,
            generator1: g1.clone(),
            generator2: g2.clone(),
        }
    }
}

impl<E, G1, S1, G2, S2> Gen<E, S1> for ChainGen<E, G1, S1, G2, S2>
where
    E: Clone + 'static,
    G1: Gen<E, S1>,
    S1: Shrink<E>,
    G2: Gen<E, S2>,
    S2: Shrink<E>,
{
    fn examples(&self, seed: u64) -> SomeIter<E> {
        Box::new(
            self.generator1
                .examples(seed)
                .chain(self.generator2.examples(seed)),
        )
    }

    fn shrinker(&self) -> S1 {
        self.generator1.shrinker().clone()
    }
}

#[cfg(test)]
mod test {
    use crate::{gen::fixed, Gen};

    use super::ChainGen;

    #[test]
    fn empty_generators() {
        let gen = ChainGen::new(&fixed::sequence::<u8>(&[]), &fixed::sequence::<u8>(&[]));
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
