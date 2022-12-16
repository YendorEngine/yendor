use crate::prelude::*;

pub struct IDAstar;

impl PathAlgorithm for IDAstar {
    fn compute_path<T>(
        origin: ChunkPosition,
        destination: ChunkPosition,
        provider: &mut impl PathProvider<T>,
        mut pass_through_data: T,
    ) -> Vec<ChunkPosition> {
        let result = idastar(
            &origin,
            |&p| provider.generate_successors(p, &mut pass_through_data),
            |&p| provider.distance(p, destination),
            |&p| p == destination,
        );

        match result {
            Some((path, _length)) => path,
            None => Vec::new(),
        }
    }
}
