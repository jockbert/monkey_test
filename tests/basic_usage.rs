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
        .with_shrinker(shrink::number())
        .check_true(|x| x < 15);

    assert_eq!(
        actual_result,
        MonkeyResult::MonkeyErr {
            minimum_failure: 15,
            original_failure: 20,
            some_other_failures: vec!(19, 18, 17, 16),
            success_count: 4,
            shrink_count: 5,
            seed: 123456
        }
    );
}

#[test]
#[should_panic(expected = "Monkey test property failed!\nCounterexample: 15")]
fn can_fail_with_panic_when_using_assert() {
    monkey_test()
        .with_seed(123456)
        .with_generator(gen::fixed::sequence(&[1, 2, 3, 10, 20, 30]))
        .with_shrinker(shrink::number())
        .assert_true(|x| x < 15);
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
