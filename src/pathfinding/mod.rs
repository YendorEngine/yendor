mod algorithms {
    mod astar;
    pub use astar::*;
    mod bfs;
    pub use bfs::*;
    mod dfs;
    pub use dfs::*;
    mod dijkstra;
    pub use dijkstra::*;
    mod dijkstra_partial;
    pub use dijkstra_partial::*;
    mod id_astar;
    pub use id_astar::*;
    mod id_dfs;
    pub use id_dfs::*;
}
pub use algorithms::*;

mod path_algorithm;
pub use path_algorithm::*;
mod path_provider;
pub use path_provider::*;
mod pathfinder;
pub use pathfinder::*;
