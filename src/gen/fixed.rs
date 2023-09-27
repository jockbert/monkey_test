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
pub fn sequence<E>(examples: &[E]) -> SliceGen<E>
where
    E: Clone + std::fmt::Debug,
{
    SliceGen::new(examples)
}

/// Generator from a given set of examples to return.
#[derive(Clone)]
pub struct SliceGen<E> {
    data: Vec<E>,
}

impl<E> SliceGen<E>
where
    E: Clone,
{
    fn new(data: &[E]) -> Self {
        SliceGen {
            data: Vec::from(data),
        }
    }
}

impl<E: Clone + 'static> crate::Gen<E> for SliceGen<E> {
    fn iter(&self, _seed: u64) -> crate::SomeIter<E> {
        let x = self.data.clone();
        Box::new(x.into_iter())
    }
}
