use crate::prelude::*;

pub struct AStar;

impl PathAlgorithm for AStar {
    fn compute_path<T, const DIMENSIONS: UVec2>(
        origin: Position<DIMENSIONS>,
        destination: Position<DIMENSIONS>,
        provider: &mut impl PathProvider<T, DIMENSIONS>,
        mut pass_through_data: T,
    ) -> Vec<Position<DIMENSIONS>> {
        let result = astar(
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