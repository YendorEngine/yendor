use crate::prelude::*;
pub trait FovAlgorithm {
    fn compute_fov<T, const GRID_WIDTH: u32, const GRID_HEIGHT: u32>(
        origin: Position<GRID_WIDTH, GRID_HEIGHT>,
        range: u32,
        provider: &mut impl FovProvider<T, GRID_WIDTH, GRID_HEIGHT>,
        pass_through_data: T,
    ) -> HashSet<Position<GRID_WIDTH, GRID_HEIGHT>>;
}
