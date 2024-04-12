//! These tests tries to shows how to combine simple type generators into
//! generators of more complex types.

use monkey_test::*;

fn not_too_wide_tuple((width, height): (u16, u16)) -> bool {
    height < 50 || height >= width
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
fn zip_and_automatic_shrinking_of_rectangle() {
    monkey_test()
        .with_generator(rectangle_tuples())
        .test_true(not_too_wide_tuple)
        // The minimum test case (51, 50) would be hard to
        // find directly without a proper shrinker.
        .assert_minimum_failure((51, 50));
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

fn not_too_wide_struct(r: Rectangle) -> bool {
    r.height < 50 || r.height >= r.width
}

#[test]
fn map_and_automatic_shrinking_of_rectangle() {
    monkey_test()
        .with_generator(rectangle_structs())
        .test_true(not_too_wide_struct)
        // The minimum test case Rectangle{height:51, width:50} would be hard
        // to find directly without a proper shrinker.
        .assert_minimum_failure(Rectangle {
            width: 51,
            height: 50,
        });
}

#[derive(Clone, Debug, PartialEq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

/// Composing together 4 generaetors using `zip_4` to a generator of Color
/// structs.
fn any_color() -> BoxGen<Color> {
    gen::u8::any()
        .zip_4(gen::u8::any(), gen::u8::any(), gen::u8::any())
        .map(
            |(red, green, blue, alpha)| Color {
                red,
                green,
                blue,
                alpha,
            },
            |color| (color.red, color.green, color.blue, color.alpha),
        )
}

fn blue_should_not_dominate_green(c: Color) -> bool {
    c.green >= c.blue
}

#[test]
fn map_and_automatic_shrinking_of_color() {
    monkey_test()
        .with_generator(any_color())
        .test_true(blue_should_not_dominate_green)
        // The minimum test case Color{red:0, green:0, blue:1, alpha:0} would
        // be hard to find directly without a proper shrinker.
        .assert_minimum_failure(Color {
            red: 0,
            green: 0,
            blue: 1,
            alpha: 0,
        });
}
