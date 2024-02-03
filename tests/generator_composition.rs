//! These tests tries to shows how to combine simple type generators into
//! generators of more complex types.

use monkey_test::*;

fn no_big_square_tuple((width, height): (u16, u16)) -> bool {
    height < 50 || height != width
}

fn heights() -> BoxGen<u16> {
    gen::u16::ranged(..100)
}

fn widths() -> BoxGen<u16> {
    gen::u16::ranged(..100)
}

/// Zipping two generators into generator of tuples automatically also creates
/// appropriate shrinker too.
fn rectangle_tuples() -> BoxGen<(u16, u16)> {
    // Combining two generators with zip
    heights().zip(widths())
}

#[test]
fn zip_and_automatic_shrinking() {
    monkey_test()
        .with_generator(rectangle_tuples())
        .test_property(no_big_square_tuple)
        // The minimum test case (50, 50) would be hard to
        // find directly without a proper shrinker.
        .assert_minimum_failure((50, 50));
}
