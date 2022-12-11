use crate::prelude::*;
pub trait PathAlgorithm {
    fn compute_path<T, const GRID_WIDTH: u32, const GRID_HEIGHT: u32>(
        origin: Position<GRID_WIDTH, GRID_HEIGHT>,
        destination: Position<GRID_WIDTH, GRID_HEIGHT>,
        provider: &mut impl PathProvider<T, GRID_WIDTH, GRID_HEIGHT>,
        pass_through_data: &T,
    ) -> Vec<Position<GRID_WIDTH, GRID_HEIGHT>>;
}
