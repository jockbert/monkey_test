use crate::BoxGen;
use crate::BoxIter;
use crate::ExampleSize;
use crate::Gen;

/// Create a new generator where each request for examples-iterator calls the
/// provided closure `F: Fn(u64, ExampleSize) -> Iterator<Item=E> + Clone + 'static`.
///
/// The argument to the closure is the randomisation seed provided by
/// Monkey Test.
///
/// Please note! No shrinker is associated with the resulting generator, so
/// shrinker need to be provided too. That can be done by using either
/// [Gen::with_shrinker] or [crate::gens::other_shrinker], alternatively provide
/// at place of testing the propery with [crate::ConfAndGen::with_shrinker]
///
/// For alternatives, see [from_fn_boxed].
///
/// ```rust
/// use monkey_test::*;
///
/// // Creating a generator by providing closure returning an iterator.
/// let my_gen = gens::from_fn(|seed, _size| std::iter::repeat(42));
///
/// // First alternative for providing a shrinker - attaching it to the generator.
/// let my_shrinking_gen = my_gen.with_shrinker(shrinks::int());
/// monkey_test()
///     .with_generator(my_shrinking_gen)
///     .test_property(|n| n <= 10)
///     .assert_minimum_failure(11);
///
/// // Second alternative for providing a shrinker - explicitly providing it at
/// // point of property testing.
/// monkey_test()
///     .with_generator(my_gen)
///     .with_shrinker(shrinks::int())
///     .test_property(|n| n <= 10)
///     .assert_minimum_failure(11);
/// ```
///
pub fn from_fn<E, I, F>(f: F) -> BoxGen<E>
where
    E: Clone + 'static,
    I: Iterator<Item = E> + 'static,
    F: Fn(u64, ExampleSize) -> I + Clone + 'static,
{
    Box::new(FromFnGen {
        f: move |seed, size| Box::new((f)(seed, size)),
    })
}

/// Create a new generator where each request for examples-iterator calls the
/// provided closure `F: Fn(u64, ExampleSize) -> BoxIter<E> + Clone + 'static`.
///
/// This function does the same thing as [from_fn], but with the exception that
/// the returned iterator must be boxed, as in being a trait object. This can
/// be convenient when the closure want to return one of several different
/// iterator implementations, hence iterator type being unclear.
///
/// For more details see [from_fn].
///
/// ```rust
/// use monkey_test::*;
///
/// // Creating a generator by providing closure returning a boxed iterator.
/// let my_gen: BoxGen<i64> =
///     gens::from_fn_boxed(|seed, _size| Box::new(std::iter::repeat(42)));
/// ```
///
pub fn from_fn_boxed<E, F>(f: F) -> BoxGen<E>
where
    E: Clone + 'static,
    F: Fn(u64, ExampleSize) -> BoxIter<E> + Clone + 'static,
{
    Box::new(FromFnGen { f })
}

#[derive(Clone)]
struct FromFnGen<E, F>
where
    E: Clone + 'static,
    F: Fn(u64, ExampleSize) -> BoxIter<E> + Clone + 'static,
{
    f: F,
}

impl<E, F> Gen<E> for FromFnGen<E, F>
where
    E: Clone + 'static,
    F: Fn(u64, ExampleSize) -> BoxIter<E> + Clone + 'static,
{
    fn examples(&self, seed: u64, size: ExampleSize) -> BoxIter<E> {
        (self.f)(seed, size)
    }

    fn shrinker(&self) -> crate::BoxShrink<E> {
        crate::shrinks::none()
    }
}
