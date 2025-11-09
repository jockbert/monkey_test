use crate::BoxShrink;

/// Boolean shrinker seeing `false` as a smaller value than `true`,
/// hence shrinking towards `false`.
pub fn bool() -> BoxShrink<bool> {
    bool_to(false)
}

/// Boolean shrinker seeing `true` as a smaller value than `false`,
/// hence shrinking towards `true`.
pub fn bool_to_true() -> BoxShrink<bool> {
    bool_to(true)
}

fn bool_to(shrink_to_true: bool) -> BoxShrink<bool> {
    crate::shrinks::from_fn_boxed(move |original: bool| {
        if original ^ shrink_to_true {
            Box::new(std::iter::once(!original))
        } else {
            Box::new(std::iter::empty::<bool>())
        }
    })
}
