use crate::prelude::*;

pub trait PathAlgorithm {
    fn compute_path<T>(
        origin: IVec2,
        destination: IVec2,
        provider: &mut impl PathProvider<T>,
        pass_through_data: T,
    ) -> Vec<IVec2>;
}
