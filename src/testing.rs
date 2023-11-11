use crate::Gen;

pub mod distribution;
pub mod numbers;

use crate::BoxGen;

/// Makes sure generator is empty
pub fn assert_generator_is_empty<E>(gen_to_check: BoxGen<E>)
where
    E: Clone + 'static,
{
    assert_eq!(gen_to_check.examples(1234).count(), 0);
}
