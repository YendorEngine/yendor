use crate::prelude::*;
pub trait PathProvider<T, const GRID_WIDTH: u32, const GRID_HEIGHT: u32> {
    fn is_walkable(&self, position: Position<GRID_WIDTH, GRID_HEIGHT>, pass_through_data: &mut T) -> bool;

    fn get_neighbors(
        &self,
        position: Position<GRID_WIDTH, GRID_HEIGHT>,
        pass_through_data: &T,
    ) -> Vec<(Position<GRID_WIDTH, GRID_HEIGHT>, u32)>;

    fn distance(
        &self,
        origin: Position<GRID_WIDTH, GRID_HEIGHT>,
        destination: Position<GRID_WIDTH, GRID_HEIGHT>,
    ) -> u32 {
        origin.distance(destination)
    }
}
