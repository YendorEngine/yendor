use crate::prelude::*;

pub struct Bfs;

impl PathAlgorithm for Bfs {
    fn compute_path<T, const DIMENSIONS: UVec2>(
        origin: Position<DIMENSIONS>,
        destination: Position<DIMENSIONS>,
        provider: &mut impl PathProvider<T, DIMENSIONS>,
        mut pass_through_data: T,
    ) -> Vec<Position<DIMENSIONS>> {
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