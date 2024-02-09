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

#[derive(Clone, Debug, PartialEq)]
struct Rectangle {
    width: u16,
    height: u16,
}

/// Mapping a generator into generator of structs automatically also creates
/// appropriate shrinker, if an unmapping function is provided too. The
/// unmapping is needed to convert the struct back to the original pre-mapping
/// type, that (hopefully) already have good shrinker defined.
fn rectangle_structs() -> BoxGen<Rectangle> {
    // Combining two generators with map and unmap.
    gen::map(
        rectangle_tuples(),
        |(width, height)| Rectangle { width, height },
        |r: Rectangle| (r.width, r.height),
    )
}

fn no_big_square_struct(r: Rectangle) -> bool {
    r.height < 50 || r.height != r.width
}

#[test]
fn map_and_automatic_shrinking() {
    monkey_test()
        .with_generator(rectangle_structs())
        .test_property(no_big_square_struct)
        // The minimum test case Rectangle{height:50, width:50} would be hard
        // to find directly without a proper shrinker.
        .assert_minimum_failure(Rectangle {
            width: 50,
            height: 50,
        });
}
