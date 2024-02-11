use crate::BoxIter;
use crate::BoxShrink;
use crate::Shrink;

/// Boolean shrinker seeing `false` as a smaller value than `true`,
/// hence shrinking towards `false`.
pub fn bool() -> BoxShrink<bool> {
    Box::new(BoolShrink {
        shrink_to_true: false,
    })
}

/// Boolean shrinker seeing `true` as a smaller value than `false`,
/// hence shrinking towards `true`.
pub fn bool_to_true() -> BoxShrink<bool> {
    Box::new(BoolShrink {
        shrink_to_true: true,
    })
}

#[derive(Clone)]
struct BoolShrink {
    shrink_to_true: bool,
}

impl Shrink<bool> for BoolShrink {
    fn candidates(&self, original: bool) -> BoxIter<bool> {
        if original ^ self.shrink_to_true {
            Box::new(std::iter::once(!original))
        } else {
            Box::new(std::iter::empty::<bool>())
        }
    }
}
