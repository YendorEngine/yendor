use crate::prelude::*;

pub enum PathFinder<T> {
    Astar(T),
    Dijkstras(bool, T),
}

impl<T> PathFinder<T> {
    pub fn compute<const GRID_WIDTH: u32, const GRID_HEIGHT: u32>(
        &self,
        origin: Position<GRID_WIDTH, GRID_HEIGHT>,
        destination: Position<GRID_WIDTH, GRID_HEIGHT>,
        partial_path_on_failure: bool,
        provider: &mut impl PathProvider<T, GRID_WIDTH, GRID_HEIGHT>,
    ) -> Vec<Position<GRID_WIDTH, GRID_HEIGHT>> {
        match self {
            Self::Astar(&pass_through_data) => AStar::compute_path(
                origin,
                destination,
                provider,
                pass_through_data,
            ),
            Self::Dijkstras(&partial_path, &pass_through_data) => {
                match partial_path {
                    true => Dijkstras::compute_path_partial(
                        origin,
                        destination,
                        provider,
                        pass_through_data
                    ),
                    false => Dijkstras::compute_path(
                        origin,
                        destination,
                        provider,
                        pass_through_data
                    ),
                }
            }
        }
    }
}
