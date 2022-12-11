use crate::prelude::*;
pub trait PathAlgorithm {
    fn compute_path<T, const GRID_WIDTH: u32, const GRID_HEIGHT: u32>(
        origin: Position<GRID_WIDTH, GRID_HEIGHT>,
        destination: Position<GRID_WIDTH, GRID_HEIGHT>,
        partial_path_on_failure: bool,
        provider: &mut impl PathProvider,
        pass_through_data: T,
    ) -> Option<Vec<Position>>;
}
