use crate::shrinks;
use crate::BoxShrink;
use num_traits::Float;

/// Shrink float types towards the value zero.
#[deprecated = "use float_in_range for better shrink behaviour, \
or float_to_zero for the old behaviour."]
pub fn float<F>() -> BoxShrink<F>
where
    F: Float + std::fmt::Debug + Clone + 'static,
{
    float_to_zero()
}

/// Float value (both types `f32` and `f64`) shrinker shrinking towards `+0.0`.
///
/// In particular, the float values are regarded to have the following size
/// order, from largest to smallest in decending order:
///
/// `NaN`, `-Inf`, `Inf`, `Min`, `Max`, ...., `-100`, `100`, ..., `-10`, `10`,
///  ..., `-1`, `1`, ..., `-0`, `0`.
pub fn float_to_zero<F>() -> BoxShrink<F>
where
    F: Float + std::fmt::Debug + 'static,
{
    float_in_range(F::min_value(), F::max_value())
}

/// Shrink float types towards the value zero if in range, or other value
/// nearest zero within range of given min and max.
pub fn float_in_range<F>(min: F, max: F) -> BoxShrink<F>
where
    F: Float + std::fmt::Debug + 'static,
{
    shrinks::from_fn(move |original: F| {
        let finite_original = if original.is_finite() {
            original
        } else {
            F::min_value()
        };

        let finites = finite_values(finite_original, min, max);
        let specials = special_values(original);

        specials.clone().into_iter().chain(finites.clone())
    })
}

fn special_values<F>(original: F) -> Vec<F>
where
    F: Float + std::fmt::Debug + Clone + 'static,
{
    let mut values = Vec::<F>::new();
    if original.is_nan() {
        values.push(F::neg_infinity());
        values.push(F::infinity());
    } else if original.is_infinite() && original.is_sign_negative() {
        values.push(F::infinity());
    }
    values
}

fn finite_values<F>(
    original: F,
    min: F,
    max: F,
) -> impl Iterator<Item = F> + Clone
where
    F: Float + std::fmt::Debug + Clone + 'static,
{
    check_example_is_in_range(original, min, max);

    let mut subtraction = original;
    let mut old_result = original;
    let two = F::one() + F::one();

    std::iter::from_fn(move || {
        let result = original - subtraction;
        if result == old_result {
            None
        } else {
            subtraction = subtraction / two;
            old_result = result;

            Some(vec![-result, result])
        }
    })
    .flatten()
    // Filter out examples out of range
    .filter(move |e| min <= *e && *e <= max)
}

/// Panics if example is out of range
fn check_example_is_in_range<F: Float + std::fmt::Debug>(
    example: F,
    min: F,
    max: F,
) {
    if !(example >= min && example <= max) {
        panic!("Given example {example:?} is not in range {min:?}..={max:?}.")
    }
}

/// Chooses shrink target based on given valid range.
///
/// By default zero is the shrink target, but this can be shifted by the given
/// valid range.
fn shrink_target<E: Float>(range_min: E, range_max: E) -> E {
    if range_min > E::zero() {
        range_min
    } else if range_max < E::zero() {
        range_max
    } else {
        E::zero()
    }
}

#[cfg(test)]
mod test {
    use crate::gens;
    use crate::testing::assert_iter_eq;
    use crate::*;

    #[test]
    fn target_adjusts_with_range() {
        assert_eq!(super::shrink_target(-5.0, 9.0), 0.0);
        assert_eq!(super::shrink_target(-1.0, 9.0), 0.0);
        assert_eq!(super::shrink_target(-11.0, -9.0), -9.0);
        assert_eq!(super::shrink_target(-11.0, -2.0), -2.0);
        assert_eq!(super::shrink_target(1.0, 9.0), 1.0);
        assert_eq!(super::shrink_target(3.0, 9.0), 3.0);
    }

    /// Providing failing examples out of range is logically wrong and
    /// will panic. Assumptions and graceful degradation can be made but
    /// not sure it is the logically right thing to do. Better to be
    /// restrictive now and possibly open up API at a later time.
    #[test]
    #[should_panic = "Given example 1337.0 is not in range 0.0..=16.0."]
    fn out_of_range_example_should_panic() {
        let _ = super::finite_values(1337.0, 0.0, 16.0);
    }

    #[test]
    #[should_panic = "Given example NaN is not in range 0.0..=16.0."]
    fn nan_example_should_panic() {
        let _ = super::finite_values(f64::NAN, 0.0, 16.0);
    }

    #[test]
    fn should_be_able_to_shrink_from_nan() {
        assert_iter_eq(
            super::float_to_zero().candidates(f32::NAN).take(4),
            vec![f32::NEG_INFINITY, f32::INFINITY, -0.0, 0.0],
            "should shrink to (-)inf and further from NaN",
        )
    }

    #[test]
    fn shrinker_should_always_terminate() {
        monkey_test()
            .with_generator(gens::f64::any())
            .assert_true(|f| {
                dbg!(super::float_to_zero().candidates(f).count()) < 200
            });
    }

    #[test]
    fn should_only_return_values_within_range() {
        let mmms = crate::gen::f64::finite()
            .zip_3(crate::gen::f64::finite(), crate::gen::f64::finite());

        assert_within_range(mmms);
    }

    fn assert_within_range<E>(gen: BoxGen<(E, E, E)>)
    where
        E: num_traits::Float
            + std::fmt::Debug
            + std::panic::UnwindSafe
            + 'static,
    {
        crate::monkey_test()
            .with_example_count(1000)
            .with_generator(gen)
            .assert_no_panic(|(a, b, c)| {
                let min = a.min(b).min(c);
                let max = a.max(b).max(c);
                let middle = if a != min && a != max {
                    a
                } else if b != min && b != max {
                    b
                } else {
                    c
                };

                super::float_in_range(min, max).candidates(middle).for_each(
                    |candidate| {
                        assert!(
                            candidate >= min,
                            "candidate {candidate:?} >= min {min:?}"
                        );
                        assert!(
                            candidate <= max,
                            "candidate {candidate:?} <= max {max:?}"
                        );
                    },
                );
            });
    }

    #[test]
    fn shrink_to_positive_target() {
        monkey_test()
            .with_generator(gen::f64::positive().zip(gen::f64::positive()))
            .assert_eq(
                |(a, b)| a.min(b),
                |(a, b)| {
                    let range_min = a.min(b);
                    let failing_original = a.max(b);

                    if a == b {
                        // threre are no candidates if shrinking from x to x.
                        a
                    } else {
                        dbg!(range_min, failing_original);
                        super::float_in_range(range_min, f64::MAX)
                            .candidates(failing_original)
                            .last()
                            .unwrap()
                    }
                },
            );
    }
}
