use crate::{Gen, Shrink, SomeIter};
use std::marker::PhantomData;

/// Generator wrapper that allows binding new shrinker to existing generator.
#[derive(Clone)]
pub struct OtherShrinkGen<E, G, S>
where
    E: Clone,
    G: Gen<E>,
    S: Shrink<E>,
{
    e_phantom: PhantomData<E>,
    generator: G,
    shrinker: S,
}

impl<E, G, S> OtherShrinkGen<E, G, S>
where
    E: Clone,
    G: Gen<E>,
    S: Shrink<E>,
{
    /// Create a new generator with (other) shrinker
    pub fn new(g: &G, s2: S) -> OtherShrinkGen<E, G, S> {
        OtherShrinkGen::<E, G, S> {
            e_phantom: PhantomData,
            generator: g.clone(),
            shrinker: s2,
        }
    }
}

impl<E, G, S> Gen<E> for OtherShrinkGen<E, G, S>
where
    E: Clone + 'static,
    G: Gen<E>,
    S: Shrink<E>,
{
    type Shrink = S;

    fn examples(&self, seed: u64) -> SomeIter<E> {
        self.generator.examples(seed)
    }

    fn shrinker(&self) -> S {
        self.shrinker.clone()
    }
}
