//! Module for float value utilities.

/// Enables generic bit manipulation for both `f32` and `f64`.
///
/// For referense, see
/// <https://en.wikipedia.org/wiki/Double-precision_floating-point_format> and
/// <https://en.wikipedia.org/wiki/Single-precision_floating-point_format>
pub trait FloatParts: std::fmt::Debug {
    /// Max exponent value still being a finite (normal) number.
    fn exponent_normal_max() -> u16;

    /// Max fraction value still being a finite (normal) number.
    fn fraction_normal_max() -> u64;

    /// Number of bits to shift exponent.
    fn exponent_bit_position() -> u8;

    /// Number of bits to shift sign bit.
    fn sign_bit_position() -> u8;

    /// Transform float value to bits.
    fn to_bits(&self) -> u64;

    /// Transform bits to float value.
    fn from_bits(bitz: u64) -> Self;

    /// Get exponent from float value.
    fn exponent(&self) -> u16 {
        let mut bits = FloatParts::to_bits(self);

        //  eliminate sign bit
        bits &= (0x1 << Self::sign_bit_position()) - 1;

        (bits >> Self::exponent_bit_position()) as u16
    }

    /// Get fraction from float value.
    fn fraction(&self) -> u64 {
        let mut bits = FloatParts::to_bits(self);

        // eliminate sign and exponent bits
        bits &= (0x1 << Self::exponent_bit_position()) - 1;

        bits
    }

    /// Determin if sign bit is negative.
    fn is_sign_negative(&self) -> bool {
        let bits = self.to_bits();
        (bits >> Self::sign_bit_position()) != 0
    }

    /// Compose sign bit, exponent and fraction into a float value.
    fn compose(sign_is_negative: bool, exponent: u16, fraction: u64) -> u64 {
        let sign: u64 = if sign_is_negative { 0x1 } else { 0x0 };

        (sign << Self::sign_bit_position())
            | (exponent as u64) << Self::exponent_bit_position()
            | fraction
    }
}

impl FloatParts for f32 {
    fn exponent_normal_max() -> u16 {
        0xfe
    }

    fn fraction_normal_max() -> u64 {
        0x7fffff
    }

    fn exponent_bit_position() -> u8 {
        23
    }

    fn sign_bit_position() -> u8 {
        31
    }

    fn to_bits(&self) -> u64 {
        f32::to_bits(*self) as u64
    }

    fn from_bits(bitz: u64) -> Self {
        f32::from_bits(bitz as u32)
    }
}

impl FloatParts for f64 {
    fn exponent_normal_max() -> u16 {
        0x7fe
    }

    fn fraction_normal_max() -> u64 {
        0xfffffffffffff
    }

    fn exponent_bit_position() -> u8 {
        52
    }

    fn sign_bit_position() -> u8 {
        63
    }

    fn to_bits(&self) -> u64 {
        f64::to_bits(*self)
    }

    fn from_bits(bitz: u64) -> Self {
        f64::from_bits(bitz)
    }
}

#[cfg(test)]
mod test {
    use super::FloatParts;
    use crate::*;

    fn f32_exp() -> BoxGen<u16> {
        gen::u16::ranged(0..=f32::exponent_normal_max())
    }

    fn f64_exp() -> BoxGen<u16> {
        gen::u16::ranged(0..=f64::exponent_normal_max())
    }

    fn f32_frac() -> BoxGen<u64> {
        gen::u64::ranged(0..=f32::fraction_normal_max())
    }

    fn f64_frac() -> BoxGen<u64> {
        gen::u64::ranged(0..=f64::fraction_normal_max())
    }

    fn compose_f32(
        sign_is_negative: bool,
        exponent: u16,
        fraction: u64,
    ) -> f32 {
        f32::from_bits(f32::compose(sign_is_negative, exponent, fraction) as u32)
    }

    fn compose_f64(
        sign_is_negative: bool,
        exponent: u16,
        fraction: u64,
    ) -> f64 {
        f64::from_bits(f64::compose(sign_is_negative, exponent, fraction))
    }

    #[test]
    fn roundtrip_test_f32() {
        crate::monkey_test()
            .with_generator(
                f32_exp().zip(f32_frac()).zip(crate::gen::bool::evenly()),
            )
            .assert_true(|((e, f), s)| {
                compose_f32(s, e, f).is_sign_negative() == s
            })
            .assert_true(|((e, f), s)| compose_f32(s, e, f).exponent() == e)
            .assert_true(|((e, f), s)| compose_f32(s, e, f).fraction() == f);
    }

    #[test]
    fn roundtrip_test_f64() {
        crate::monkey_test()
            .with_generator(
                f64_exp().zip(f64_frac()).zip(crate::gen::bool::evenly()),
            )
            .assert_true(|((e, f), s)| {
                compose_f64(s, e, f).is_sign_negative() == s
            })
            .assert_true(|((e, f), s)| compose_f64(s, e, f).exponent() == e)
            .assert_true(|((e, f), s)| compose_f64(s, e, f).fraction() == f);
    }
}
