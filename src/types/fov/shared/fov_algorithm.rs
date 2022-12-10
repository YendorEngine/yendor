use crate::prelude::*;
pub trait FovAlgorithm<'w, 's, T: Component, const GRID_WIDTH: u32, const GRID_HEIGHT: u32> {
    fn compute_fov(
        origin: Position<GRID_WIDTH, GRID_HEIGHT>,
        vision_type: u8,
        range: u32,
        provider: &mut impl FovProvider<T, GRID_WIDTH, GRID_HEIGHT>,
        q_blocks_vision: &Query<'w, 's, &'static T>,
        receiver: &mut impl FovReceiver<GRID_WIDTH, GRID_HEIGHT>,
    );
}
