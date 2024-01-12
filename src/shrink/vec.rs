//! Shrinkers for vectors

use crate::{BoxIter, BoxShrink, Shrink};
use std::marker::PhantomData;

/// Default vector shrinker
pub fn default<E: Clone + 'static>() -> BoxShrink<Vec<E>> {
    Box::new(VecShrink::<E> {
        phantom: PhantomData,
    })
}

/// Vector version of shrinker
#[derive(Clone)]
struct VecShrink<E>
where
    E: Clone,
{
    phantom: PhantomData<E>,
}

impl<E> Shrink<Vec<E>> for VecShrink<E>
where
    E: Clone + 'static,
{
    fn candidates(&self, original: Vec<E>) -> BoxIter<Vec<E>> {
        Box::new(VecIterator::<E> { current: original })
    }
}

/// Vector shrink iterator
struct VecIterator<E> {
    current: Vec<E>,
}

impl<E> Iterator for VecIterator<E>
where
    E: Clone,
{
    type Item = Vec<E>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_empty() {
            None
        } else {
            self.current.remove(self.current.len() - 1);
            Some(self.current.clone())
        }
    }
}
