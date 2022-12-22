use crate::prelude::*;
pub trait FovAlgorithm {
    fn compute_fov<T>(
        origin: IVec2,
        range: u32,
        provider: &mut impl FovProvider<T>,
        pass_through_data: T,
    ) -> HashSet<IVec2>;
}
