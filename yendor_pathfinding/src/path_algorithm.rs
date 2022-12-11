use crate::prelude::*;
pub trait PathAlgorithm {
    fn compute_path<T, const DIMENSIONS: UVec2>(
        origin: Position<DIMENSIONS>,
        destination: Position<DIMENSIONS>,
        provider: &mut impl PathProvider<T, DIMENSIONS>,
        pass_through_data: T,
    ) -> Vec<Position<DIMENSIONS>>;
}
