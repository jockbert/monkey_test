use crate::BoxShrink;
use crate::shrink;
use num_traits::Float;

/// Float value (both types `f32` and `f64`) shrinker shrinking towards `+0.0`.
///
/// In particular, the float values are regarded to have the following size
/// order, from largest to smallest in decending order:
///
/// `NaN`, `-Inf`, `Inf`, `Min`, `Max`, ...., `-100`, `100`, ..., `-10`, `10`,
///  ..., `-1`, `1`, ..., `-0`, `0`.
pub fn float<F>() -> BoxShrink<F>
where
    F: Float + std::fmt::Debug + Clone + 'static,
{
    shrink::from_fn(move |original: F| {
        let finite_original = if original.is_finite() {
            original
        } else {
            F::min_value()
        };

        let finites = finite_values(finite_original);
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

fn finite_values<F>(original: F) -> impl Iterator<Item = F> + Clone
where
    F: Float + std::fmt::Debug + Clone + 'static,
{
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
}

#[cfg(test)]
mod test {
    use crate::gens;
    use crate::monkey_test;
    use crate::testing::assert_iter_eq;

    #[test]
    fn should_be_able_to_shrink_from_nan() {
        assert_iter_eq(
            super::float().candidates(f32::NAN).take(4),
            vec![f32::NEG_INFINITY, f32::INFINITY, -0.0, 0.0],
            "should shrink to (-)inf and further from NaN",
        )
    }

    #[test]
    fn shrinker_should_always_terminate() {
        monkey_test()
            .with_generator(gens::f64::any())
            .assert_true(|f| dbg!(super::float().candidates(f).count()) < 200);
    }
}
