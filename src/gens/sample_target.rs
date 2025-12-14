/// A ratio of an outcome in relation to the aggregated sum of all ratios.
pub type Ratio = u8;

/// Maps from a sample domain to a target type range.
///
/// Let say we want to toss a coin with wanted distribution 75% heads
/// and 25% tails. Then we can specify the sample domain as \[1, 4\], where
/// first three samples (1, 2 and 3) returns a head and one sample (4)
/// returns a tail.
/// ```text
/// SampleTarget::new_from_ratios(&[(3, "head"),((1, "tail"))]);
/// ```
#[derive(Clone)]
pub struct SampleTarget<E> {
    /// Sample max per example, in ascending order, so pervious max defines
    /// lower bound for this example.
    sample_max_per_target: Vec<(usize, E)>,
}

impl<T> SampleTarget<T> {
    /// Create new sample target with even distribution between targets.
    pub fn evenly(targets: &[T]) -> SampleTarget<T>
    where
        T: Clone + core::fmt::Debug,
    {
        SampleTarget::with_ratios(
            &targets
                .iter()
                .map(|target| (1, target.clone()))
                .collect::<Vec<_>>(),
        )
    }

    /// Create new sample target.
    pub fn with_ratios(ratios_and_targets: &[(Ratio, T)]) -> SampleTarget<T>
    where
        T: Clone + core::fmt::Debug,
    {
        let nonzero_ratios_and_targets = ratios_and_targets
            .iter()
            .filter(|(ratio, _)| *ratio > 0u8)
            .collect::<Vec<_>>();

        if nonzero_ratios_and_targets.is_empty() {
            panic!(
                "Given argument {ratios_and_targets:?} has no target value \
             with non-zero ratio."
            );
        }

        let mut sample_max_per_target = Vec::<(usize, T)>::new();
        let mut max = 0usize;
        for (ratio, target) in nonzero_ratios_and_targets {
            max += *ratio as usize;
            sample_max_per_target.push((max, target.clone()));
        }

        SampleTarget {
            sample_max_per_target,
        }
    }

    /// Map values to other type, keeping the same ratios.
    pub fn map<Q>(self, f: impl Fn(T) -> Q) -> SampleTarget<Q> {
        SampleTarget {
            sample_max_per_target: self
                .sample_max_per_target
                .into_iter()
                .map(|pair| (pair.0, f(pair.1)))
                .collect::<Vec<_>>(),
        }
    }

    /// Sample domain min is always 1.
    pub fn sample_domain_max(&self) -> usize {
        self.sample_max_per_target
            .last()
            .expect("at least one target")
            .0
    }

    /// Translate from sample domain to target type value.
    pub fn target_from_sample(&self, sample: usize) -> Option<&T> {
        self.sample_max_per_target
            .iter()
            .filter(|(max, _)| sample <= *max)
            .map(|(_, target)| target)
            .next()
    }

    /// Translate from sample domain to target type value.
    pub fn target_from_sample_mut(&mut self, sample: usize) -> Option<&mut T> {
        self.sample_max_per_target
            .iter_mut()
            .filter(|(max, _)| sample <= *max)
            .map(|(_, target)| target)
            .next()
    }
}
