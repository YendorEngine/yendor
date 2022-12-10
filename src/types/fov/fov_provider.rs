use crate::prelude::*;
pub trait FovProvider {
    fn is_opaque<T, const GRID_WIDTH: u32, const GRID_HEIGHT: u32>(
        &mut self,
        position: Position<GRID_WIDTH, GRID_HEIGHT>,
        pass_through_data: T
    ) -> bool;
}
