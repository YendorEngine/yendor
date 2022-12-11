use crate::prelude::*;

pub trait FovProvider<T, const GRID_WIDTH: u32, const GRID_HEIGHT: u32> {
    fn is_opaque(&mut self, position: Position<GRID_WIDTH, GRID_HEIGHT>, pass_through_data: &'_ T) -> bool;
}
