use crate::prelude::*;

/// Trait to implement 2D path algorithms.
pub struct Dijkstra;

impl PathAlgorithm for Dijkstra {
    fn compute_path<T>(
        origin: IVec2,
        destination: IVec2,
        provider: &mut impl PathProvider<T>,
        mut pass_through_data: T,
    ) -> Vec<IVec2> {
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
