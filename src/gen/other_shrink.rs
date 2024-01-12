use crate::{BoxGen, BoxIter, BoxShrink, Gen};

/// Create new generator with other shrinker
///
/// ```rust
/// use monkey_test::*;
///
/// let gen = gen::u8::any();
/// assert!(gen.shrinker().candidates(123).next().is_some());
///
/// // let generator have other shrinker
/// let gen2 = gen::other_shrinker(gen, shrink::none());
/// assert!(gen2.shrinker().candidates(123).next().is_none());
///
/// // let generator have other shrinker again (alternative way)
/// let gen3 = gen2.with_shrinker(shrink::number());
/// assert!(gen3.shrinker().candidates(123).next().is_some());
/// ```
pub fn other_shrinker<E: Clone + 'static>(
    gen: BoxGen<E>,
    other_shrink: BoxShrink<E>,
) -> BoxGen<E> {
    Box::new(OtherShrinkGen {
        generator: gen,
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
    fn examples(&self, seed: u64) -> BoxIter<E> {
        self.generator.examples(seed)
    }

    fn shrinker(&self) -> BoxShrink<E> {
        self.shrinker.clone()
    }
}
