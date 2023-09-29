//! The `gen` module contains built in generators.

use std::marker::PhantomData;

use crate::Gen;
use crate::Shrink;
use crate::SomeIter;

pub mod fixed;
pub mod u8;
pub mod vec;

/// Generator wrapper that allows binding new shrinker to existing generator.
#[derive(Clone)]
pub struct OtherShrinkGen<E, G, S, S2>
where
    E: Clone,
    G: Gen<E, S>,
    S: Shrink<E>,
    S2: Shrink<E>,
{
    e_phantom: PhantomData<E>,
    s_phantom: PhantomData<S>,
    generator: G,
    shrinker: S2,
}

impl<E, G, S, S2> OtherShrinkGen<E, G, S, S2>
where
    E: Clone,
    G: Gen<E, S>,
    S: Shrink<E>,
    S2: Shrink<E>,
{
    /// Create a new generator with (other) shrinker
    pub fn new(g: &G, s2: S2) -> OtherShrinkGen<E, G, S, S2> {
        OtherShrinkGen::<E, G, S, S2> {
            e_phantom: PhantomData,
            s_phantom: PhantomData,
            generator: g.clone(),
            shrinker: s2,
        }
    }
}

impl<E, G, S, S2> Gen<E, S2> for OtherShrinkGen<E, G, S, S2>
where
    E: Clone + 'static,
    G: Gen<E, S>,
    S: Shrink<E>,
    S2: Shrink<E>,
{
    fn examples(&self, seed: u64) -> SomeIter<E> {
        self.generator.examples(seed)
    }

    /// Returns a predefined shrinker, or a empty shrinker if no suitable exists.
    ///
    /// This enables distributing a default shrinker with given generator,
    /// reducing the need to explicitly configure a shrinker at place of use.
    ///
    /// When implementing a [Gen], you can return a empty
    /// [crate::shrink::NoShrink] shrinker,
    /// if that makes the implementation easier, but when you will not
    /// get any shrinking functionality applied to failing example.
    fn shrinker(&self) -> S2 {
        self.shrinker.clone()
    }
}
