//! Generators that constrain the size of generated examples.

use std::ops::RangeBounds;

use crate::internal::int_bounds::to_inclusive_range;
use crate::BoxGen;

/// Creates a generator decorator that constrains the example sizes to be
/// within the given range.
///
/// This function takes an existing generator and a range, and returns a new
/// generator that will only generate examples with sizes within the given
/// range. The size range passed to the original generator will be overridden
/// with the given range.
pub fn of_size<E, R>(gen: BoxGen<E>, range: R) -> BoxGen<E>
where
    E: Clone + 'static,
    R: RangeBounds<usize> + Clone + 'static,
{
    let shrinker = gen.shrinker();
    let constrained_size = to_inclusive_range(&range);

    crate::gens::from_fn(move |seed, _ignored_size| {
        gen.examples(seed, constrained_size.clone())
    })
    .with_shrinker(shrinker)
}
