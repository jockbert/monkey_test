//! This is the MonkeyTest implementations of a
//! [Shrinking Challenge](https://github.com/jlink/shrinking-challenge).
//!
//! # About the Reverse challenge
//!
//! This tests the (wrong) property that reversing a list of integers results
//! in the same list. It is a basic example to validate that a library can
//! reliably normalize simple sample data.
//!
//! For details on this challenge, see
//! https://github.com/jlink/shrinking-challenge/blob/main/challenges/reverse.md

use monkey_test::*;

fn faulty_reverse<E>(list: Vec<E>) -> Vec<E> {
    // Not reversing the list, just returning it.
    list
}

/// Should preferably shrink down to two element vector [0,1].
#[test]
fn test_reverse() {
    let result = monkey_test()
        .with_generator(gen::vec::any(gen::i16::any()))
        .test_true(|list| {
            let mut expected = list.clone();
            expected.reverse();

            let actual = faulty_reverse(list);

            expected == actual
        });

    match result {
        MonkeyResult::MonkeyErr {
            minimum_failure,
            original_failure,
            shrink_count,
            some_other_failures,
            ..
        } => {
            println!("original failure: {:?}", original_failure);
            println!("shrink count: {:?}", shrink_count);
            println!("other failures..: {:?}", some_other_failures);
            assert_eq!(minimum_failure, vec![0, 1])
        }
        other => panic!("{:?} is unexpected", other),
    }
}
