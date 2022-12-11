use crate::prelude::*;

pub enum PathFinder {
    Astar,
    Dijkstras(bool),
}

impl PathFinder {
    pub fn compute<T, const GRID_WIDTH: u32, const GRID_HEIGHT: u32>(
        &self,
        origin: Position<GRID_WIDTH, GRID_HEIGHT>,
        destination: Position<GRID_WIDTH, GRID_HEIGHT>,
        provider: &mut impl PathProvider<T, GRID_WIDTH, GRID_HEIGHT>,
        pass_through_data: T,
    ) -> Vec<Position<GRID_WIDTH, GRID_HEIGHT>> {
        match self {
            Self::Astar => {
                AStar::compute_path(origin, destination, provider, pass_through_data)
            },
            Self::Dijkstras(partial_path) => match partial_path {
                true => Dijkstras::compute_path_partial(origin, destination, provider, pass_through_data),
                false => Dijkstras::compute_path(origin, destination, provider, pass_through_data),
            },
        }
    }
}
