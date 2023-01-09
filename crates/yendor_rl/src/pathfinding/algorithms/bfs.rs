use crate::prelude::*;

pub struct Bfs;

impl PathAlgorithm for Bfs {
    fn compute_path<T>(
        origin: IVec2,
        destination: IVec2,
        provider: &mut impl PathProvider<T>,
        mut pass_through_data: T,
    ) -> Vec<IVec2> {
        let result = bfs(
            &origin,
            |&p| provider.get_neighbors(p, &mut pass_through_data),
            |&p| p == destination,
        );

        result.map_or_else(Vec::new, |path| path)
    }
}
