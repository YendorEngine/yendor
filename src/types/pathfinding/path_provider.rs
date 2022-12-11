use crate::prelude::*;
pub trait PathProvider<T, const GRID_WIDTH: u32, const GRID_HEIGHT: u32> {
    fn is_walkable(
        &mut self,
        position: Position<GRID_WIDTH, GRID_HEIGHT>,
        pass_through_data: T
    ) -> bool;

    fn get_neighbors(
        &mut self,
        position: Position<GRID_WIDTH, GRID_HEIGHT>,
        pass_through_data: &mut T,
    ) -> Vec<Position<GRID_WIDTH, GRID_HEIGHT>>;

    fn cost(&mut self, position: Position<GRID_WIDTH, GRID_HEIGHT>, pass_through_data: T) -> u32;
}
