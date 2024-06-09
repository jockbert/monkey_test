//! Generators for String type.

use crate::BoxGen;
use crate::MapWithGen;

/// Build String generator from char generator.
fn strings_from_chars(chars: BoxGen<char>) -> BoxGen<String> {
    crate::gens::vec::any(chars)
        .map(|v| v.into_iter().collect(), |s| s.chars().collect())
}

/// Strings of any possible Rust `char` value, that is any valid unicode scalar
/// value. For reference, see
/// [Rust official documentation on `char` type](https://doc.rust-lang.org/std/primitive.char.html).
pub fn unicode() -> BoxGen<String> {
    strings_from_chars(crate::gens::char::unicode())
}

/// Shorthand for [unicode].
pub fn any() -> BoxGen<String> {
    unicode()
}

/// Strings of any arabic numeral 0..9.
pub fn number() -> BoxGen<String> {
    strings_from_chars(crate::gens::char::number())
}

/// Strings of any alpha upper char.
pub fn alpha_upper() -> BoxGen<String> {
    strings_from_chars(crate::gens::char::alpha_upper())
}

/// Strings of any alpha lower char.
pub fn alpha_lower() -> BoxGen<String> {
    strings_from_chars(crate::gens::char::alpha_lower())
}

/// Strings of any alpha char.
pub fn alpha() -> BoxGen<String> {
    strings_from_chars(crate::gens::char::alpha())
}

/// Strings of any alpha numeric char.
pub fn alpha_numeric() -> BoxGen<String> {
    strings_from_chars(crate::gens::char::alpha_numeric())
}

/// Strings of any ASCII printable character
pub fn ascii_printable() -> BoxGen<String> {
    strings_from_chars(crate::gens::char::ascii_printable())
}

/// Strings of any ASCII character, including both printable and non-printable
/// characters.
pub fn ascii() -> BoxGen<String> {
    strings_from_chars(crate::gens::char::ascii())
}
