//! Generators for String type.

use crate::BoxGen;
use crate::MapWithGen;

/// Build String generator from char generator.
fn strings_from_chars(chars: BoxGen<char>) -> BoxGen<String> {
    crate::gens::vec::any(chars)
        .map(|v| v.into_iter().collect(), |s| s.chars().collect())
}

/// Strings of any possible Rust `char` value, that is any valid unicode scalar
/// value.
///
/// For implementation details, see [crate::gens::char::unicode].
pub fn unicode() -> BoxGen<String> {
    strings_from_chars(crate::gens::char::unicode())
}

/// Shorthand for [unicode].
pub fn any() -> BoxGen<String> {
    unicode()
}

/// Strings of any arabic numeral 0..9.
///
/// For implementation details, see [crate::gens::char::number].
pub fn number() -> BoxGen<String> {
    strings_from_chars(crate::gens::char::number())
}

/// Strings of any alpha upper char.
///
/// For implementation details, see [crate::gens::char::alpha_upper].
pub fn alpha_upper() -> BoxGen<String> {
    strings_from_chars(crate::gens::char::alpha_upper())
}

/// Strings of any alpha lower char.
///
/// For implementation details, see [crate::gens::char::alpha_lower].
pub fn alpha_lower() -> BoxGen<String> {
    strings_from_chars(crate::gens::char::alpha_lower())
}

/// Strings of any alpha char.
///
/// For implementation details, see [crate::gens::char::alpha].
pub fn alpha() -> BoxGen<String> {
    strings_from_chars(crate::gens::char::alpha())
}

/// Strings of any alpha numeric char.
///
/// For implementation details, see [crate::gens::char::alpha_numeric].
pub fn alpha_numeric() -> BoxGen<String> {
    strings_from_chars(crate::gens::char::alpha_numeric())
}

/// Strings of any ASCII printable character.
///
/// For implementation details, see [crate::gens::char::ascii_printable].
pub fn ascii_printable() -> BoxGen<String> {
    strings_from_chars(crate::gens::char::ascii_printable())
}

/// Strings of any ASCII character, including both printable and non-printable
/// characters.
///
/// For implementation details, see [crate::gens::char::ascii].
pub fn ascii() -> BoxGen<String> {
    strings_from_chars(crate::gens::char::ascii())
}
