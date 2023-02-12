use crate::prelude::*;

/// Different pathfinding algorithms.
pub enum PathFinder {
    /// Breadth-first search algorithm.
    Bfs,
    /// Depth-first search algorithm.
    Dfs,
    /// Iterative deepening depth-first search algorithm.
    IDDfs,
    /// A* algorithm.
    Astar,
    /// Iterative deepening A* algorithm.
    IDAstar,
    /// Dijkstra algorithm that computes the full path.
    Dijkstra,
    /// Dijkstra algorithm that provides partial paths to destination.
    DijkstraPartial,
}

impl PathFinder {
    /// Computes a path from the origin to the destination using the specified algorithm.
    pub fn compute<T>(
        &self,
        origin: IVec2,
        destination: IVec2,
        provider: &mut impl PathProvider<T>,
        pass_through_data: T,
    ) -> Vec<IVec2> {
        match self {
            Self::Astar => AStar::compute_path(origin, destination, provider, pass_through_data),
            Self::Bfs => Bfs::compute_path(origin, destination, provider, pass_through_data),
            Self::Dfs => Dfs::compute_path(origin, destination, provider, pass_through_data),
            Self::Dijkstra => {
                Dijkstra::compute_path(origin, destination, provider, pass_through_data)
            }
            Self::DijkstraPartial => {
                DijkstraPartial::compute_path(origin, destination, provider, pass_through_data)
            }
            Self::IDAstar => {
                IDAstar::compute_path(origin, destination, provider, pass_through_data)
            }
            Self::IDDfs => IDDfs::compute_path(origin, destination, provider, pass_through_data),
        }
    }
}
