use monkey_test::*;

#[test]
fn add_up_to_overflow() {
    monkey_test()
        .with_generator(gen::u8::any())
        .assert_true(|x| x == 255 || x + 1 > x);
}

#[test]
fn can_fail_with_details_when_using_check() {
    let actual_result: MonkeyResult<u8> = monkey_test()
        .with_seed(123456)
        .with_generator(gen::fixed::sequence(&[1, 2, 3, 10, 20, 30]))
        .with_shrinker(shrink::int())
        .title("Less than thirteen")
        .test_property(|x| x < 13);

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
#[should_panic(expected = "Monkey test property failed!\nFailure: 15")]
fn can_fail_with_panic_when_using_assert() {
    monkey_test()
        .with_seed(123456)
        .with_generator(gen::fixed::sequence(&[1, 2, 3, 10, 20, 30]))
        .with_shrinker(shrink::int())
        .assert_true(|x| x < 15);
}

/// Can do the same as above by asserting minimum failing example
#[test]
fn can_assert_minimumfail_with_panic_when_using_assert() {
    monkey_test()
        .with_seed(123456)
        .with_generator(gen::fixed::sequence(&[1, 2, 3, 10, 20, 30]))
        .with_shrinker(shrink::int())
        .test_property(|x| x < 15)
        .assert_minimum_failure(15);
}

#[test]
#[should_panic(
    expected = "Reason: Expecting no panic, but got panic \"attempt to divide by zero\""
)]
fn can_assert_that_there_is_no_panic_thrown() {
    monkey_test()
        .with_example_count(1_000)
        .with_generator(gen::u32::any())
        .assert_no_panic(|n| {
            let _ = 1 / (n / u32::MAX);
        });
}

#[test]
fn use_all_settings_available() {
    monkey_test()
        .with_example_count(1_000)
        .with_seed(1234567890)
        .with_generator(gen::u8::any())
        .with_shrinker(shrink::none())
        .assert_true(|x| x as u16 * x as u16 >= x as u16);
}

#[test]
fn pick_from_alternatives_evenly() {
    monkey_test()
        .with_generator(gen::pick_evenly(&["Apple", "Orange", "Banana"]))
        .assert_true(|fruit| fruit.ends_with('e') || fruit.ends_with('a'));
}

#[test]
fn pick_from_alternatives_with_ratio() {
    monkey_test()
        .with_generator(gen::pick_with_ratio(&[
            (1, "Apple"),
            (2, "Orange"),
            (55, "Banana"),
        ]))
        .assert_true(|fruit| fruit.ends_with('e') || fruit.ends_with('a'));
}

#[test]
fn mix_from_alternative_generators_with_ratio() {
    let evens = gen::pick_evenly(&[0, 2, 4, 6, 8]);
    let odds = gen::pick_evenly(&[1, 3, 5, 7, 9]);

    let mostly_evens = gen::mix_with_ratio(&[(93, evens), (7, odds)]);

    monkey_test()
        .with_generator(mostly_evens)
        .assert_true(|number| (0..10).contains(&number));
}
