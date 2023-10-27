use crate::{Gen, SomeIter};

/// Generator wrapper that allows binding new shrinker to existing generator.
#[derive(Clone)]
pub struct ChainGen<G1, G2>
where
    G1: Gen,
    G2: Gen<Example = G1::Example>,
{
    generator1: G1,
    generator2: G2,
}

impl<G1, G2> ChainGen<G1, G2>
where
    G1: Gen,
    G2: Gen<Example = G1::Example>,
{
    /// Create a new generator with (other) shrinker
    pub fn new(g1: &G1, g2: &G2) -> ChainGen<G1, G2> {
        ChainGen::<G1, G2> {
            generator1: g1.clone(),
            generator2: g2.clone(),
        }
    }
}

impl<G1, G2> Gen for ChainGen<G1, G2>
where
    G1: Gen,
    G1::Example: 'static,
    G2: Gen<Example = G1::Example>,
{
    type Example = G1::Example;
    type Shrink = G1::Shrink;

    fn examples(&self, seed: u64) -> SomeIter<Self::Example> {
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
