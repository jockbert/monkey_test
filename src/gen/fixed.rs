//! Generators mainly used for internal testing, where you want a
//! deterministicly generated values.

/// Generates a fixed sequence of examples.
///
/// ```
/// use crate::monkey_test::Gen;
///
/// let gen = monkey_test::gen::fixed::sequence::<u64>(&[1,20,300]);
/// let mut it = gen.iter(1337);
/// assert_eq!(Some(1), it.next());
/// assert_eq!(Some(20), it.next());
/// assert_eq!(Some(300), it.next());
/// assert_eq!(None, it.next());
/// assert_eq!(None, it.next());
/// assert_eq!(None, it.next());
/// ```
pub fn sequence<T>(examples: &[T]) -> SliceGen<T>
where
    T: Clone + std::fmt::Debug,
{
    SliceGen::new(examples)
}

/// Generator from a given set of examples to return.
#[derive(Clone)]
pub struct SliceGen<T> {
    data: Vec<T>,
}

impl<T> SliceGen<T>
where
    T: Clone,
{
    fn new(data: &[T]) -> Self {
        SliceGen {
            data: Vec::from(data),
        }
    }
}

impl<T: Clone + 'static> crate::Gen<T> for SliceGen<T> {
    fn iter(&self, _seed: u64) -> crate::SomeIter<T> {
        let x = self.data.clone();
        Box::new(x.into_iter())
    }
}
