//! Generators for char type.

use crate::BoxGen;
use crate::FilterWithGen;
use crate::MapWithGen;

/// Any possible Rust `char` value, that is any valid unicode scalar value.
/// For reference, see
/// [Rust official documentation on `char` type](https://doc.rust-lang.org/std/primitive.char.html).
pub fn unicode() -> BoxGen<char> {
    crate::gen::u32::ranged(0..=0x10ffff)
        .filter(|&num| !(0xD800..=0xDFFF).contains(&num))
        .map(|num| char::from_u32(num).unwrap(), |ch| ch as u32)
}

/// Shorthand for [unicode].
pub fn any() -> BoxGen<char> {
    unicode()
}

/// Build char generator from u32 range (inclusive)
fn chars_from_u32_range(min: u32, max_inclusive: u32) -> BoxGen<char> {
    crate::gen::u32::ranged(min..=max_inclusive)
        .map(|num| char::from_u32(num).unwrap(), |ch| ch as u32)
}

/// Any arabic numeral 0..9.
pub fn number() -> BoxGen<char> {
    chars_from_u32_range(48, 57)
}

/// Any alpha upper char.
pub fn alpha_upper() -> BoxGen<char> {
    chars_from_u32_range(65, 90)
}

/// Any alpha lower char.
pub fn alpha_lower() -> BoxGen<char> {
    chars_from_u32_range(97, 122)
}

/// Any alpha char.
pub fn alpha() -> BoxGen<char> {
    crate::gen::mix_evenly(&[alpha_upper(), alpha_lower()])
}

/// Any alpha numeric char.
pub fn alpha_numeric() -> BoxGen<char> {
    crate::gen::mix_with_ratio(&[(9, alpha()), (1, number())])
}

/// Any ASCII printable character
pub fn ascii_printable() -> BoxGen<char> {
    chars_from_u32_range(32, 126)
}

/// Any ASCII character, including both printable and non-printable characters.
pub fn ascii() -> BoxGen<char> {
    chars_from_u32_range(0, 127)
}
