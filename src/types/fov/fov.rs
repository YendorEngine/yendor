use crate::prelude::*;

pub enum Fov {
    Shadowcast,
    ShadowcastDirection(CardinalDirection),
}

impl Fov {
    pub fn compute<FovRange: Into<u32>>(
        &self,
        origin: Position,
        vision_type: u8,
        range: FovRange,
        provider: &mut impl FovProvider,
        q_blocks_vision: &Query<&'static BlocksVision>,
        receiver: &mut impl FovReceiver,
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
