//! Convenience traits for generator and shrinker combinators for zipping.

use crate::{gens, shrinks, BoxGen, BoxShrink, Gen, MapWithGen, Shrink};

/// Not dyn compatible (a.k.a. object safe) trait for providing generator
/// zipping.
///
/// The [ZipWithGen::zip] extension method cannot be implemented directly on
/// [Gen] object trait, since generic method in respect to other type `E1`, does
/// not seem to be allowed on trait objects.
pub trait ZipWithGen<E0>
where
    E0: Clone + 'static,
{
    /// See [gens::zip].
    fn zip<E1>(&self, other_gen: BoxGen<E1>) -> BoxGen<(E0, E1)>
    where
        E1: Clone + 'static;

    /// Zip together 3 generators.
    fn zip_3<E1, E2>(
        &self,
        gen1: BoxGen<E1>,
        gen2: BoxGen<E2>,
    ) -> BoxGen<(E0, E1, E2)>
    where
        E1: Clone + 'static,
        E2: Clone + 'static;

    /// Zip together 4 generators.
    fn zip_4<E1, E2, E3>(
        &self,
        gen1: BoxGen<E1>,
        gen2: BoxGen<E2>,
        gen3: BoxGen<E3>,
    ) -> BoxGen<(E0, E1, E2, E3)>
    where
        E1: Clone + 'static,
        E2: Clone + 'static,
        E3: Clone + 'static;

    /// Zip together 5 generators.
    fn zip_5<E1, E2, E3, E4>(
        &self,
        gen1: BoxGen<E1>,
        gen2: BoxGen<E2>,
        gen3: BoxGen<E3>,
        gen4: BoxGen<E4>,
    ) -> BoxGen<(E0, E1, E2, E3, E4)>
    where
        E1: Clone + 'static,
        E2: Clone + 'static,
        E3: Clone + 'static,
        E4: Clone + 'static;

    /// Zip together 6 generators.
    fn zip_6<E1, E2, E3, E4, E5>(
        &self,
        gen1: BoxGen<E1>,
        gen2: BoxGen<E2>,
        gen3: BoxGen<E3>,
        gen4: BoxGen<E4>,
        gen5: BoxGen<E5>,
    ) -> BoxGen<(E0, E1, E2, E3, E4, E5)>
    where
        E1: Clone + 'static,
        E2: Clone + 'static,
        E3: Clone + 'static,
        E4: Clone + 'static,
        E5: Clone + 'static;
}

impl<E0: Clone + 'static> ZipWithGen<E0> for dyn Gen<E0> {
    fn zip<E1>(&self, other_gen: BoxGen<E1>) -> BoxGen<(E0, E1)>
    where
        E1: Clone + 'static,
    {
        gens::zip(self.clone_box(), other_gen)
    }

    fn zip_3<E1, E2>(
        &self,
        gen1: BoxGen<E1>,
        gen2: BoxGen<E2>,
    ) -> BoxGen<(E0, E1, E2)>
    where
        E1: Clone + 'static,
        E2: Clone + 'static,
    {
        gens::zip(gens::zip(self.clone_box(), gen1), gen2)
            .map(|((e0, e1), e2)| (e0, e1, e2), |(e0, e1, e2)| ((e0, e1), e2))
    }

    fn zip_4<E1, E2, E3>(
        &self,
        gen1: BoxGen<E1>,
        gen2: BoxGen<E2>,
        gen3: BoxGen<E3>,
    ) -> BoxGen<(E0, E1, E2, E3)>
    where
        E1: Clone + 'static,
        E2: Clone + 'static,
        E3: Clone + 'static,
    {
        gens::zip(gens::zip(self.clone_box(), gen1), gens::zip(gen2, gen3)).map(
            |((e0, e1), (e2, e3))| (e0, e1, e2, e3),
            |(e0, e1, e2, e3)| ((e0, e1), (e2, e3)),
        )
    }

    fn zip_5<E1, E2, E3, E4>(
        &self,
        gen1: BoxGen<E1>,
        gen2: BoxGen<E2>,
        gen3: BoxGen<E3>,
        gen4: BoxGen<E4>,
    ) -> BoxGen<(E0, E1, E2, E3, E4)>
    where
        E1: Clone + 'static,
        E2: Clone + 'static,
        E3: Clone + 'static,
        E4: Clone + 'static,
    {
        gens::zip(gens::zip(self.clone_box(), gen1), gen2.zip_3(gen3, gen4))
            .map(
                |((e0, e1), (e2, e3, e4))| (e0, e1, e2, e3, e4),
                |(e0, e1, e2, e3, e4)| ((e0, e1), (e2, e3, e4)),
            )
    }

    fn zip_6<E1, E2, E3, E4, E5>(
        &self,
        gen1: BoxGen<E1>,
        gen2: BoxGen<E2>,
        gen3: BoxGen<E3>,
        gen4: BoxGen<E4>,
        gen5: BoxGen<E5>,
    ) -> BoxGen<(E0, E1, E2, E3, E4, E5)>
    where
        E1: Clone + 'static,
        E2: Clone + 'static,
        E3: Clone + 'static,
        E4: Clone + 'static,
        E5: Clone + 'static,
    {
        gens::zip(self.zip_3(gen1, gen2), gen3.zip_3(gen4, gen5)).map(
            |((e0, e1, e2), (e3, e4, e5))| (e0, e1, e2, e3, e4, e5),
            |(e0, e1, e2, e3, e4, e5)| ((e0, e1, e2), (e3, e4, e5)),
        )
    }
}

/// Not dyn compatible (a.k.a. object safe) trait for providing shrinker
/// zipping.
pub trait ZipWithShrink<E0>
where
    E0: Clone + 'static,
{
    /// See [shrinks::zip].
    fn zip<E1>(&self, other_shrink: BoxShrink<E1>) -> BoxShrink<(E0, E1)>
    where
        E1: Clone + 'static;
}

impl<E0: Clone + 'static> ZipWithShrink<E0> for dyn Shrink<E0> {
    fn zip<E1>(&self, other_gen: BoxShrink<E1>) -> BoxShrink<(E0, E1)>
    where
        E1: Clone + 'static,
    {
        shrinks::zip(self.clone_box(), other_gen)
    }
}
