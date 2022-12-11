use crate::prelude::*;

pub struct AStar;

impl PathAlgorithm for AStar {
    fn compute_path<T, const GRID_WIDTH: u32, const GRID_HEIGHT: u32>(
        origin: Position<GRID_WIDTH, GRID_HEIGHT>,
        destination: Position<GRID_WIDTH, GRID_HEIGHT>,
        provider: &mut impl PathProvider<T, GRID_WIDTH, GRID_HEIGHT>,
        pass_through_data: &T,
    ) -> Vec<Position<GRID_WIDTH, GRID_HEIGHT>> {
        let result = astar(
            &origin,
            |&p| provider.get_neighbors(p, pass_through_data),
            |&p| provider.distance(p, destination),
            |p| *p == destination,
        );

        match result {
            Some((path, _length)) => path,
            None => Vec::new(),
        }
    }
}
