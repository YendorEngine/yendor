use crate::prelude::*;

mod adams;
pub use adams::*;

mod fov_algorithm;
pub use fov_algorithm::*;
mod fov_provider;
pub use fov_provider::*;
mod slope;
pub use slope::*;
mod shadowcast;
pub use shadowcast::*;

// TODO: Add more Fov Algorithms: http://www.adammil.net/blog/v125_Roguelike_Vision_Algorithms.html
// TODO: Adam
// TODO: Ray casting
// TODO: Diamond walls (point-to-tile or point-to-point)
// TODO: Half-width walls (point-to-tile or point-to-point)
// TODO: Permissive field of view (tile-to-tile)
// TODO: Digital field of view (diamond-to-diamond)
pub enum Fov {
    Adams,
    Shadowcast,
    ShadowcastDirection(Direction),
}

impl Fov {
    pub fn compute<FovRange: Into<u32>, T>(
        &self,
        origin: IVec2,
        range: FovRange,
        provider: &mut impl FovProvider<T>,
        pass_through_data: T,
    ) -> HashSet<IVec2> {
        let range = range.into();
        match self {
            Self::Adams => AdamsFov::compute_fov(origin, range, provider, pass_through_data),
            Self::Shadowcast => Shadowcast::compute_fov(origin, range, provider, pass_through_data),
            Self::ShadowcastDirection(direction) => Shadowcast::compute_direction(
                origin,
                range,
                provider,
                *direction,
                pass_through_data,
            ),
        }
    }

    pub fn within_fov<FovRange: Into<u32>, T>(
        &self,
        origin: IVec2,
        target: IVec2,
        range: FovRange,
        provider: &mut impl FovProvider<T>,
        pass_through_data: T,
    ) -> bool {
        let range = range.into();
        Self::compute(self, origin, range, provider, pass_through_data).contains(&target)
    }
}
