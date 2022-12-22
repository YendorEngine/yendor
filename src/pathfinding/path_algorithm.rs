use crate::prelude::*;

pub trait PathAlgorithm {
    fn compute_path<T>(
        origin: UVec2,
        destination: UVec2,
        provider: &mut impl PathProvider<T>,
        pass_through_data: T,
    ) -> Vec<UVec2>;
}
