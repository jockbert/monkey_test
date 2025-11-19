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
//! let some_size = 0..=1000;
//! let vectors_of_nine = gens::vec::any(gens::fixed::constant(9));
//!
//! let actual_examples = vectors_of_nine
//!     .examples(some_seed, some_size)
//!     .take(20)
//!     .collect::<Vec<Vec<i32>>>();
//!
//! assert_eq!{
//!     actual_examples,
//!     vec![
//!         vec![],
//!         vec![9],
//!         vec![9],
//!         vec![9, 9],
//!         vec![9, 9],
//!         vec![9, 9, 9, 9, 9],
//!         vec![9, 9],
//!         vec![9, 9, 9, 9, 9, 9],
//!         vec![9, 9, 9, 9],
//!         vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9],
//!         vec![9, 9, 9],
//!         vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9],
//!
//!         vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
//!              9],
//!
//!         vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
//!              9, 9, 9, 9, 9, 9],
//!
//!         vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9],
//!         vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
//!              9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9],
//!
//!         vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
//!              9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
//!              9],
//!
//!         vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
//!              9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
//!              9, 9, 9, 9, 9, 9, 9, 9, 9],
//!
//!         vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
//!              9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
//!              9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
//!              9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9],
//!
//!         vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
//!              9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9]
//!     ]
//! };
//! ```

use crate::BoxGen;

/// Any vector filled with values from given element generator
pub fn any<E: Clone + 'static>(element_gen: BoxGen<E>) -> BoxGen<Vec<E>> {
    let element_shrinker = element_gen.shrinker();

    crate::gens::from_fn(move |seed, size| {
        let sizes = crate::gens::sized::default().examples(seed, size.clone());
        let seeds = crate::gens::seeds().examples(seed, size.clone());
        let element_gen = element_gen.clone();

        sizes.zip(seeds).map(move |(sz, seed)| {
            element_gen
                .examples(seed, size.clone())
                .take(sz)
                .collect::<Vec<_>>()
        })
    })
    .with_shrinker(crate::shrinks::vec::default(element_shrinker))
}
