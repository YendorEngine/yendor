use crate::prelude::*;
pub trait PathProvider {
    fn is_walkable<T, const GRID_WIDTH: u32, const GRID_HEIGHT: u32>(
        &mut self,
        position: Position<GRID_WIDTH, GRID_HEIGHT>,
        pass_through_data: T
    ) -> bool;

    fn cost<T, const GRID_WIDTH: u32, const GRID_HEIGHT: u32>(&mut self, position: Position<GRID_WIDTH, GRID_HEIGHT>, pass_through_data: T) -> u32;
}
