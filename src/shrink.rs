//! The `shrink` module contains built in shrinkers.

pub mod vec;

use num_traits::Num;

use crate::Shrink;

/// Shrinker that does nothing.
pub struct NoShrink {}

impl<E: 'static> Shrink<E> for NoShrink {
    fn candidates(&self, _: E) -> Box<dyn Iterator<Item = E>> {
        Box::new(std::iter::empty::<E>())
    }
}

/// A collection of shrinkers for numeric type T.
pub struct NumericShrinks<E>
where
    E: Num + Copy,
{
    min: E,
    max: E,
}

impl<E> NumericShrinks<E>
where
    E: Num + Copy + 'static,
{
    /// Shrinks a value to zero.
    pub fn to_zero(&self) -> NumericShrink {
        NumericShrink {}
    }

    /// Shrinker not producing any smaller values.
    pub fn no_shrink(&self) -> NumericShrink {
        NumericShrink {}
    }

    /// Shrinks a value to zero.
    pub fn decrement(&self) -> NumDecrementShrink {
        NumDecrementShrink {}
    }
}

/// Shrinker that decrements a value towards zero.
pub struct NumDecrementShrink {}

impl<E> Shrink<E> for NumDecrementShrink
where
    E: Num + Copy + 'static,
{
    fn candidates(&self, original: E) -> Box<dyn Iterator<Item = E>> {
        let _next = match original {
            x if x == E::zero() => None,
            _ => Some(original.sub(E::one())),
        };

        Box::new(NumDecrementIterator::<E> { current: original })
    }
}

/// Iterator that decrements a value towards zero.
pub struct NumDecrementIterator<E> {
    current: E,
}

impl<E> Iterator for NumDecrementIterator<E>
where
    E: Num + Copy,
{
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == E::zero() {
            None
        } else {
            self.current = self.current.sub(E::one());
            Some(self.current)
        }
    }
}

/// A shrinker for numeric type
pub struct NumericShrink {}

impl<E> Shrink<E> for NumericShrink
where
    E: Num + Copy + 'static,
{
    fn candidates(self: &NumericShrink, _original: E) -> Box<dyn Iterator<Item = E>> {
        Box::new(NumericShrinkIterator::<E> {
            start: _original,
            target: E::zero(),
            next: Some(E::zero().sub(_original)),
        })
    }
}

/// Iterator for shrinking numerical values
pub struct NumericShrinkIterator<E> {
    start: E,
    target: E,
    next: Option<E>,
}

impl<E> Iterator for NumericShrinkIterator<E>
where
    E: Num + Copy,
{
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.next;

        if self.next.is_some() && self.next.unwrap().eq(&self.target) {
            self.next = None
        }

        result
    }
}

/// Shrinkers for u8-type.
pub fn u8() -> NumericShrinks<u8> {
    NumericShrinks {
        min: u8::MIN,
        max: u8::MAX,
    }
}
