use crate::prelude::*;
pub trait FovReceiver {
    fn set_visible<const GRID_WIDTH: u32, const GRID_HEIGHT: u32>(&mut self, position: Position<GRID_WIDTH, GRID_HEIGHT>);
    fn get_visible<const GRID_WIDTH: u32, const GRID_HEIGHT: u32>(&self, position: Position<GRID_WIDTH, GRID_HEIGHT>) -> bool;
    fn get_all<const GRID_WIDTH: u32, const GRID_HEIGHT: u32>(&self) -> HashSet<Position<GRID_WIDTH, GRID_HEIGHT>>;
}
