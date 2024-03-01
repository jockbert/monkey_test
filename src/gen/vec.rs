//! Generators for vectors.
//!
//! One design choice of the vector generator implementation is that example
//! vectors are expected to over time be potentially longer and longer,
//! when iterating over the example iterator.
//!
//! ```rust
//! use monkey_test::*;
//!
//! let some_seed = 1337;
//! let vectors_of_nine = gen::vec::any(gen::fixed::constant(9));
//!
//! let actual_examples = vectors_of_nine
//!     .examples(some_seed)
//!     .take(20)
//!     .collect::<Vec<Vec<i32>>>();
//!
//! assert_eq!{
//!     actual_examples,
//!     vec![
//!         vec![],
//!         vec![9],
//!         vec![9, 9],
//!         vec![9, 9, 9],
//!         vec![9, 9, 9, 9],
//!         vec![9, 9, 9],
//!         vec![9, 9, 9, 9, 9, 9],
//!         vec![9, 9, 9, 9, 9],
//!         vec![9, 9, 9, 9, 9, 9],
//!         vec![9, 9, 9],
//!         vec![9, 9, 9, 9, 9, 9, 9],
//!         vec![9, 9, 9],
//!         vec![9],
//!         vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9],
//!         vec![9],
//!
//!         vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9],
//!
//!         vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
//!              9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
//!              9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9],
//!
//!         vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
//!              9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
//!              9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
//!              9],
//!
//!         vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
//!              9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
//!              9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
//!              9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
//!              9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9],
//!
//!         vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9]
//!     ]
//! };
//! ```

use crate::BoxGen;
use crate::BoxIter;
use crate::BoxShrink;
use crate::Gen;

/// Any vector filled with values from given element generator
pub fn any<E: Clone + 'static>(element_gen: BoxGen<E>) -> BoxGen<Vec<E>> {
    Box::new(VecGen::<E> { element_gen })
}

#[derive(Clone)]
struct VecGen<E> {
    element_gen: BoxGen<E>,
}

impl<E> Gen<Vec<E>> for VecGen<E>
where
    E: Clone + 'static,
{
    fn examples(&self, seed: u64) -> BoxIter<Vec<E>> {
        let sizes = crate::gen::sized::default().examples(seed);
        let seeds = crate::gen::seeds().examples(seed);
        let element_gen = self.element_gen.clone();

        let vectors = sizes.zip(seeds).map(move |(size, seed)| {
            element_gen.examples(seed).take(size).collect::<Vec<_>>()
        });

        Box::new(vectors)
    }

    fn shrinker(&self) -> BoxShrink<Vec<E>> {
        crate::shrink::vec::default(self.element_gen.shrinker())
    }
}
