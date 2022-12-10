use crate::prelude::*;
pub trait FovAlgorithm<'w, 's> {
    fn compute_fov<T, const GRID_WIDTH: u32, const GRID_HEIGHT: u32>(
        origin: Position<GRID_WIDTH, GRID_HEIGHT>,
        range: u32,
        provider: &mut impl FovProvider,
        receiver: &mut impl FovReceiver,
        pass_through_data: T,
    );
}
