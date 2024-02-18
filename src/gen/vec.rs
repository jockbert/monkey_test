//! Generators for vectors.

use crate::BoxGen;
use crate::BoxIter;
use crate::BoxShrink;
use crate::Gen;

/// Any vector filled with values from given element generator
pub fn any<E: Clone + 'static>(element_gen: BoxGen<E>) -> BoxGen<Vec<E>> {
    Box::new(VecGen::<E> { element_gen })
}

#[derive(Clone)]
struct VecGen<E> {
    element_gen: BoxGen<E>,
}

impl<E> Gen<Vec<E>> for VecGen<E>
where
    E: Clone + 'static,
{
    fn examples(&self, seed: u64) -> BoxIter<Vec<E>> {
        let sizes = crate::gen::sized::default().examples(seed);
        let seeds = crate::gen::u64::completely_random(..).examples(seed);
        let element_gen = self.element_gen.clone();

        let vectors = sizes.zip(seeds).map(move |(size, seed)| {
            element_gen.examples(seed).take(size).collect::<Vec<_>>()
        });

        Box::new(vectors)
    }

    fn shrinker(&self) -> BoxShrink<Vec<E>> {
        crate::shrink::vec::default(self.element_gen.shrinker())
    }
}
