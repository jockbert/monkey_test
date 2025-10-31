//! Shrinkers for vectors.
//!
//! One design choice of the vector shrinker implementation is to aggressively
//! try to shrink size before individual elements are shrunk.
//!
//! ```rust
//! use monkey_test::*;
//!
//! let failing_example_to_shrink = vec![-1, 2, 3, 4, 5, 6, 7, 8];
//!
//! let int_vectors = gen::vec::any(gen::i16::any());
//!
//! let smaller_candidates =  int_vectors
//!     .shrinker()
//!     .candidates(failing_example_to_shrink)
//!     .take(25)
//!     .collect::<Vec<_>>();
//!
//! assert_eq!{
//!     smaller_candidates,
//!     vec![
//!         // Vector shrinker tries to aggressively shrink size before
//!         // individual elements are shrunk.
//!         vec![                       ],
//!         vec![             5, 6, 7, 8],
//!         vec![-1, 2, 3, 4,           ],
//!         vec![       3, 4, 5, 6, 7, 8],
//!         vec![-1, 2,       5, 6, 7, 8],
//!         vec![-1, 2, 3, 4,       7, 8],
//!         vec![-1, 2, 3, 4, 5, 6,     ],
//!         vec![    2, 3, 4, 5, 6, 7, 8],
//!         vec![-1,    3, 4, 5, 6, 7, 8],
//!         vec![-1, 2,    4, 5, 6, 7, 8],
//!         vec![-1, 2, 3,    5, 6, 7, 8],
//!         vec![-1, 2, 3, 4,    6, 7, 8],
//!         vec![-1, 2, 3, 4, 5,    7, 8],
//!         vec![-1, 2, 3, 4, 5, 6,    8],
//!         vec![-1, 2, 3, 4, 5, 6, 7,  ],
//!         // Shrinking of individul elements starts here.
//!         //    ↓ First element
//!         vec![ 1, 2, 3, 4, 5, 6, 7, 8],
//!         vec![ 0, 2, 3, 4, 5, 6, 7, 8],
//!         //       ↓ Second element
//!         vec![-1, -1, 3, 4, 5, 6, 7, 8],
//!         vec![-1, 1, 3, 4, 5, 6, 7, 8],
//!         vec![-1, 0, 3, 4, 5, 6, 7, 8],
//!         //          ↓ Third element
//!         vec![-1, 2, -2, 4, 5, 6, 7, 8],
//!         vec![-1, 2, 2, 4, 5, 6, 7, 8],
//!         vec![-1, 2, -1, 4, 5, 6, 7, 8],
//!         vec![-1, 2, 1, 4, 5, 6, 7, 8],
//!         vec![-1, 2, 0, 4, 5, 6, 7, 8],
//!         // And further ...
//!     ]
//! };
//! ```

use crate::BoxIter;
use crate::BoxShrink;

/// Default vector shrinker.
pub fn default<E: Clone + 'static>(
    element_shrinker: BoxShrink<E>,
) -> BoxShrink<Vec<E>> {
    crate::shrink::from_fn(move |original: Vec<E>| {
        eager_size(original.clone())
            .chain(per_element(original, element_shrinker.clone()))
    })
}

/// Shrinker that only tries to reduce the vector size, not trying to shrink
/// individual elements.
pub fn no_element_shrinkning<E: Clone + 'static>() -> BoxShrink<Vec<E>> {
    crate::shrink::from_fn(move |original: Vec<E>| eager_size(original))
}

fn eager_size<E>(original: Vec<E>) -> EagerIterator<E> {
    EagerIterator {
        len_to_remove: original.len(),
        original,
        start_of_remove: 0,
    }
}

/// Eager vector element removal iterator
struct EagerIterator<E> {
    original: Vec<E>,
    len_to_remove: usize,
    start_of_remove: usize,
}

impl<E> EagerIterator<E> {
    fn half_length_to_remove(&mut self) {
        self.len_to_remove = match self.len_to_remove {
            usize::MAX => usize::MAX / 2,
            1 => 0,
            _ => self.len_to_remove.div_ceil(2),
        }
    }

    fn advance(&mut self) {
        self.start_of_remove += self.len_to_remove;
        if self.start_of_remove >= self.original.len() {
            self.half_length_to_remove();
            self.start_of_remove = 0;
        }
    }
}

impl<E> Iterator for EagerIterator<E>
where
    E: Clone,
{
    type Item = Vec<E>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len_to_remove == 0 {
            None
        } else {
            let start_of_remove = self.start_of_remove;
            let first = self.original.clone().into_iter().take(start_of_remove);

            let end_of_remove = self.start_of_remove + self.len_to_remove;
            let rest = self.original.clone().into_iter().skip(end_of_remove);

            let candidate = first.chain(rest).collect::<Vec<_>>();

            self.advance();

            Some(candidate)
        }
    }
}

/// Per element shrink iterator
fn per_element<E>(
    original: Vec<E>,
    elem_shrinker: BoxShrink<E>,
) -> BoxIter<Vec<E>>
where
    E: Clone + 'static,
{
    let candidate_vec = original.clone();

    original
        .clone()
        .iter()
        .enumerate()
        .map(|(index, elem)| {
            let cv2 = candidate_vec.clone();
            let idx = index;

            elem_shrinker.candidates(elem.clone()).take(1000).map(
                move |candidate| {
                    let mut vec = cv2.clone();
                    let _ = std::mem::replace(&mut vec[idx], candidate);
                    vec
                },
            )
        })
        .fold(Box::new(std::iter::empty::<Vec<E>>()), |acc, it| {
            let x: BoxIter<Vec<E>> = Box::new(acc.chain(it));
            x
        })
}

#[cfg(test)]
mod test {
    use crate::testing::assert_iter_eq;

    #[test]
    pub fn eager_removes_all_and_then_iteratively_smaller_parts() {
        assert_iter_eq(
            super::eager_size(vec![1, 2, 3, 4, 5, 6, 7, 8]),
            vec![
                // everything is removed
                vec![],
                // 1st half removed
                vec![5, 6, 7, 8],
                // 2nd half removed
                vec![1, 2, 3, 4],
                // 1st quarter removed
                vec![3, 4, 5, 6, 7, 8],
                // 2nd quarter removed
                vec![1, 2, 5, 6, 7, 8],
                // 3rd quarter removed
                vec![1, 2, 3, 4, 7, 8],
                // 4th quarter removed
                vec![1, 2, 3, 4, 5, 6],
                // 1st eight removed
                vec![2, 3, 4, 5, 6, 7, 8],
                // 2nd eight removed
                vec![1, 3, 4, 5, 6, 7, 8],
                // 3rd eight removed
                vec![1, 2, 4, 5, 6, 7, 8],
                // 4th eight removed
                vec![1, 2, 3, 5, 6, 7, 8],
                // 5th eight removed
                vec![1, 2, 3, 4, 6, 7, 8],
                // 6th eight removed
                vec![1, 2, 3, 4, 5, 7, 8],
                // 7th eight removed
                vec![1, 2, 3, 4, 5, 6, 8],
                // 8th eight removed
                vec![1, 2, 3, 4, 5, 6, 7],
            ],
            "removes iteratively smaller parts of original failing vector",
        )
    }

    #[test]
    pub fn eager_can_handle_odd_sizes() {
        assert_iter_eq(
            super::eager_size(vec![1, 2, 3, 4, 5]),
            vec![
                // everything is removed
                vec![],
                // 2nd iteration step 1, 3 elements (=5/2 rounded up) removed
                vec![4, 5],
                // 2nd iteration step 2. 2 reamining elements removed
                vec![1, 2, 3],
                // 3rd iteration step 1, 2 eleemnts (=3/2 rounded up) removed
                vec![3, 4, 5],
                // 3rd iteration step 2, 2 elements removed
                vec![1, 2, 5],
                // 3rd iteration step 3, 1 remaining element removed
                vec![1, 2, 3, 4],
                // 4th iteration step 1, 1 element (22/2 rounded up) removed
                vec![2, 3, 4, 5],
                // 4th iteration step 2
                vec![1, 3, 4, 5],
                // 4th iteration step 3
                vec![1, 2, 4, 5],
                // 4th iteration step 4
                vec![1, 2, 3, 5],
                // 4th iteration step 5
                vec![1, 2, 3, 4],
            ],
            "partition odd vector sizes",
        )
    }

    #[test]
    pub fn eager_can_handle_size_one() {
        assert_iter_eq(
            super::eager_size(vec![1]),
            vec![
                // everything is removed
                vec![],
            ],
            "only size zero is a shrunken size one",
        )
    }

    #[test]
    pub fn eager_can_handle_size_zero() {
        assert_iter_eq(
            super::eager_size(Vec::<u8>::new()),
            Vec::<Vec<_>>::new(),
            "shrinking empty vector givies no shrinking candidated",
        );
    }

    #[test]
    fn per_element_shrinker_tries_every_element() {
        assert_iter_eq(
            super::per_element(
                vec![1, 2, 3, 4],
                crate::shrink::fixed::sequence(&[0]),
            ),
            vec![
                // shrinking 1st element
                vec![0, 2, 3, 4],
                // shrinking 2nd element
                vec![1, 0, 3, 4],
                // shrinking 3rd element
                vec![1, 2, 0, 4],
                // shrinking 4th element
                vec![1, 2, 3, 0],
            ],
            "per element shrinker tries to shrink every element, in \
            this case using zero",
        )
    }
}
