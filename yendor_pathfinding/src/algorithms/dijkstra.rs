use crate::prelude::*;

pub struct Dijkstra;

impl PathAlgorithm for Dijkstra {
    fn compute_path<T>(
        origin: ChunkPosition,
        destination: ChunkPosition,
        provider: &mut impl PathProvider<T>,
        mut pass_through_data: T,
    ) -> Vec<ChunkPosition> {
        let dijkstra_path = dijkstra(
            &origin,
            |&p| provider.generate_successors(p, &mut pass_through_data),
            |&p| p == destination,
        );

        match dijkstra_path {
            Some((path, _length)) => path,
            None => Vec::new(),
        }
    }
}
