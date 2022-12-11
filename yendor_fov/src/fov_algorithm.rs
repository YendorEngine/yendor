use crate::prelude::*;
pub trait FovAlgorithm {
    fn compute_fov<T, const DIMENSIONS: UVec2>(
        origin: Position<DIMENSIONS>,
        range: u32,
        provider: &mut impl FovProvider<T, DIMENSIONS>,
        pass_through_data: T,
    ) -> HashSet<Position<DIMENSIONS>>;
}
