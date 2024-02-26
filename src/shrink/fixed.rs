//! Shrinkers mainly used for internal testing or debugging, where you want a
//! deterministicly provided candidate values.

use crate::{BoxShrink, Shrink};

/// Provides a fixed sequence of candidates and then ends, having no more values.
pub fn sequence<E>(candidates: &[E]) -> BoxShrink<E>
where
    E: Clone + std::fmt::Debug + 'static,
{
    Box::new(SequenceShrink {
        data: candidates.to_vec(),
    })
}

#[derive(Clone)]
struct SequenceShrink<E> {
    data: Vec<E>,
}

impl<E> Shrink<E> for SequenceShrink<E>
where
    E: std::clone::Clone + std::fmt::Debug + 'static,
{
    fn candidates(&self, _original: E) -> crate::BoxIter<E> {
        Box::new(self.data.clone().into_iter())
    }
}
