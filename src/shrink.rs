//! The `shrink` module contains built in shrinkers.

pub mod vec;

use num_traits::Num;

/// A collection of shrinkers for numeric type T.
pub struct NumericShrinks<T>
where
    T: Num + Copy,
{
    min: T,
    max: T,
}

impl<T> NumericShrinks<T>
where
    T: Num + Copy + 'static,
{
    /// Shrinks a value to zero.
    pub fn to_zero(&self) -> crate::SomeShrink<T> {
        Box::new(NumericShrink {})
    }

    /// Shrinker not producing any smaller values.
    pub fn no_shrink(&self) -> crate::SomeShrink<T> {
        Box::new(NumericShrink {})
    }

    /// Shrinks a value to zero.
    pub fn decrement(&self) -> crate::SomeShrink<T> {
        Box::new(NumDecrementShrink {})
    }
}

/// Shrinker that decrements a value towards zero.
pub struct NumDecrementShrink {}

impl<T> super::Shrink<T> for NumDecrementShrink
where
    T: Num + Copy + 'static,
{
    fn candidates(&self, original: T) -> Box<dyn Iterator<Item = T>> {
        let _next = match original {
            x if x == T::zero() => None,
            _ => Some(original.sub(T::one())),
        };

        Box::new(NumDecrementIterator::<T> { current: original })
    }
}

/// Iterator that decrements a value towards zero.
pub struct NumDecrementIterator<T> {
    current: T,
}

impl<T> Iterator for NumDecrementIterator<T>
where
    T: Num + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == T::zero() {
            None
        } else {
            self.current = self.current.sub(T::one());
            Some(self.current)
        }
    }
}

/// A shrinker for numeric type
pub struct NumericShrink {}

impl<T> super::Shrink<T> for NumericShrink
where
    T: Num + Copy + 'static,
{
    fn candidates(self: &NumericShrink, _original: T) -> Box<dyn Iterator<Item = T>> {
        Box::new(NumericShrinkIterator::<T> {
            start: _original,
            target: T::zero(),
            next: Some(T::zero().sub(_original)),
        })
    }
}

/// Iterator for shrinking numerical values
pub struct NumericShrinkIterator<T> {
    start: T,
    target: T,
    next: Option<T>,
}

impl<T> Iterator for NumericShrinkIterator<T>
where
    T: Num + Copy,
{
    type Item = T;

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
