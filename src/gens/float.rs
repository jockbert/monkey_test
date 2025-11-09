//! Module with generators for generic floating point values.
//! For non-generic types `f64` and `f32`, see modules [crate::gens::f64]
//! and [crate::gens::f32] instead, which are specializations of this module.

use super::float_parts::FloatParts;
use crate::gens;
use crate::BoxGen;
use crate::MapWithGen;
use num_traits::Float;
use rand::distributions::uniform::SampleUniform;
use rand::Rng;
use rand::SeedableRng;
use std::fmt::Debug;
use std::ops::Bound;
use std::ops::RangeBounds;

/// Generator that return any floating point value, including any
/// finite number, NaN, Inf and -Inf.
pub fn any<F>() -> BoxGen<F>
where
    F: Float + FloatParts + SampleUniform + 'static,
{
    let nans = gens::fixed::constant(F::nan());
    gens::mix_with_ratio(&[(98, number()), (2, nans)])
}

/// Generator that only return finite numbers, `-Inf` and
/// `Inf`. In other words any float value besides `NaN`.
pub fn number<F>() -> BoxGen<F>
where
    F: Float + FloatParts + SampleUniform + 'static,
{
    let infs = gens::pick_evenly(&[F::neg_infinity(), F::infinity()]);
    gens::mix_with_ratio(&[(98, finite()), (2, infs)])
}

/// Generator that only return numbers between 0 and `+Inf`.
pub fn positive<F>() -> BoxGen<F>
where
    F: Float + FloatParts + SampleUniform + 'static,
{
    let infs = gens::fixed::constant(F::infinity());
    let finites = ranged(F::zero()..=F::max_value());
    gens::mix_with_ratio(&[(98, finites), (2, infs)])
}

/// Generator that only return numbers between `-Inf` and -0.
pub fn negative<F>() -> BoxGen<F>
where
    F: Float + FloatParts + SampleUniform + 'static,
{
    let infs = gens::fixed::constant(F::neg_infinity());
    let finites = ranged(F::min_value()..=F::neg_zero());
    gens::mix_with_ratio(&[(98, finites), (2, infs)])
}

/// Generator that only return finite numbers between minimum
/// and maximum finite number.
pub fn finite<F>() -> BoxGen<F>
where
    F: Float + FloatParts + SampleUniform + 'static,
{
    ranged(F::min_value()..=F::max_value())
}

/// Generator that only return finite numbers in the range from 0
/// (inclusive) to 1 (exclusive).
pub fn zero_to_one<F>() -> BoxGen<F>
where
    F: Float + FloatParts + SampleUniform + 'static,
{
    ranged(F::zero()..F::one())
}

/// Generator that only return finite numbers in the given range.
pub fn ranged<F, B>(bound: B) -> BoxGen<F>
where
    F: Float + FloatParts + SampleUniform + 'static,
    B: RangeBounds<F>,
{
    let start: F = from_twos_complement_bits(start(&bound));
    let end: F = from_twos_complement_bits(end(&bound));

    check_bounds_are_finite(start, end);

    let (start, end) = if start > end {
        (end, start)
    } else {
        (start, end)
    };

    let all_special_values = [
        F::zero(),
        F::neg_zero(),
        F::one(),
        F::one().neg(),
        F::min_value(),
        F::max_value(),
        F::min_positive_value(),
        F::min_positive_value().neg(),
        start,
        end,
    ];

    let relevant_special_values = all_special_values
        .iter()
        .filter(|v| {
            // special handling if one f bound extremes happen to be zero. We
            // want to distinguish between -0 and +0.
            let value_has_sign_within_range = v.is_sign_negative()
                && start.is_sign_negative()
                || v.is_sign_positive() && end.is_sign_positive();

            let is_in_range = start <= **v && **v <= end;

            is_in_range && value_has_sign_within_range
        })
        .cloned()
        .collect::<Vec<_>>();

    let special_values = gens::pick_evenly(&relevant_special_values);

    gens::mix_with_ratio(&[
        (90, completely_random_range(bound)),
        (10, special_values),
    ])
}

/// Float generator with completely random distribution. This function has a
/// long name, since `ranged` should be preferred.
pub fn completely_random_range<F, B>(bound: B) -> BoxGen<F>
where
    F: Float + FloatParts + SampleUniform + 'static,
    B: RangeBounds<F>,
{
    let min = start(&bound);
    let max = end(&bound);

    check_bounds_are_finite::<F>(
        from_twos_complement_bits(min),
        from_twos_complement_bits(max),
    );

    let (min, max) = if min > max { (max, min) } else { (min, max) };

    // Generate two's complement bits signed integer representation of finite
    // floats
    gens::from_fn(move |seed| {
        //let distr = rand::distributions::Uniform::new_inclusive(min, max);
        let mut x = rand_chacha::ChaCha8Rng::seed_from_u64(seed);

        std::iter::from_fn(move || Some(x.gen_range(min..=max)))
    })
    // Map to actual floats from two's complement bits
    .map(
        |i| from_twos_complement_bits(i),
        |f| to_twos_complement_bits(f),
    )
    .with_shrinker(crate::shrinks::float())
}

fn check_bounds_are_finite<F>(start: F, end: F)
where
    F: Float + Debug + 'static,
{
    assert!(
        start.is_finite(),
        "Given range can not have non-finite value {start:?} as range start"
    );
    assert!(
        end.is_finite(),
        "Given range can not have non-finite value {end:?} as range end"
    );
}

/// Convert floating point to signed bit representation
fn to_twos_complement_bits<F>(f: F) -> i64
where
    F: Float + FloatParts,
{
    let sign_is_negative = f.is_sign_negative();
    let exponent = f.exponent() as i64;
    let fraction = f.fraction() as i64;

    let unsigned_bits = (exponent << F::exponent_bit_position()) | fraction;

    if sign_is_negative {
        // special case for -0.0, which can not be represented by negation only,
        // hence also subtracting one.
        -unsigned_bits - 1
    } else {
        unsigned_bits
    }
}

fn from_twos_complement_bits<F>(i: i64) -> F
where
    F: Float + FloatParts,
{
    let sign_is_negative = i < 0;
    let unsigned_bits = if sign_is_negative { -i - 1 } else { i };

    let fraction_mask = (1 << F::exponent_bit_position()) - 1;
    let fraction = (unsigned_bits & fraction_mask) as u64;
    let exponent = (unsigned_bits >> F::exponent_bit_position()) as u16;

    F::from_bits(F::compose(sign_is_negative, exponent, fraction))
}

fn start<F, B>(bounds: &B) -> i64
where
    F: Float + FloatParts + Copy,
    B: RangeBounds<F>,
{
    match bounds.start_bound() {
        Bound::Included(x) => to_twos_complement_bits(*x),
        Bound::Excluded(x) => to_twos_complement_bits(*x) + 1,
        Bound::Unbounded => to_twos_complement_bits(F::min_value()),
    }
}

fn end<F, B>(bounds: &B) -> i64
where
    F: Float + FloatParts + Copy,
    B: RangeBounds<F>,
{
    match bounds.end_bound() {
        Bound::Included(x) => to_twos_complement_bits(*x),
        Bound::Excluded(x) => to_twos_complement_bits(*x) - 1,
        Bound::Unbounded => to_twos_complement_bits(F::max_value()),
    }
}

#[cfg(test)]
mod test {
    //! In many of the tests, we only test f32, not f64, since expecting the
    //! behaviour to be type agnostic.

    use super::from_twos_complement_bits;
    use super::to_twos_complement_bits;
    use crate::gens;
    use crate::monkey_test;
    use crate::testing::assert_generator_can_shrink;
    use crate::BoxGen;
    use std::ops::RangeBounds;

    #[test]
    fn convert_to_and_from_finite_f64() {
        // For more info on the composition of double precision floats, see
        // https://en.wikipedia.org/wiki/Double-precision_floating-point_format
        let exponent_max: i64 = 0x7fe; // 11 bits
        let fraction_max: i64 = 0xF_FFFF_FFFF_FFFF; // 52 bits
        let max = (exponent_max << 52) | fraction_max;
        let min = -max;

        monkey_test()
            .with_generator(gens::i64::ranged(min..=max))
            .assert_true(|i| {
                let f: f64 = from_twos_complement_bits(i);
                let j = to_twos_complement_bits(f);

                i == j
            });
    }

    #[test]
    fn convert_to_and_from_finite_f32() {
        // For more info on the composition of single precision floats, see
        // https://en.wikipedia.org/wiki/Single-precision_floating-point_format
        let exponent_max: i64 = 0xfe; // 8 bits
        let fraction_max: i64 = 0x7f_ffff; // 23 bits
        let max = (exponent_max << 23) | fraction_max;
        let min = -max;

        monkey_test()
            .with_generator(gens::i64::ranged(min..=max))
            .assert_true(|i| {
                let f: f32 = from_twos_complement_bits(i);
                let j = to_twos_complement_bits(f);

                i == j
            });
    }

    #[test]
    fn sign_is_kept_on_zero_in_conversion() {
        let original = -0.0;
        let bits = to_twos_complement_bits(original);

        let copy: f64 = from_twos_complement_bits(bits);
        assert!(bits < 0, "negative sign is kept in bits representation");
        assert!(
            copy.is_sign_negative(),
            "negative sign is kept in roundtrip float"
        );
        assert!(
            copy == 0.0,
            "value 0 is kept in roundtrip float, got {copy}"
        );
    }

    #[test]
    fn verify_generator_any() {
        let generator = gens::f32::any();

        assert_has_values(
            generator,
            &[
                f32::NAN,
                f32::MAX,
                f32::MIN,
                f32::INFINITY,
                f32::NEG_INFINITY,
                0.0,
                -0.0,
                1.0,
                -1.0,
            ],
        );
    }

    #[test]
    fn verify_generator_number() {
        let generator = gens::f32::number();
        assert_all_values_are_in_range(
            generator.clone(),
            f32::NEG_INFINITY..=f32::INFINITY,
        );
        assert_has_values(generator, &[0.0]);
    }

    #[test]
    fn verify_generator_positive() {
        let generator = gens::f32::positive();
        assert_all_values_are_in_range(generator.clone(), 0.0..=f32::INFINITY);
        assert_does_not_have_value(generator.clone(), -0.0);
        assert_has_values(generator, &[0.0, 1.0, f32::MAX, f32::INFINITY]);
    }

    #[test]
    fn verify_generator_negative() {
        let generator = gens::f32::negative();
        assert_all_values_are_in_range(
            generator.clone(),
            f32::NEG_INFINITY..=-0.0,
        );
        assert_does_not_have_value(generator.clone(), 0.0);
        assert_has_values(
            generator,
            &[-0.0, -1.0, f32::MIN, f32::NEG_INFINITY],
        );
    }

    #[test]
    fn verify_generator_finite() {
        let generator = gens::f32::finite();
        assert_all_values_are_in_range(generator.clone(), f32::MIN..=f32::MAX);
        assert_has_values(
            generator,
            &[-0.0, -1.0, 0.0, 1.0, f32::MIN, f32::MAX],
        );
    }

    #[test]
    fn verify_generator_ranged() {
        // negative range
        let generator = gens::f32::ranged(-555.0..=-72.0);
        assert_all_values_are_in_range(generator.clone(), -555.0..=-72.0);
        assert_has_values(generator, &[-555.0, -72.0]);

        // positive range
        let generator = gens::f32::ranged(72.0..=555.0);
        assert_all_values_are_in_range(generator.clone(), 72.0..=555.0);
        assert_has_values(generator, &[72.0, 555.0]);

        // range inverted. lagest value first in range
        let generator = gens::f32::ranged(555.0..=72.0);
        assert_all_values_are_in_range(generator.clone(), 72.0..=555.0);
        assert_has_values(generator, &[72.0, 555.0]);

        // sign-straddling range
        let generator = gens::f32::ranged(-555.0..=72.0);
        assert_all_values_are_in_range(generator.clone(), -555.0..=72.0);
        assert_has_values(generator, &[-555.0, 72.0, -0.0, 0.0, -1.0, 1.0]);
    }

    #[test]
    fn verify_generator_zero_to_one() {
        let generator = gens::f32::zero_to_one();
        assert_all_values_are_in_range(generator.clone(), 0.0..1.0);
        assert_has_values(generator.clone(), &[0.0]);
        assert_does_not_have_value(generator, -0.0);
    }

    #[test]
    #[should_panic]
    fn let_ranged_panic_on_nan() {
        gens::f32::ranged(f32::NAN..10.0);
    }

    #[test]
    #[should_panic]
    fn let_ranged_panic_on_neg_inf() {
        gens::f32::ranged(f32::NEG_INFINITY..10.0);
    }

    #[test]
    #[should_panic]
    fn let_ranged_panic_on_pos_inf() {
        gens::f32::ranged(10.0..=f32::INFINITY);
    }

    #[test]
    #[should_panic]
    fn let_completely_random_panic_on_nan() {
        gens::f32::completely_random(f32::NAN..10.0);
    }

    #[test]
    #[should_panic]
    fn let_completely_random_panic_on_neg_inf() {
        gens::f32::completely_random(f32::NEG_INFINITY..10.0);
    }

    #[test]
    #[should_panic]
    fn let_completely_random_panic_on_pos_inf() {
        gens::f32::completely_random(10.0..=f32::INFINITY);
    }

    #[test]
    fn should_have_shrinker() {
        assert_generator_can_shrink(gens::f64::any(), std::f64::consts::PI)
    }

    fn assert_all_values_are_in_range<B>(generator: BoxGen<f32>, range: B)
    where
        B: RangeBounds<f32> + std::fmt::Debug,
    {
        generator
            .examples(crate::seed_to_use())
            .take(1000)
            .for_each(|v| {
                assert!(
                    range.contains(&v),
                    "Range {range:?} should contain {v}"
                )
            });
    }

    fn assert_has_values(generator: BoxGen<f32>, values: &[f32]) {
        values.iter().for_each(|v| {
            let examples_count = 1000;
            let has_value =
                generator.examples(crate::seed_to_use()).take(examples_count).any(|e| {
                  float_equals(e, *v)
                });

            assert!(
                has_value,
                "Generator did not have {v} in the first {examples_count} examples."
            );
        });
    }

    fn assert_does_not_have_value(generator: BoxGen<f32>, value: f32) {
        let examples_count = 500;
        generator
            .examples(crate::seed_to_use())
            .take(examples_count)
            .for_each(|e| {
                let r = float_equals(e, value);

                println!("cmp: {e} == {value}: {r}");

                assert!(
                    !r,
                    "Search for {value} in generator found {e} in \
                    the first {examples_count} examples."
                );
            });
    }

    fn float_equals(a: f32, b: f32) -> bool {
        // NaN never equals anything else, not even other NaN
        let both_are_nan = a.is_nan() && b.is_nan();
        // In these generators +0.0 and -0.0 are not regarded to be equal, even
        // though for normal floats they are regarded to be equal.
        let signums_are_the_same = a.signum() == b.signum();
        // Exact comparison is used on purpose
        both_are_nan || (signums_are_the_same && a == b)
    }
}
