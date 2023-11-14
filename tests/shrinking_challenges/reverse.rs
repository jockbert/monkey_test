//! This tests the (wrong) property that reversing a list of integers results
//! in the same list. It is a basic example to validate that a library can
//! reliably normalize simple sample data.
//!
//! See https://github.com/jlink/shrinking-challenge/blob/main/challenges/distinct.md
//! for details on this challenge.

use monkey_test::*;

fn faulty_reverse<E>(list: Vec<E>) -> Vec<E> {
    // Not reversing the list, just returning it.
    list
}

/// Should preferably shrink down to vector [0,1], but for now at least
/// shrinks down to two elements.
#[test]
fn test_reverse() {
    let result = monkey_test()
        .with_generator(gen::vec::any(gen::u8::any()))
        .check_true(|list| {
            let mut expected = list.clone();
            expected.reverse();

            let actual = faulty_reverse(list);
            expected == actual
        });

    match result {
        MonkeyResult::MonkeyErr {
            minimum_failure, ..
        } => {
            assert_eq!(minimum_failure.len(), 2)
            // assert_eq!(minimum_failure, vec![1, 0])
        }
        other => panic!("{:?} is unexpected", other),
    }
}
