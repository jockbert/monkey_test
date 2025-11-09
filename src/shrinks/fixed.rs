//! Shrinkers mainly used for internal testing or debugging, where you want a
//! deterministicly provided candidate values.

use crate::BoxShrink;

/// Provides a fixed sequence of candidates and then ends, having no more values.
pub fn sequence<E>(candidates: &[E]) -> BoxShrink<E>
where
    E: Clone + std::fmt::Debug + 'static,
{
    let data = candidates.to_vec();
    crate::shrinks::from_fn(move |_original| data.clone().into_iter())
}
