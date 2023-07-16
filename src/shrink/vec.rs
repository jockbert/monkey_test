//! Shrinkers for vectors

use crate::Shrink;
use std::marker::PhantomData;

/// Default vector shrinker
pub fn default<T: Clone + 'static>() -> crate::SomeShrink<Vec<T>> {
    Box::new(VecShrink::<T> {
        phantom: PhantomData,
    })
}

/// Vector version of shrinker
pub struct VecShrink<T>
where
    T: Clone,
{
    phantom: PhantomData<T>,
}

impl<T> Shrink<Vec<T>> for VecShrink<T>
where
    T: Clone + 'static,
{
    fn candidates(&self, original: Vec<T>) -> Box<dyn Iterator<Item = Vec<T>>> {
        Box::new(VecIterator::<T> { current: original })
    }
}

/// Vector shrink iterator
pub struct VecIterator<T> {
    current: Vec<T>,
}

impl<T> Iterator for VecIterator<T>
where
    T: Clone,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_empty() {
            None
        } else {
            self.current.remove(self.current.len() - 1);
            Some(self.current.clone())
        }
    }
}
