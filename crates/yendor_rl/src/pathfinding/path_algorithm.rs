use crate::prelude::*;

/// Trait to implement 2D path algorithms.
pub trait PathAlgorithm {
    /// Provides a 2D path between points, using the specified algorithm.
    fn compute_path<T>(
        origin: IVec2,
        destination: IVec2,
        provider: &mut impl PathProvider<T>,
        pass_through_data: T,
    ) -> Vec<IVec2>;
}
