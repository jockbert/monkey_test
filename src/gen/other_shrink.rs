use crate::{Gen, Shrink, SomeIter};

/// Generator wrapper that allows binding new shrinker to existing generator.
#[derive(Clone)]
pub struct OtherShrinkGen<G, S>
where
    G: Gen,
    S: Shrink<G::Example>,
{
    generator: G,
    shrinker: S,
}

impl<G, S> OtherShrinkGen<G, S>
where
    G: Gen,
    S: Shrink<G::Example>,
{
    /// Create a new generator with (other) shrinker
    pub fn new(g: &G, s2: S) -> OtherShrinkGen<G, S> {
        OtherShrinkGen::<G, S> {
            generator: g.clone(),
            shrinker: s2,
        }
    }
}

impl<G, S> Gen for OtherShrinkGen<G, S>
where
    G: Gen,
    S: Shrink<G::Example>,
{
    type Example = G::Example;
    type Shrink = S;

    fn examples(&self, seed: u64) -> SomeIter<Self::Example> {
        self.generator.examples(seed)
    }

    fn shrinker(&self) -> S {
        self.shrinker.clone()
    }
}
