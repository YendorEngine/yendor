use crate::prelude::*;

pub struct IDAstar;

impl PathAlgorithm for IDAstar {
    fn compute_path<T, const DIMENSIONS: UVec2>(
        origin: Position<DIMENSIONS>,
        destination: Position<DIMENSIONS>,
        provider: &mut impl PathProvider<T, DIMENSIONS>,
        mut pass_through_data: T,
    ) -> Vec<Position<DIMENSIONS>> {
        let result = idastar(
            &origin,
            |&p| {
                let neighbors = provider.get_neighbors(p, &mut pass_through_data);
                let mut successors = Vec::with_capacity(neighbors.len());
                for neighbor in neighbors {
                    successors.push((neighbor, provider.cost(neighbor, &mut pass_through_data)));
                }
                successors
            },
            |&p| provider.distance(p, destination),
            |&p| p == destination,
        );

        match result {
            Some((path, _length)) => path,
            None => Vec::new(),
        }
    }
}
