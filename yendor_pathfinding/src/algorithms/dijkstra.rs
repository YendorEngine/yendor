use crate::prelude::*;

pub struct Dijkstra;

impl PathAlgorithm for Dijkstra {
    fn compute_path<T, const DIMENSIONS: UVec2>(
        origin: Position<DIMENSIONS>,
        destination: Position<DIMENSIONS>,
        provider: &mut impl PathProvider<T, DIMENSIONS>,
        mut pass_through_data: T,
    ) -> Vec<Position<DIMENSIONS>> {
        let dijkstra_path = dijkstra(
            &origin,
            |&p| {
                let neighbors = provider.get_neighbors(p, &mut pass_through_data);
                let mut successors = Vec::with_capacity(neighbors.len());
                for neighbor in neighbors {
                    successors.push((neighbor, provider.cost(neighbor, &mut pass_through_data)));
                }
                successors
            },
            |&p| p == destination,
        );

        match dijkstra_path {
            Some((path, _length)) => path,
            None => Vec::new(),
        }
    }
}
