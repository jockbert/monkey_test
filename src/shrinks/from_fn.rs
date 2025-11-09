use crate::BoxIter;
use crate::BoxShrink;
use crate::Shrink;
use std::marker::PhantomData;

/// Create a new shrinker where each request for candidates-iterator calls the
/// provided closure `F: Fn(E) -> Iterator<Item=E> + Clone + 'static`.
///
/// For alternatives, see [from_fn_boxed].
///
///
/// ```rust
/// use monkey_test::*;
///
/// // Creating a shrinker by providing closure returning an iterator.
/// let my_shrink = shrinks::from_fn(|original_failure| std::iter::repeat(42));
///
/// assert_eq!(my_shrink.candidates(1337).next(), Some(42));
/// ```
///
pub fn from_fn<E, I, F>(f: F) -> BoxShrink<E>
where
    E: Clone + 'static,
    I: Iterator<Item = E> + 'static,
    F: Fn(E) -> I + Clone + 'static,
{
    Box::new(FromFnShrink {
        e: PhantomData,
        f: move |original| Box::new((f)(original)),
    })
}

/// Create a new shrinker where each request for candidates-iterator calls the
/// provided closure `F: Fn(E) -> BoxIter<E> + Clone + 'static`.
///
/// This function does the same thing as [from_fn], but with the exception that
/// the returned iterator must be boxed, as in being a trait object. This can
/// be convenient when the closure want to return one of several different
/// iterator implementations, hence iterator type being unclear.
///
/// For more details see [from_fn].
pub fn from_fn_boxed<E, F>(f: F) -> BoxShrink<E>
where
    E: Clone + 'static,
    F: Fn(E) -> BoxIter<E> + Clone + 'static,
{
    Box::new(FromFnShrink { e: PhantomData, f })
}

#[derive(Clone)]
struct FromFnShrink<E, F>
where
    E: Clone + 'static,
    F: Fn(E) -> BoxIter<E> + Clone + 'static,
{
    e: PhantomData<E>,
    f: F,
}

impl<E, F> Shrink<E> for FromFnShrink<E, F>
where
    E: Clone + 'static,
    F: Fn(E) -> BoxIter<E> + Clone + 'static,
{
    fn candidates(&self, original: E) -> BoxIter<E> {
        (self.f)(original)
    }
}
