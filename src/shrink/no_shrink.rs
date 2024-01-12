use crate::BoxIter;
use crate::BoxShrink;
use crate::Shrink;
use std::marker::PhantomData;

/// Empty shrinker not producing any smaller examples given original example.
pub fn none<E>() -> BoxShrink<E>
where
    E: Clone + 'static,
{
    Box::new(NoShrink {
        phantom: PhantomData,
    })
}

/// Shrinker that does nothing.
#[derive(Clone)]
struct NoShrink<E> {
    phantom: PhantomData<E>,
}

impl<E: Clone + 'static> Shrink<E> for NoShrink<E> {
    fn candidates(&self, _: E) -> BoxIter<E> {
        Box::new(std::iter::empty::<E>())
    }
}
