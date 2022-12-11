#![allow(incomplete_features)]
#![allow(clippy::module_inception)]
// https://github.com/rust-lang/rust/issues/95174
#![feature(adt_const_params)]
// https://github.com/rust-lang/rust/issues/85077
#![feature(generic_arg_infer)]
#![feature(inherent_associated_types)]

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

mod path_algorithm;
pub use path_algorithm::*;
mod path_provider;
pub use path_provider::*;
mod pathfinder;
pub use pathfinder::*;

pub mod imports;

pub mod prelude {
    pub(crate) use crate::{algorithms::*, imports::*};
    pub use crate::{path_algorithm::*, path_provider::*, pathfinder::*};
}
