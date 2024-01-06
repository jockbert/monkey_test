use std::marker::PhantomData;

use crate::{BoxIter, Shrink};

/// Shrinker that does nothing.
#[derive(Clone)]
pub struct NoShrink<E> {
    phantom: PhantomData<E>,
}

impl<E> Default for NoShrink<E> {
    fn default() -> Self {
        NoShrink::<E> {
            phantom: PhantomData,
        }
    }
}

impl<E: Clone + 'static> Shrink<E> for NoShrink<E> {
    fn candidates(&self, _: E) -> BoxIter<E> {
        Box::new(std::iter::empty::<E>())
    }
}
