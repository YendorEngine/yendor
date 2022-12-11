#![allow(incomplete_features)]
#![allow(clippy::module_inception)]
// https://github.com/rust-lang/rust/issues/95174
#![feature(adt_const_params)]
// https://github.com/rust-lang/rust/issues/85077
#![feature(generic_arg_infer)]

mod grids {
    #[cfg(feature = "bitvec")]
    mod bitgrid;
    #[cfg(feature = "bitvec")]
    pub use bitgrid::*;
    mod grid_2d;
    pub use grid_2d::*;
    mod grid_3d;
    pub use grid_3d::*;
}

mod grid_iterable;
mod grid_layer;
mod grid_param;

pub mod imports;

pub mod prelude {
    pub(crate) use crate::imports::*;
    pub use crate::{grid_iterable::*, grid_layer::*, grid_param::*, grids::*};
}
