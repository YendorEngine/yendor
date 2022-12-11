use crate::prelude::*;

pub enum PathFinder {
    Astar,
    Bfs,
    Dfs,
    Dijkstra,
    DijkstraPartial,
    IDAstar,
    IDDfs,
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
            Self::Astar => AStar::compute_path(origin, destination, provider, pass_through_data),
            Self::Bfs => Bfs::compute_path(origin, destination, provider, pass_through_data),
            Self::Dfs => Dfs::compute_path(origin, destination, provider, pass_through_data),
            Self::Dijkstra => Dijkstra::compute_path(origin, destination, provider, pass_through_data),
            Self::DijkstraPartial => {
                DijkstraPartial::compute_path(origin, destination, provider, pass_through_data)
            },
            Self::IDAstar => IDAstar::compute_path(origin, destination, provider, pass_through_data),
            Self::IDDfs => IDDfs::compute_path(origin, destination, provider, pass_through_data),
        }
    }
}
