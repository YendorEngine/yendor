use crate::prelude::*;

pub struct Bfs;

impl PathAlgorithm for Bfs {
    fn compute_path<T>(
        origin: ChunkPosition,
        destination: ChunkPosition,
        provider: &mut impl PathProvider<T>,
        mut pass_through_data: T,
    ) -> Vec<ChunkPosition> {
        let result = bfs(
            &origin,
            |&p| provider.get_neighbors(p, &mut pass_through_data),
            |&p| p == destination,
        );

        match result {
            Some(path) => path,
            None => Vec::new(),
        }
    }
}
