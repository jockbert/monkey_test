use crate::{BoxGen, BoxIter, BoxShrink, Gen};

/// Generator wrapper that allows binding new shrinker to existing generator.
#[derive(Clone)]
pub struct OtherShrinkGen<E> {
    generator: BoxGen<E>,
    shrinker: BoxShrink<E>,
}

impl<E: Clone + 'static> OtherShrinkGen<E> {
    /// Create a new generator with (other) shrinker
    pub fn new(g: BoxGen<E>, s2: BoxShrink<E>) -> BoxGen<E> {
        Box::new(OtherShrinkGen {
            generator: g,
            shrinker: s2,
        })
    }
}

impl<E: Clone + 'static> Gen<E> for OtherShrinkGen<E> {
    fn examples(&self, seed: u64) -> BoxIter<E> {
        self.generator.examples(seed)
    }

    fn shrinker(&self) -> BoxShrink<E> {
        self.shrinker.clone()
    }
}
