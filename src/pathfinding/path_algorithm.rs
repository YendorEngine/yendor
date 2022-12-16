use crate::prelude::*;

pub trait PathAlgorithm {
    fn compute_path<T>(
        origin: ChunkPosition,
        destination: ChunkPosition,
        provider: &mut impl PathProvider<T>,
        pass_through_data: T,
    ) -> Vec<ChunkPosition>;
}
