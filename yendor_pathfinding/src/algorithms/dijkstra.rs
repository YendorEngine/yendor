use crate::prelude::*;

pub struct Dijkstra;

impl PathAlgorithm for Dijkstra {
    fn compute_path<T, const DIM: UVec2>(
        origin: Position<DIM>,
        destination: Position<DIM>,
        provider: &mut impl PathProvider<T>,
        mut pass_through_data: T,
    ) -> Vec<Position<DIM>> {
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
