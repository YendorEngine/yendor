use crate::prelude::*;

pub enum Fov<T> {
    Shadowcast(T),
    ShadowcastDirection(CardinalDirection, T),
}

impl<T> Fov<T> {
    pub fn compute<FovRange: Into<u32>, const GRID_WIDTH: u32, const GRID_HEIGHT: u32>(
        &self,
        origin: Position<GRID_WIDTH, GRID_HEIGHT>,
        range: FovRange,
        provider: &mut impl FovProvider,
        receiver: &mut impl FovReceiver,
    ) {
        let range = range.into();
        match self {
            Self::Shadowcast(&pass_through_data) => Shadowcast::compute_fov(
                origin,
                range,
                provider,
                receiver,
                pass_through_data,
            ),
            Self::ShadowcastDirection(direction, &pass_through_data) => Shadowcast::compute_direction(
                origin,
                range,
                provider,
                receiver,
                *direction,
                pass_through_data,
            ),
        }
    }
}
