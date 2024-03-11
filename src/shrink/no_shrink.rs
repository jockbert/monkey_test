use crate::BoxShrink;

/// Empty shrinker not producing any smaller examples given original example.
pub fn none<E>() -> BoxShrink<E>
where
    E: Clone + 'static,
{
    crate::shrink::from_fn(|_original| std::iter::empty())
}
