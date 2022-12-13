use crate::prelude::*;
pub trait PathAlgorithm {
    fn compute_path<T, const DIM: UVec2>(
        origin: Position<DIM>,
        destination: Position<DIM>,
        provider: &mut impl PathProvider<T>,
        pass_through_data: T,
    ) -> Vec<Position<DIM>>;
}
