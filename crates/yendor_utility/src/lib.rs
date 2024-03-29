//! This crate provides small utilities for Yendor-Lib
#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod canvas;
pub mod macros;
pub mod range;
pub mod ulid;

pub(crate) mod imports {
    pub use bevy::prelude::*;
}

/// The prelude.
pub mod prelude {
    pub(crate) use crate::imports::*;
    pub use crate::{impl_as, impl_as_array, impl_as_tuple};
}
