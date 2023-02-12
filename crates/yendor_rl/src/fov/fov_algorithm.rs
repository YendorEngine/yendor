use crate::prelude::*;

/// A trait for computing the field of view.
pub trait FovAlgorithm {
    /// Computes the field of view.
    fn compute_fov<T>(
        origin: IVec2,
        range: u32,
        provider: &mut impl FovProvider<T>,
        pass_through_data: T,
    ) -> HashSet<IVec2>;
}
