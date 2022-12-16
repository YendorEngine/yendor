// Clippy
#![allow(clippy::module_inception)]
// Features
#![feature(trait_alias)]

pub mod directions;
pub mod distance;
#[cfg(feature = "fov")]
pub mod fov;
pub mod grid;
#[cfg(feature = "pathfinding")]
pub mod pathfinding;
#[cfg(feature = "random")]
pub mod random;
pub mod utility;

pub(crate) mod imports;

pub mod prelude {
    #[rustfmt::skip]
    pub use crate::imports::*;

    #[cfg(feature = "fov")]
    pub use crate::fov::*;
    #[cfg(feature = "pathfinding")]
    pub use crate::pathfinding::*;
    #[cfg(feature = "random")]
    pub use crate::random::*;
    pub use crate::{directions::*, distance::*, grid::*, utility::*};
}
