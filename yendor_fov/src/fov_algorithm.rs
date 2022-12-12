use crate::prelude::*;
pub trait FovAlgorithm {
    fn compute_fov<T, const DIM: UVec2>(
        origin: Position<DIM>,
        range: u32,
        provider: &mut impl FovProvider<T, DIM>,
        pass_through_data: T,
    ) -> HashSet<Position<DIM>>;
}
