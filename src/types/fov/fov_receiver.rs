use crate::prelude::*;

pub trait FovReceiver<const GRID_WIDTH: u32, const GRID_HEIGHT: u32> {
    fn set_visible(&mut self, position: Position<GRID_WIDTH, GRID_HEIGHT>);
    fn get_visible(&self, position: Position<GRID_WIDTH, GRID_HEIGHT>) -> bool;
    fn get_all(&self) -> HashSet<Position<GRID_WIDTH, GRID_HEIGHT>>;
}
