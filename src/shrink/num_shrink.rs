use crate::BoxIter;
use crate::BoxShrink;
use crate::Shrink;
use num_traits::Num;

/// Shrink number types towards the value zero.
pub fn to_zero<E>() -> BoxShrink<E>
where
    E: Num + Copy + std::cmp::PartialOrd + 'static,
{
    Box::new(NumShrink {})
}

/// Shrinker that decrements a value towards zero.
#[derive(Clone)]
struct NumShrink {}

impl<E> Shrink<E> for NumShrink
where
    E: Num + Copy + std::cmp::PartialOrd + 'static,
{
    fn candidates(&self, original: E) -> BoxIter<E> {
        Box::new(NumShrinkIt::<E> { current: original })
    }
}

/// Iterator that decrements a value towards zero.
struct NumShrinkIt<E> {
    current: E,
}

impl<E> Iterator for NumShrinkIt<E>
where
    E: Num + Copy + std::cmp::PartialOrd,
{
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == E::zero() {
            None
        } else if self.current < E::zero() {
            self.current = self.current.add(E::one());
            Some(self.current)
        } else {
            self.current = self.current.sub(E::one());
            Some(self.current)
        }
    }
}

#[cfg(test)]
mod test {
    use super::NumShrink;
    use crate::Shrink;

    #[test]
    fn can_shrink_both_positive_and_negative_numbers() {
        let shrink = NumShrink {};

        assert!(shrink.candidates(0).next().is_none());
        assert!(shrink.candidates(1).next().is_some());
        assert!(shrink.candidates(-1).next().is_some());
        assert!(shrink.candidates(i8::MAX).next().is_some());
        assert!(shrink.candidates(i8::MIN).next().is_some());
    }
}
