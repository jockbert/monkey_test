use num_traits::Num;

use crate::{Shrink, SomeIter};

/// Shrinker that decrements a value towards zero.
#[derive(Clone)]
pub struct NumShrink {}

impl<E> Shrink<E> for NumShrink
where
    E: Num + Copy + 'static,
{
    fn candidates(&self, original: E) -> SomeIter<E> {
        let _next = match original {
            x if x == E::zero() => None,
            _ => Some(original.sub(E::one())),
        };

        Box::new(NumShrinkIt::<E> { current: original })
    }
}

/// Iterator that decrements a value towards zero.
pub struct NumShrinkIt<E> {
    current: E,
}

impl<E> Iterator for NumShrinkIt<E>
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
