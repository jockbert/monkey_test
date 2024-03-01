//! Generators mainly used for internal testing, where you want a
//! deterministicly generated values.

use crate::{BoxGen, BoxIter, BoxShrink, Gen};

/// Generates a fixed sequence of examples and then ends, having no more values.
///
/// ```
/// let gen = monkey_test::gen::fixed::sequence(&[1, 20, 300]);
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
    Box::new(SequenceGen {
        data: examples.to_vec(),
    })
}

/// Generator from a given set of examples to return.
#[derive(Clone)]
struct SequenceGen<E> {
    data: Vec<E>,
}

impl<E: Clone + 'static> Gen<E> for SequenceGen<E> {
    fn examples(&self, _seed: u64) -> BoxIter<E> {
        let x = self.data.clone();
        Box::new(x.into_iter())
    }

    fn shrinker(&self) -> BoxShrink<E> {
        crate::shrink::none()
    }
}

/// Generates a fixed loop of examples. This generator is convenient when you,
/// for instance, want to know the exact distribution of the generator. The
/// distribition is not always exact when dealing with randomness.
///
/// ```
/// let looper = monkey_test::gen::fixed::in_loop(&[1, 20, 300]);
///
/// let examples = looper.examples(42).take(10).collect::<Vec<_>>();
/// assert_eq!(examples, vec![1, 20, 300, 1, 20, 300, 1, 20, 300, 1]);
///
/// // Returns the same examples when using other seed
/// let other_seed = looper.examples(1337).take(10).collect::<Vec<_>>();
/// assert_eq!(examples, other_seed);
/// ```
pub fn in_loop<E>(examples: &[E]) -> BoxGen<E>
where
    E: Clone + 'static,
{
    Box::new(LoopGen {
        data: examples.to_vec(),
    })
}

/// Generator from a given set of examples to return in loop.
#[derive(Clone)]
struct LoopGen<E> {
    data: Vec<E>,
}

impl<E: Clone + 'static> Gen<E> for LoopGen<E> {
    fn examples(&self, _seed: u64) -> BoxIter<E> {
        let x = self.data.clone();
        Box::new(LoopIter { data: x, index: 0 })
    }

    fn shrinker(&self) -> BoxShrink<E> {
        crate::shrink::none()
    }
}

/// Generator from a given set of examples to return in loop.
#[derive(Clone)]
struct LoopIter<E> {
    data: Vec<E>,
    index: usize,
}

impl<E: Clone + 'static> Iterator for LoopIter<E> {
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        let index_to_use = self.index;
        self.index = (self.index + 1) % self.data.len();
        self.data.get(index_to_use).cloned()
    }
}

/// Infinite generator always returning given constant
pub fn constant<E>(example: E) -> BoxGen<E>
where
    E: Clone + 'static,
{
    Box::new(LoopGen {
        data: vec![example],
    })
}
