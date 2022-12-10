use crate::{imports::*, prelude::*};

pub trait FovProvider<T: Component, const GRID_WIDTH: u32, const GRID_HEIGHT: u32> {
    fn is_opaque(
        &mut self,
        position: Position<GRID_WIDTH, GRID_HEIGHT>,
        vision_type: u8,
        q_blocks_vision: &Query<&T>,
    ) -> bool;
}
