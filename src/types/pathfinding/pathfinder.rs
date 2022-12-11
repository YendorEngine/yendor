use super::PathAlgorithm;
use crate::prelude::*;

pub enum PathFinder<T> {
    Astar(T),
    Dijkstras(T),
}

impl<T> PathFinder<T> {
    pub fn compute<const GRID_WIDTH: u32, const GRID_HEIGHT: u32>(
        &self,
        origin: Position<GRID_WIDTH, GRID_HEIGHT>,
        destination: Position<GRID_WIDTH, GRID_HEIGHT>,
        partial_path_on_failure: bool,
        provider: &mut impl PathProvider,
    ) -> Option<Vec<Position>> {
        match self {
            Self::Astar => AStar::compute_path(
                origin,
                destination,
                partial_path_on_failure,
                provider,
            ),
            Self::Dijkstras => Dijkstras::compute_path(
                origin,
                destination,
                partial_path_on_failure,
                provider,
            ),
        }
    }
}
