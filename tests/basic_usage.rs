use monkey_test::*;

#[test]
fn add_up_to_overflow() {
    monkey_test()
        .with_generator(gens::u8::any())
        .assert_true(|x| x == 255 || x + 1 > x);
}

#[test]
fn can_fail_with_details_when_using_check() {
    let actual_result: MonkeyResult<u8> = monkey_test()
        .with_seed(123456)
        .with_generator(gens::fixed::sequence(&[1, 2, 3, 10, 20, 30]))
        .with_shrinker(shrinks::int_to_zero())
        .title("Less than thirteen")
        .test_true(|x| x < 13);

    assert_eq!(
        actual_result,
        MonkeyResult::MonkeyErr {
            minimum_failure: 13,
            original_failure: 20,
            some_other_failures: vec!(15, 14),
            success_count: 4,
            shrink_count: 3,
            seed: 123456,
            title: Some("Less than thirteen".into()),
            reason: "Expecting 'true' but got 'false'.".into(),
        }
    );
}

#[test]
#[should_panic(expected = "Monkey test property failed!\nFailure: 13")]
fn can_fail_with_panic_when_using_assert() {
    monkey_test()
        .with_seed(123456)
        .with_generator(gens::fixed::sequence(&[1, 2, 3, 10, 20, 30]))
        .with_shrinker(shrinks::int_to_zero())
        .assert_true(|x| x < 13);
}

/// Can do the same as above by asserting minimum failing example
#[test]
fn can_assert_minimumfail_with_panic_when_using_assert() {
    monkey_test()
        .with_seed(123456)
        .with_generator(gens::fixed::sequence(&[1, 2, 3, 10, 20, 30]))
        .with_shrinker(shrinks::int_to_zero())
        .test_true(|x| x < 15)
        .assert_minimum_failure(15);
}

#[test]
#[should_panic(
    expected = "Reason: Expecting no panic, but got panic \"attempt to divide by zero\""
)]
fn can_assert_that_there_is_no_panic_thrown() {
    monkey_test()
        .with_example_count(1_000)
        .with_generator(gens::u32::any())
        .assert_no_panic(|n| {
            let _ = 1 / (n / u32::MAX);
        });
}

/// Assert for equality prints out expected and actual values when failing.
/// Things to note:
/// 1. The first argument in `[monkey_test::ConfAndGen::assert_eq]` is the
///    expected value and the second argument is the actual value tested.
///    This may not be the usual rust `assert_eq!` macro convention, but is
///    common in other test frameworks, like
///    [in JUnit](https://junit.org/junit5/docs/5.0.1/api/org/junit/jupiter/api/Assertions.html).
/// 2. When property is failing, all shrinked values are kept in the same range
///    as given in the generator, namely 10 or larger. That is why 11 is the
///    smallest failure.
#[test]
#[should_panic(expected = "Monkey test property failed!\n\
    Failure: 11\n\
    Reason: Actual value should equal expected 11, but got 10.")]
fn can_assert_eq() {
    monkey_test()
        .with_generator(gens::u32::ranged(10..))
        .assert_eq(|n| n, |n| n / 2 * 2);
}

/// Shows how to test for the proper of values being not equal, here shown as a
/// faulty property due to positive odd numbers divided by two are rounded down.
///
/// Here for n=0, both 2/2=1 and 3/2=1.
#[test]
#[should_panic(
    expected = "Reason: Actual value should not equal expected 1, but got 1."
)]
fn can_assert_ne() {
    monkey_test()
        .with_example_count(1_000)
        .with_generator(gens::u32::ranged(0..10_000))
        .assert_ne(|n| (n + 2) / 2, |n| (n + 3) / 2);
}

#[test]
fn use_all_settings_available() {
    monkey_test()
        .with_example_count(1_000)
        .with_example_size(..50)
        .with_seed(1234567890)
        .with_generator(gens::u8::any())
        .with_shrinker(shrinks::none())
        .title("square of x is equal or greater than x")
        .assert_true(|x| x as u16 * x as u16 >= x as u16);
}

#[test]
fn pick_from_alternatives_evenly() {
    monkey_test()
        .with_generator(gens::pick_evenly(&["Apple", "Orange", "Banana"]))
        .assert_true(|fruit| fruit.ends_with('e') || fruit.ends_with('a'));
}

#[test]
fn pick_from_alternatives_with_ratio() {
    monkey_test()
        .with_generator(gens::pick_with_ratio(&[
            (1, "Apple"),
            (2, "Orange"),
            (55, "Banana"),
        ]))
        .assert_true(|fruit| fruit.ends_with('e') || fruit.ends_with('a'));
}

#[test]
fn mix_from_alternative_generators_with_ratio() {
    let evens = gens::pick_evenly(&[0, 2, 4, 6, 8]);
    let odds = gens::pick_evenly(&[1, 3, 5, 7, 9]);

    let mostly_evens = gens::mix_with_ratio(&[(93, evens), (7, odds)]);

    monkey_test()
        .with_generator(mostly_evens)
        .assert_true(|number| (0..10).contains(&number));
}

/// Adding filter to generator, also reuses the filtering in shrinking.
#[test]
fn filtering_of_generated_values() {
    monkey_test()
        .with_generator(
            gens::u8::any()
                // only odd numbers
                .filter(|e| e % 2 == 1)
                // only numbers equal or greater to 100
                .filter(|&e| e >= 100u8),
        )
        .test_true(|_example| false)
        .assert_minimum_failure(101);
}
