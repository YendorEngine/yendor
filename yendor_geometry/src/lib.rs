#![allow(incomplete_features)]
#![allow(clippy::module_inception)]
// https://github.com/rust-lang/rust/issues/95174
#![feature(adt_const_params)]
// https://github.com/rust-lang/rust/issues/85077
#![feature(generic_arg_infer)]

//mod axis {
//    mod axis;
//    pub use axis::*;
//}
//
//
//
//pub mod position {
//    mod local_position;
//    pub use local_position::*;
//    mod octant;
//    pub use octant::*;
//    mod position;
//    pub use position::*;
//    mod world_position;
//    pub use world_position::*;
//}

mod imports;

pub mod prelude {
    pub use bevy::{
        prelude::{IVec2, IVec3, UVec2, UVec3, Vec2, Vec3},
        utils::{HashMap, HashSet},
    };

    pub(crate) use crate::imports::*;
    pub use crate::{
        //axis::*,
        //position::*,
    };
}
