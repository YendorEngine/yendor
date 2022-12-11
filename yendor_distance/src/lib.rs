#![allow(clippy::module_inception)]

mod chebyshev;
mod diagonal;
mod distance;
mod distance_algorithm;
mod manhattan;
mod pythagoras;

pub mod imports;

pub mod prelude {
    pub(crate) use crate::imports::*;
    pub use crate::{
        chebyshev::*, diagonal::*, distance::*, distance_algorithm::*, manhattan::*, pythagoras::*,
    };
}
