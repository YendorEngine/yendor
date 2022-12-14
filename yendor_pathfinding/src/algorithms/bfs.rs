use crate::prelude::*;

pub struct Bfs;

impl PathAlgorithm for Bfs {
    fn compute_path<T, const DIM: UVec2>(
        origin: Position<DIM>,
        destination: Position<DIM>,
        provider: &mut impl PathProvider<T, DIM>,
        mut pass_through_data: T,
    ) -> Vec<Position<DIM>> {
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
