use crate::{BoxGen, BoxIter, BoxShrink, ExampleSize, Gen};

/// Create new generator with other shrinker
///
/// ```rust
/// use monkey_test::*;
///
/// let gen1 = gens::u8::any();
/// assert!(gen1.shrinker().candidates(123).next().is_some());
///
/// // let generator have other shrinker
/// let gen2 = gens::other_shrinker(gen1, shrinks::none());
/// assert!(gen2.shrinker().candidates(123).next().is_none());
///
/// // let generator have other shrinker again (alternative way)
/// let gen3 = gen2.with_shrinker(shrinks::int());
/// assert!(gen3.shrinker().candidates(123).next().is_some());
/// ```
pub fn other_shrinker<E: Clone + 'static>(
    generator: BoxGen<E>,
    other_shrink: BoxShrink<E>,
) -> BoxGen<E> {
    Box::new(OtherShrinkGen {
        generator,
        shrinker: other_shrink,
    })
}

/// Generator wrapper that allows binding new shrinker to existing generator.
#[derive(Clone)]
struct OtherShrinkGen<E> {
    generator: BoxGen<E>,
    shrinker: BoxShrink<E>,
}

impl<E: Clone + 'static> Gen<E> for OtherShrinkGen<E> {
    fn examples(&self, seed: u64, size: ExampleSize) -> BoxIter<E> {
        self.generator.examples(seed, size)
    }

    fn shrinker(&self) -> BoxShrink<E> {
        self.shrinker.clone()
    }
}
