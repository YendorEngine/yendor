#![allow(incomplete_features)]
#![allow(clippy::module_inception)]
// https://github.com/rust-lang/rust/issues/95174
#![feature(adt_const_params)]
// https://github.com/rust-lang/rust/issues/85077
#![feature(generic_arg_infer)]

mod axis {
    mod axis;
    pub use axis::*;
}

pub mod dimensions {
    mod dimensions;
    pub use dimensions::*;
}

pub mod directions {
    mod cardinal_direction;
    pub use cardinal_direction::*;
    mod direction_flags;
    pub(crate) use direction_flags::*;
    mod direction_iter;
    pub(crate) use direction_iter::*;
    mod direction_iterator;
    pub use direction_iterator::*;
    mod direction_table;
    pub(crate) use direction_table::*;
    mod direction_type;
    pub(crate) use direction_type::*;
    mod direction;
    pub use direction::*;
    mod ordinal_direction;
    pub use ordinal_direction::*;
    mod vertical_direction;
    pub use vertical_direction::*;
    mod horizontal_direction;
    pub use horizontal_direction::*;
}

pub mod point {
    mod point_impl;
    pub use point_impl::*;
    mod point_iter;
    pub use point_iter::*;
    mod point;
    pub use point::*;
}

pub mod position {
    mod local_position;
    pub use local_position::*;
    mod octant;
    pub use octant::*;
    mod position;
    pub use position::*;
    mod world_position;
    pub use world_position::*;
}

pub mod shapes {
    mod iter {
        mod rect_iter;
        pub use rect_iter::*;
        mod line_iter;
        pub use line_iter::*;
    }
    pub(crate) use iter::*;

    mod circle;
    pub use circle::*;
    mod line;
    pub use line::*;
    mod rectangle;
    pub use rectangle::*;

    mod shape;
    pub use shape::*;
}

mod imports;

pub mod prelude {
    pub use bevy::{
        prelude::{IVec2, IVec3, UVec2, UVec3, Vec2, Vec3},
        utils::{HashMap, HashSet},
    };

    pub(crate) use crate::imports::*;
    pub use crate::{axis::*, dimensions::*, directions::*, point::*, position::*, shapes::*};
}
