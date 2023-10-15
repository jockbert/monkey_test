//! The `gen` module contains built in generators.

use crate::Gen;
use crate::Shrink;

pub use other_shrink::OtherShrinkGen;

pub(crate) mod chain;
pub mod fixed;
mod other_shrink;
pub mod u8;
pub mod usize;
pub mod vec;

/// Create new generator with other shrinker
///
/// ```rust
/// use monkey_test::*;
///
/// let gen = gen::u8::any();
/// assert!(gen.shrinker().candidates(123).next().is_some());
///
/// // let generator have other shrinker
/// let gen2 = gen::other_shrinker(&gen, shrink::none());
/// assert!(gen2.shrinker().candidates(123).next().is_none());
///
/// // let generator have other shrinker again (alternative way)
/// let gen3 = gen2.with_shrinker(shrink::number());
/// assert!(gen3.shrinker().candidates(123).next().is_some());
/// ```
pub fn other_shrinker<E, G, S, S2>(
    gen: &G,
    other_shrink: S2,
) -> OtherShrinkGen<E, G, S, S2>
where
    E: Clone,
    G: Gen<E, S>,
    S: Shrink<E>,
    S2: Shrink<E>,
{
    OtherShrinkGen::new(gen, other_shrink)
}
