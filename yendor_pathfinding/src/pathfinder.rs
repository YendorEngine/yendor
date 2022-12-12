use crate::prelude::*;

pub enum PathFinder {
    Bfs,
    Dfs,
    IDDfs,
    Astar,
    IDAstar,
    Dijkstra,
    DijkstraPartial,
}

impl PathFinder {
    pub fn compute<T, const DIMENSIONS: UVec2>(
        &self,
        origin: Position<DIMENSIONS>,
        destination: Position<DIMENSIONS>,
        provider: &mut impl PathProvider<T, DIMENSIONS>,
        pass_through_data: T,
    ) -> Vec<Position<DIMENSIONS>> {
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