#![allow(incomplete_features)]
#![allow(clippy::module_inception)]
// https://github.com/rust-lang/rust/issues/95174
#![feature(adt_const_params)]
// https://github.com/rust-lang/rust/issues/85077
#![feature(generic_arg_infer)]

mod adams {
    mod adams;
    pub use adams::*;
}

mod shadowcast {
    mod shadowcast;
    pub use shadowcast::*;
    mod quadrant;
    mod row;
}

mod fov;
mod fov_algorithm;
mod fov_provider;
mod slope;

mod imports;

pub mod prelude {
    pub use crate::{adams::*, fov::*, fov_algorithm::*, fov_provider::*, slope::*};
    pub(crate) use crate::{imports::*, shadowcast::*};
}
