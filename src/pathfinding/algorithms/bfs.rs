use crate::prelude::*;

pub struct Bfs;

impl PathAlgorithm for Bfs {
    fn compute_path<T>(
        origin: UVec2,
        destination: UVec2,
        provider: &mut impl PathProvider<T>,
        mut pass_through_data: T,
    ) -> Vec<UVec2> {
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
