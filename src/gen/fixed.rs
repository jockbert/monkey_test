//! Generators mainly used for internal testing, where you want a
//! deterministicly generated values.

use crate::{BoxGen, BoxShrink, Gen};

/// Generates a fixed sequence of examples.
///
/// ```
/// use crate::monkey_test::Gen;
///
/// let gen = monkey_test::gen::fixed::sequence::<u64>(&[1,20,300]);
///
/// let mut ex1 = gen.examples(1337);
/// assert_eq!(Some(1), ex1.next());
/// assert_eq!(Some(20), ex1.next());
/// assert_eq!(Some(300), ex1.next());
/// assert_eq!(None, ex1.next());
/// assert_eq!(None, ex1.next());
/// assert_eq!(None, ex1.next());
///
/// let mut ex2 = gen.examples(42);
/// assert_eq!(Some(1), ex2.next());
/// assert_eq!(Some(20), ex2.next());
/// assert_eq!(Some(300), ex2.next());
/// assert_eq!(None, ex2.next());
/// ```
pub fn sequence<E>(examples: &[E]) -> BoxGen<E>
where
    E: Clone + std::fmt::Debug + 'static,
{
    Box::new(SequenceGen::new(examples))
}

/// Generator from a given set of examples to return.
#[derive(Clone)]
pub struct SequenceGen<E> {
    data: Vec<E>,
}

impl<E> SequenceGen<E>
where
    E: Clone,
{
    fn new(data: &[E]) -> Self {
        SequenceGen {
            data: Vec::from(data),
        }
    }
}

impl<E: Clone + 'static> Gen<E> for SequenceGen<E> {
    fn examples(&self, _seed: u64) -> crate::BoxIter<E> {
        let x = self.data.clone();
        Box::new(x.into_iter())
    }

    fn shrinker(&self) -> BoxShrink<E> {
        crate::shrink::none()
    }
}
