//! This crate provides an assortment of opinionated utilities for roguelike development.

#![warn(clippy::nursery)]
#![warn(missing_docs)]

pub mod directions;
pub mod distance;
#[cfg(feature = "fov")]
pub mod fov;
pub mod geometry;
pub mod grid;
#[cfg(feature = "pathfinding")]
pub mod pathfinding;
#[cfg(feature = "random")]
pub mod random;

pub(crate) mod imports {
    pub use std::{
        fmt::{Debug, Display},
        ops::{
            Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, RangeBounds, Sub,
            SubAssign,
        },
        slice,
    };

    #[cfg(feature = "reflect")]
    pub use bevy::reflect::{FromReflect, Reflect};
    pub use bevy::{
        ecs::{component::Component, system::Query},
        log::warn,
        math::{IVec2, IVec3, UVec2, UVec3, Vec2, Vec3},
        prelude::{Deref, DerefMut},
        utils::{HashMap, HashSet},
    };
    #[cfg(feature = "pathfinding")]
    pub use pathfinding::prelude::{
        astar, bfs, build_path, dfs, dijkstra, dijkstra_partial, idastar, iddfs,
    };
    #[cfg(feature = "random")]
    pub use rand::{
        distributions::{Distribution, Standard},
        prelude::*,
        Rng as RandRng, SeedableRng,
    };
    #[cfg(feature = "serialize")]
    pub use serde::{Deserialize, Serialize};
}

/// A prelude for crates using `yendor-rl`.
pub mod prelude {
    #[rustfmt::skip]
    pub use crate::imports::*;

    #[cfg(feature = "fov")]
    pub use crate::fov::*;
    #[cfg(feature = "pathfinding")]
    pub use crate::pathfinding::*;
    #[cfg(feature = "random")]
    pub use crate::random::*;
    pub use crate::{directions::*, distance::*, geometry::*, grid::*};
}
