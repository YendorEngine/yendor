use crate::prelude::*;

pub enum Fov {
    Shadowcast,
    ShadowcastDirection(CardinalDirection),
}

impl<T: Component, const GRID_WIDTH: u32, const GRID_HEIGHT: u32> Fov {
    pub fn compute<FovRange: Into<u32>>(
        &self,
        origin: Position<GRID_WIDTH, GRID_HEIGHT>,
        vision_type: u8,
        range: FovRange,
        provider: &mut impl FovProvider<T, GRID_WIDTH, GRID_HEIGHT>,
        q_blocks_vision: &Query<&'static T>,
        receiver: &mut impl FovReceiver<GRID_WIDTH, GRID_HEIGHT>,
    ) {
        let range = range.into();
        match self {
            Self::Shadowcast => Shadowcast::compute_fov(
                origin,
                vision_type,
                range,
                provider,
                q_blocks_vision,
                receiver,
            ),
            Self::ShadowcastDirection(direction) => Shadowcast::compute_direction(
                origin,
                vision_type,
                range,
                provider,
                q_blocks_vision,
                receiver,
                *direction,
            ),
        }
    }
}
