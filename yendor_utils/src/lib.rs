#![allow(clippy::module_inception)]

mod canvas;
mod range;

mod imports;

pub mod prelude {
    pub(crate) use crate::imports::*;
    pub use crate::{canvas::*, range::*};
}
