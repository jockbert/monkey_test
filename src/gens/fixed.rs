//! Generators mainly used for internal testing, where you want a
//! deterministicly generated values.

use crate::BoxGen;

/// Generates a fixed sequence of examples and then ends, having no more values.
///
/// ```
/// let generator = monkey_test::gens::fixed::sequence(&[1, 20, 300]);
///
/// assert_eq!(generator.examples(1337).collect::<Vec<_>>(), vec![1, 20, 300]);
/// assert_eq!(generator.examples(42).collect::<Vec<_>>(), vec![1, 20, 300]);
/// ```
pub fn sequence<E>(examples: &[E]) -> BoxGen<E>
where
    E: Clone + std::fmt::Debug + 'static,
{
    let example_vec = examples.to_vec();
    crate::gens::from_fn(move |_seed| example_vec.clone().into_iter())
}

/// Infinite generator always returning given constant
pub fn constant<E: Clone + 'static>(example: E) -> BoxGen<E> {
    crate::gens::from_fn(move |_seed| std::iter::repeat(example.clone()))
}

/// Generates a fixed loop of examples. This generator is convenient when you,
/// for instance, want to know the exact distribution of the generator. The
/// distribition is not always exact when dealing with randomness.
///
/// ```
/// let looper = monkey_test::gens::fixed::in_loop(&[1, 20, 300]);
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
    let examples_vec = examples.to_vec().clone();
    crate::gens::from_fn(move |_seed| {
        let x = examples_vec.clone();
        LoopIter { data: x, index: 0 }
    })
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
